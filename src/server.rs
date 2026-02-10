use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};

const HTML: &str = include_str!("../editor/index.html");

#[derive(Clone)]
struct AppState {
    file_path: PathBuf,
    content: Arc<RwLock<String>>,
    broadcast_tx: broadcast::Sender<String>,
}

pub async fn serve(file: PathBuf, port: u16) {
    // Read initial content
    let content = match std::fs::read_to_string(&file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: Failed to read {}: {}", file.display(), e);
            std::process::exit(1);
        }
    };

    // Create broadcast channel for WebSocket updates
    let (broadcast_tx, _) = broadcast::channel::<String>(100);

    let state = AppState {
        file_path: file.clone(),
        content: Arc::new(RwLock::new(content)),
        broadcast_tx: broadcast_tx.clone(),
    };

    // Start file watcher
    let watch_state = state.clone();
    let watch_file = file.clone();
    tokio::spawn(async move {
        if let Err(e) = watch_file_changes(watch_file, watch_state).await {
            eprintln!("File watcher error: {}", e);
        }
    });

    // Build router
    let app = Router::new()
        .route("/", get(serve_index))
        .route("/api/blueprint", get(get_blueprint).put(put_blueprint))
        .route("/api/validate", get(get_validation))
        .route("/ws", get(websocket_handler))
        .with_state(state);

    // Bind server
    let addr = format!("127.0.0.1:{}", port);
    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("Error: Failed to bind to {}: {}", addr, e);
            std::process::exit(1);
        }
    };

    println!("✓ Server started at http://{}", addr);
    println!("\nOpening browser...");

    // Open browser
    let url = format!("http://{}", addr);
    if let Err(e) = open::that(&url) {
        eprintln!("Warning: Failed to open browser: {}", e);
        println!("Please open manually: {}", url);
    }

    // Run server
    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("Error: Server failed: {}", e);
        std::process::exit(1);
    }
}

async fn serve_index() -> Html<&'static str> {
    Html(HTML)
}

async fn get_blueprint(State(state): State<AppState>) -> impl IntoResponse {
    let content = state.content.read().await;
    content.clone()
}

async fn put_blueprint(
    State(state): State<AppState>,
    body: String,
) -> Result<impl IntoResponse, StatusCode> {
    // Write to file
    if let Err(e) = std::fs::write(&state.file_path, &body) {
        eprintln!("Error writing file: {}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    // Update in-memory content
    let mut content = state.content.write().await;
    *content = body.clone();

    Ok(StatusCode::OK)
}

async fn get_validation(State(state): State<AppState>) -> impl IntoResponse {
    let content = state.content.read().await;
    let bp = crate::parser::parse(&content, state.file_path.clone());
    let results = crate::validator::validate(&bp);

    axum::Json(results)
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    ws.on_upgrade(|socket| websocket_connection(socket, state))
}

async fn websocket_connection(mut socket: WebSocket, state: AppState) {
    use futures::stream::StreamExt;
    use futures::sink::SinkExt;

    let mut broadcast_rx = state.broadcast_tx.subscribe();

    // Send initial content
    let content = state.content.read().await.clone();
    if socket.send(Message::Text(content.into())).await.is_err() {
        return;
    }

    loop {
        tokio::select! {
            // Receive from client
            Some(Ok(msg)) = socket.next() => {
                if let Message::Text(_text) = msg {
                    // Client sent updated content - ignore for now
                    // (we rely on PUT /api/blueprint for saves)
                }
            }

            // Receive from broadcast (file changes)
            Ok(content) = broadcast_rx.recv() => {
                if socket.send(Message::Text(content.into())).await.is_err() {
                    break;
                }
            }

            else => break,
        }
    }
}

async fn watch_file_changes(
    file_path: PathBuf,
    state: AppState,
) -> notify::Result<()> {
    let (tx, mut rx) = tokio::sync::mpsc::channel(100);

    let mut watcher = RecommendedWatcher::new(
        move |res: Result<Event, notify::Error>| {
            if let Ok(event) = res {
                let _ = tx.blocking_send(event);
            }
        },
        Config::default(),
    )?;

    watcher.watch(&file_path, RecursiveMode::NonRecursive)?;

    while let Some(event) = rx.recv().await {
        use notify::EventKind;

        match event.kind {
            EventKind::Modify(_) | EventKind::Create(_) => {
                // File was modified externally
                if let Ok(content) = std::fs::read_to_string(&file_path) {
                    let mut state_content = state.content.write().await;
                    *state_content = content.clone();
                    drop(state_content);

                    // Broadcast to all WebSocket clients
                    let _ = state.broadcast_tx.send(content);
                }
            }
            _ => {}
        }
    }

    Ok(())
}

use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod blueprint;
mod parser;
mod validator;
mod prompt;
mod tasks;
mod template;
mod server;

#[derive(Parser)]
#[command(name = "nira", about = "Your personal architectural control center")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new blueprint.md in the current directory
    Init {
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        force: bool,
    },
    /// Open blueprint in browser editor with live preview
    Serve {
        #[arg(default_value = "blueprint.md")]
        file: PathBuf,
        #[arg(long, default_value = "3141")]
        port: u16,
    },
    /// Check blueprint completeness
    Validate {
        #[arg(default_value = "blueprint.md")]
        file: PathBuf,
    },
    /// Generate AI-ready prompt from blueprint
    Prompt {
        #[arg(default_value = "blueprint.md")]
        file: PathBuf,
        #[arg(long)]
        task: Option<usize>,
    },
    /// Manage tasks in Layer 4
    Task {
        #[command(subcommand)]
        action: TaskAction,
    },
}

#[derive(Subcommand)]
enum TaskAction {
    /// List all tasks
    List {
        #[arg(default_value = "blueprint.md")]
        file: PathBuf,
    },
    /// Add a task to NEXT UP
    Add {
        description: String,
        #[arg(default_value = "blueprint.md")]
        file: PathBuf,
    },
    /// Move a task to DONE
    Done {
        task_num: usize,
        #[arg(default_value = "blueprint.md")]
        file: PathBuf,
    },
    /// Move a task to IN PROGRESS
    Start {
        task_num: usize,
        #[arg(default_value = "blueprint.md")]
        file: PathBuf,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name, force } => {
            let file_path = std::path::Path::new("blueprint.md");

            // Check if file already exists
            if file_path.exists() && !force {
                eprintln!("Error: blueprint.md already exists in this directory.");
                eprintln!("Use --force to overwrite it.");
                std::process::exit(1);
            }

            // Get project name
            let project_name = name.unwrap_or_else(|| {
                // Try to get from current directory name
                std::env::current_dir()
                    .ok()
                    .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
                    .unwrap_or_else(|| "MyProject".to_string())
            });

            // Get current date
            let date = chrono::Local::now().format("%Y-%m-%d").to_string();

            // Substitute placeholders in template
            let content = template::TEMPLATE
                .replace("{PROJECT_NAME}", &project_name)
                .replace("{DATE}", &date);

            // Write file
            match std::fs::write(file_path, content) {
                Ok(_) => {
                    println!("✓ Created blueprint.md");
                    println!("\nNext steps:");
                    println!("  1. Edit blueprint.md and fill in the layers");
                    println!("  2. Run 'nira validate' to check your progress");
                    println!("  3. Run 'nira prompt' to generate AI instructions");
                }
                Err(e) => {
                    eprintln!("Error: Failed to write blueprint.md: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Serve { file, port } => {
            server::serve(file, port).await;
        }
        Commands::Validate { file } => {
            let bp = load_blueprint(&file);
            let results = validator::validate(&bp);

            println!("Blueprint Validation: {}\n", file.display());

            for result in &results {
                let symbol = match result.status {
                    blueprint::ValidationStatus::Ok => "✓",
                    blueprint::ValidationStatus::Warning => "⚠",
                    blueprint::ValidationStatus::Missing => "✗",
                };

                println!("{} Layer {}: {} - {}",
                    symbol, result.layer, result.layer_name, result.message);
            }

            // Exit with error code if there are Missing items
            if results.iter().any(|r| matches!(r.status, blueprint::ValidationStatus::Missing)) {
                std::process::exit(1);
            }
        }
        Commands::Prompt { file, task } => {
            let bp = load_blueprint(&file);

            let output = if let Some(task_num) = task {
                match prompt::generate_for_task(&bp, task_num) {
                    Some(prompt) => prompt,
                    None => {
                        eprintln!("Error: Task #{} not found.", task_num);
                        eprintln!("Run 'nira task list' to see available tasks.");
                        std::process::exit(1);
                    }
                }
            } else {
                prompt::generate(&bp)
            };

            println!("{}", output);
        }
        Commands::Task { action } => match action {
            TaskAction::List { file } => {
                let bp = load_blueprint(&file);
                tasks::list_tasks(&bp);
            }
            TaskAction::Add { description, file } => {
                match tasks::add_task(&file, &description) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            TaskAction::Done { task_num, file } => {
                match tasks::move_task(&file, task_num, blueprint::TaskStatus::Done) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        std::process::exit(1);
                    }
                }
            }
            TaskAction::Start { task_num, file } => {
                match tasks::move_task(&file, task_num, blueprint::TaskStatus::InProgress) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        std::process::exit(1);
                    }
                }
            }
        },
    }
}

/// Helper: load and parse a blueprint file, or exit with a nice error
fn load_blueprint(path: &PathBuf) -> blueprint::Blueprint {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Error: Could not read {}.", path.display());
            eprintln!("Run `nira init` to create one.");
            std::process::exit(1);
        }
    };
    parser::parse(&content, path.clone())
}

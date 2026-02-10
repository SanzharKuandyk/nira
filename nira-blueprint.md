# Blueprint: nira

> **Started:** 2026-02-10
> **Last updated:** 2026-02-10
> **Status:** Planning

---

## Layer 1: Intent Map

**PROJECT:** nira

**ONE-LINE:** Your personal architectural control center — keeps you in the driver's seat when using AI, so you always understand how your code connects and works.

**ACTORS:**
- You (the developer/architect) using the CLI
- Browser for editing/viewing blueprints
- AI agents that receive generated prompts

**CORE FLOWS:**
1. You run `nira init` → creates `blueprint.md` from template in current dir
2. You run `nira serve` → opens browser with split-pane editor (markdown left, preview right)
3. You edit in browser → saves to disk via WebSocket → preview updates live
4. You run `nira validate` → checks all 4 layers for completeness → reports missing/weak sections
5. You run `nira prompt` → outputs copy-paste-ready AI prompt wrapping the blueprint
6. You run `nira prompt --task 3` → outputs focused prompt for task #3 only
7. You run `nira task add "do the thing"` → appends to NEXT UP
8. You run `nira task start 3` / `nira task done 3` → moves tasks between sections
9. You run `nira task list` → shows numbered tasks with status and metadata

**HARD PARTS:**
- Browser editor needs split-pane with markdown preview, embedded in binary (no external files)
- WebSocket live sync between browser and disk (bidirectional — external edits too)
- Markdown parsing is structural (find sections by heading), not full AST — keep it simple
- Must embed all HTML/CSS/JS via `include_str!` for single-binary deployment

**NON-GOALS:**
- Not a general markdown editor
- No cloud sync, no collaboration, no database
- No custom themes (keep it functional)
- Not for teams — this is YOUR personal architecture control center

---

## Layer 2: Interface Contracts

### Data Shapes (Type A)

#### Blueprint

| Field | Type | Meaning |
|-------|------|---------|
| raw | String | Full markdown content (source of truth) |
| path | PathBuf | File location on disk |
| has_intent | bool | Layer 1 has real content |
| has_contracts | bool | Layer 2 has real content |
| has_skeleton | bool | Layer 3 has real content |
| tasks | TaskQueue | Parsed Layer 4 |
| project_name | Option\<String\> | Extracted from `# Blueprint: Name` heading |

- **Used by:** Validator, PromptGenerator, TaskManager, Server
- **Produced by:** Parser
- **Rules:**
  - `raw` is always the source of truth — parsed fields are derived
  - Re-parse from raw whenever file changes

#### TaskItem

| Field | Type | Meaning |
|-------|------|---------|
| text | String | Task description (stripped of bold markers) |
| status | TaskStatus (Done/InProgress/NextUp/Icebox) | Current state |
| context | Option\<String\> | Context note (for in-progress tasks) |
| files | Option\<String\> | Files involved |
| approach | Option\<String\> | Approach description |
| line_number | usize | 1-indexed line in the markdown file |

- **Used by:** TaskManager, CLI display, PromptGenerator
- **Produced by:** Parser

#### TaskQueue

| Field | Type | Meaning |
|-------|------|---------|
| done | Vec\<TaskItem\> | Completed tasks |
| in_progress | Vec\<TaskItem\> | Currently being worked on |
| next_up | Vec\<TaskItem\> | Planned next |
| icebox | Vec\<TaskItem\> | Ideas for later |

- **Key method:** `all_active_numbered() -> Vec<(usize, &TaskItem)>` — returns in_progress + next_up + icebox with sequential numbering starting at 1 (done tasks are unnumbered)

#### ValidationResult

| Field | Type | Meaning |
|-------|------|---------|
| layer | u8 | 1-4 |
| layer_name | String | "Intent Map", "Interface Contracts", etc. |
| status | Ok / Warning / Missing | Completeness level |
| message | String | Human-readable explanation |

---

### Capabilities (Type B)

#### Parser

**Purpose:** Reads blueprint markdown and extracts structured data

| Method | Signature | What it does |
|--------|-----------|-------------|
| parse | (content: &str, path: PathBuf) -> Blueprint | Parse raw markdown into structured Blueprint |
| find_section | (content: &str, heading: &str) -> Option<(usize, usize)> | Find byte range of a section by heading text (case-insensitive substring match). Returns (start_of_content, end_of_content) excluding the heading line. |

- **Implementations:** Single implementation using regex + line iteration
- **Rules:**
  - Never modifies markdown — read only
  - Handles missing sections gracefully (returns None / false)
  - `find_section` finds end by looking for next heading of same or higher level
  - Task parsing: match `- [ ]` / `- [x]` lines, then look ahead for indented metadata (Context, Files, Approach, Depends on, Blocked)
  - `section_has_content` helper: returns false if section is only placeholders like `[name]`, `[what it does]`, HTML comments, or empty

#### Validator

**Purpose:** Checks a Blueprint for completeness

| Method | Signature | What it does |
|--------|-----------|-------------|
| validate | (bp: &Blueprint) -> Vec\<ValidationResult\> | Check all 4 layers + task quality |

- **Rules:**
  - Layers 1-3: check `has_intent`, `has_contracts`, `has_skeleton`
  - Layer 4: Ok if active tasks exist, Warning if only done/icebox, Missing if empty
  - Extra warnings: in-progress tasks missing Context or Files, next-up tasks missing Approach

#### PromptGenerator

**Purpose:** Converts a Blueprint into paste-ready AI prompts

| Method | Signature | What it does |
|--------|-----------|-------------|
| generate | (bp: &Blueprint) -> String | Full prompt: blueprint in XML tags + instruction rules |
| generate_for_task | (bp: &Blueprint, task_num: usize) -> Option\<String\> | Focused prompt for one task |

- **Rules:**
  - Wraps blueprint in `<blueprint>` tags, rules in `<rules>` tags
  - Task prompt includes `<current_task>` block with all metadata
  - Rules instruct AI to: follow contracts, use file skeleton, ask before changing interfaces, report what changed

#### TaskManager

**Purpose:** Manipulates tasks by editing the markdown file directly

| Method | Signature | What it does |
|--------|-----------|-------------|
| list_tasks | (bp: &Blueprint) | Pretty-print tasks with colors and numbers |
| add_task | (path: &Path, description: &str) -> Result | Append to NEXT UP with empty metadata fields |
| move_task | (path: &Path, task_num: usize, target: TaskStatus) -> Result | Remove from current section, insert into target section |

- **Rules:**
  - `add_task` inserts at end of NEXT UP section with template metadata
  - `move_task` preserves metadata where applicable, adapts format to target section
  - Moving to Done: `- [x] text` (no metadata)
  - Moving to InProgress: adds Context/Blocked/Files fields
  - Moving to NextUp: adds Depends on/Files/Approach fields
  - Moving to Icebox: `- [ ] text` (no metadata)

---

### Boundaries (Type C)

#### CLI

**Purpose:** Command-line argument parsing and dispatch

| Command | Args | What it does |
|---------|------|-------------|
| `init` | `[--name NAME]` | Create blueprint.md from embedded template. Fills in name and date. Refuses if file exists (use --force). |
| `serve` | `[--port PORT] [FILE]` | Start HTTP server, open browser. Default port 3141, default file ./blueprint.md |
| `validate` | `[FILE]` | Print validation results with colored status icons |
| `prompt` | `[FILE] [--task N]` | Print AI prompt to stdout (pipeable). --task N focuses on specific task. |
| `task list` | `[FILE]` | Show numbered tasks |
| `task add` | `"description" [FILE]` | Add task to NEXT UP |
| `task done` | `N [FILE]` | Move task #N to DONE |
| `task start` | `N [FILE]` | Move task #N to IN PROGRESS |

- **Default FILE:** `./blueprint.md` for all commands
- **Error handling:** Clear messages like "No blueprint.md found. Run `nira init` first."

#### WebServer

**Purpose:** Serves editor UI and handles live sync via WebSocket

| Endpoint | Method | What it does |
|----------|--------|-------------|
| `/` | GET | Serve the editor HTML (embedded via `include_str!`) |
| `/api/blueprint` | GET | Return current markdown as plain text |
| `/api/blueprint` | PUT | Save request body to disk |
| `/api/validate` | GET | Return `Vec<ValidationResult>` as JSON |
| `/ws` | WebSocket | Bidirectional: server pushes file content on disk change; client sends content on edit |

- **Error handling:** File write failures → 500 with message
- **Rules:**
  - All assets embedded in binary via `include_str!`
  - Use `notify` crate to watch file for external changes → push to all connected WebSocket clients
  - Debounce file watch events (100ms) to avoid rapid-fire updates
  - On WS message from client: write to disk, suppress the resulting file-watch event to avoid echo

---

### Connection Diagram

```
  CLI (clap)
    │
    ├── init ──────────→ [Template] ──write──→ blueprint.md
    │
    ├── validate ──→ [Parser] ──→ [Validator] ──→ stdout (colored)
    │
    ├── prompt ────→ [Parser] ──→ [PromptGenerator] ──→ stdout
    │
    ├── task ──────→ [Parser] ──→ [TaskManager] ──write──→ blueprint.md
    │
    └── serve ─────→ [WebServer]
                        │
                ┌───────┼────────┐
                │       │        │
            GET/PUT   WebSocket  FileWatcher
            (axum)    (axum-ws)  (notify crate)
                │       │        │
                ▼       ▼        │
            [Editor   [Live      │
             HTML]    Sync] ◄────┘
                        │
                        ▼
                   [Parser] → [Validator] (validation results shown in editor)
```

---

## Layer 3: File Skeleton

```
nira/
├── Cargo.toml                ← deps: clap, axum, tokio, notify, serde, serde_json, regex, open
├── src/
│   ├── main.rs               ← ENTRY: clap CLI definition + command dispatch
│   ├── blueprint.rs          ← [Data: Blueprint, TaskItem, TaskQueue, ValidationResult, enums]
│   ├── parser.rs             ← [Capability: Parser] markdown → Blueprint (find_section, parse_tasks, section_has_content)
│   ├── validator.rs          ← [Capability: Validator] Blueprint → Vec<ValidationResult>
│   ├── prompt.rs             ← [Capability: PromptGenerator] Blueprint → AI prompt string
│   ├── tasks.rs              ← [Capability: TaskManager] list/add/move tasks via markdown editing
│   ├── template.rs           ← [Data: TEMPLATE const] the embedded blueprint template string
│   └── server.rs             ← [Boundary: WebServer] axum HTTP + WebSocket + file watcher
├── editor/
│   └── index.html            ← embedded browser UI: split-pane editor + preview + validation
└── README.md
```

### Dependency Choices

| Crate | Purpose | Why this one |
|-------|---------|-------------|
| clap (derive) | CLI parsing | Standard, derive macros = clean code |
| axum + axum-extra (ws) | HTTP + WebSocket | Lightweight, tokio-native |
| tokio (full) | Async runtime | Required by axum |
| notify | File watching | Cross-platform (inotify/kqueue/ReadDirectoryChanges) |
| serde + serde_json | JSON serialization | For validation API endpoint |
| regex | Section/task parsing | Overkill but reliable |
| open | Open browser | `open::that(url)` cross-platform |

---

## Layer 4: Task Queue

### DONE ✓

- [x] Scaffold Cargo project and define CLI with clap
- [x] Define core data types in blueprint.rs
- [x] Implement parser (find_section + parse + task extraction)
- [x] Implement validator and wire up `nira validate` command
- [x] Implement `nira init` command
- [x] Implement prompt generator (`nira prompt`)
- [x] Implement task manager (list/add/move/done/start)

### IN PROGRESS →

### NEXT UP

- [ ] **Build the browser editor HTML**
  - **Depends on:** nothing (parallel track)
  - **Files:** editor/index.html
  - **Approach:** Single HTML file. Split pane: left = textarea/contenteditable, right = rendered preview. Use a lightweight markdown-to-HTML renderer in JS (can inline a small one, or use marked.js from CDN — but for single-binary purity, inline a minimal renderer). Validation warnings shown as a top bar or sidebar. WebSocket client for live sync.

- [ ] **Implement web server with live sync**
  - **Depends on:** editor HTML, parser, validator
  - **Files:** src/server.rs
  - **Approach:** axum router: GET / serves `include_str!("../editor/index.html")`. GET/PUT /api/blueprint reads/writes file. GET /api/validate returns JSON. WebSocket: on connect send current content. Use notify to watch file, broadcast changes to all WS clients. On WS message from client: write to disk, set a flag to suppress the echo from file watcher.

### ICEBOX

- [ ] Export blueprint as PDF
- [ ] `nira diff` — show what changed since last git commit
- [ ] `nira watch` — auto-validate on save, show desktop notification
- [ ] VS Code extension for inline blueprint editing
- [ ] `nira prompt --clipboard` — copy to clipboard instead of stdout

---

## Starter Code

### Cargo.toml

```toml
[package]
name = "nira"
version = "0.1.0"
edition = "2021"
description = "Your personal architectural control center - stay in control when using AI"

[dependencies]
clap = { version = "4", features = ["derive"] }
axum = { version = "0.7", features = ["ws"] }
tokio = { version = "1", features = ["full"] }
notify = "6"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
regex = "1"
open = "5"

[profile.release]
strip = true
lto = true
codegen-units = 1
```

### src/main.rs (starter)

```rust
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

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name, force } => {
            todo!("init command")
        }
        Commands::Serve { file, port } => {
            todo!("serve command")
        }
        Commands::Validate { file } => {
            todo!("validate command")
        }
        Commands::Prompt { file, task } => {
            todo!("prompt command")
        }
        Commands::Task { action } => match action {
            TaskAction::List { file } => todo!("task list"),
            TaskAction::Add { description, file } => todo!("task add"),
            TaskAction::Done { task_num, file } => todo!("task done"),
            TaskAction::Start { task_num, file } => todo!("task start"),
        },
    }
}

/// Helper: load and parse a blueprint file, or exit with a nice error
fn load_blueprint(path: &PathBuf) -> blueprint::Blueprint {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Error: Could not read {}.", path.display());
            eprintln!("Run `bprint init` to create one.");
            std::process::exit(1);
        }
    };
    parser::parse(&content, path.clone())
}
```

---

## AI Agent Instructions

Paste this into your Claude Code session:

```
You are implementing `nira` - a personal tool to help the developer maintain architectural control when using AI. Read the blueprint above carefully.

Follow these rules:
1. Follow the interface contracts exactly — types, method signatures, rules.
2. Place files according to the File Skeleton.
3. If you need a new type, tell me — I'll add it to Layer 2 first.
4. If you need to change an interface, STOP and explain why.
5. Work through the Task Queue in order. One task at a time.
6. After each task, tell me what files you changed and what to update in the blueprint.
```
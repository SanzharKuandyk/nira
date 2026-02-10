# nira

Your personal architectural control center. Stay in control when using AI to write code.

## The Problem

When you use AI to implement code, you lose the mental map. You look at the result and can't trace how pieces connect. If the AI hits a limit or goes off-track, you're stuck.

## The Solution

**nira** implements the Blueprint Pattern: before ANY code is written, you create a Blueprint document with 4 layers. The AI implements from the Blueprint. You own the architecture.

The 4 layers:
1. **Intent Map** - What does this thing do and why?
2. **Interface Contracts** - What talks to what, and what do they promise?
3. **File Skeleton** - Where does each piece live on disk?
4. **Task Queue** - What to build next?

## Installation

```bash
cargo build --release
cp target/release/nira ~/.local/bin/  # or anywhere in your PATH
```

## Quick Start

```bash
# Create a new blueprint
nira init --name "MyProject"

# Edit it in your favorite editor, or use the web UI
nira serve blueprint.md

# Check if it's complete
nira validate blueprint.md

# Generate AI instructions
nira prompt blueprint.md > prompt.txt
# Copy prompt.txt and paste it to Claude/ChatGPT

# Manage your task queue
nira task list blueprint.md
nira task add "Implement feature X" blueprint.md
nira task start 1 blueprint.md
nira task done 1 blueprint.md
```

## Commands

- `nira init` - Create a new blueprint from template
- `nira serve` - Open web editor with live preview and auto-save
- `nira validate` - Check blueprint completeness
- `nira prompt` - Generate AI-ready instructions (full or for specific task)
- `nira task list` - Show all tasks with numbers
- `nira task add` - Add a new task
- `nira task start N` - Move task N to IN PROGRESS
- `nira task done N` - Mark task N as complete

## The Web Editor

Run `nira serve blueprint.md` to get:

- Split-pane interface (editor left, preview right)
- Live markdown rendering
- Auto-save to disk (1 second after you stop typing)
- Live sync (external file changes appear instantly)
- Validation warnings at the top
- Dark theme

The server runs on http://127.0.0.1:3141 by default. Use `--port` to change it.

## Why This Works

Traditional problem: AI writes code, you review it, but you've lost the mental model of how things connect.

Blueprint Pattern solution:
1. YOU design the architecture (Layers 1-3)
2. AI implements the details (you give it tasks from Layer 4 via `nira prompt --task N`)
3. You always know how pieces connect (interface contracts)
4. You never lose your mental map (file skeleton shows where everything lives)

You're not a fraud for using AI. You're an architect who delegates implementation.

## Example Workflow

```bash
# Start a new project
nira init --name "logwatch"

# Fill in the blueprint (use nira serve for live preview)
nira serve blueprint.md

# Check your work
nira validate blueprint.md

# Generate prompt for the AI
nira prompt blueprint.md --task 1 > task1.txt

# Give task1.txt to Claude/ChatGPT
# AI implements it following your architecture

# Mark it done when AI finishes
nira task done 1 blueprint.md

# Repeat for next task
nira prompt blueprint.md --task 2 > task2.txt
```

## What Makes It Different

Other tools help you organize code AFTER it's written. nira helps you design BEFORE code exists.

The blueprint is a living document. Edit it in the web UI while AI works in another window. Your architecture evolves, but you always own it.

## License

MIT

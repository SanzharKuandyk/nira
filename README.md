# nira

Your personal architectural control center. Stay in control when using AI to write code.

> **Author’s note:**  
> I am just testing this. This tool stems from issues I got with AI agents/models when I build something.
> I can ask a model to suggest what to do next, and it can happen spontaneously, or I might concentrate on issues too much.
>
> Then I end up losing track of what implementation decisions we made—even if the AI agent writes files like `CLAUDE.md`
> or planning docs like `progress.md` / `findings.md`, because nobody actually reads them.
>
> So I decided to write an architecture plan myself and try to follow it, marking what we did or didn’t do directly.
> Using diagrams and stuff you usually learn in a Software Architecture course could work too,
> but I want **one file** that contains everything and can be run per project via some tool.

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

# Copy both the binary and templates folder
cp target/release/nira ~/.local/bin/
cp -r templates ~/.local/bin/

# Or on Windows
copy target\release\nira.exe %USERPROFILE%\bin\
xcopy /E /I templates %USERPROFILE%\bin\templates
```

**Important:** The `templates/` folder must be in the same directory as `nira.exe` for template loading to work.

## Quick Start

```bash
# Create a new blueprint (uses default template)
nira init --name "MyProject"

# Or choose a different approach
nira init --template minimal
nira init --template constraints-first
nira init --template interface-driven

# List all templates with descriptions
nira init --list-templates

# Edit it in your favorite editor, or use the web UI
nira serve

# Or specify a different file
nira serve my-architecture.md

# Check if it's complete (defaults to niraprint.md)
nira validate

# Generate AI instructions
nira prompt > prompt.txt
# Copy prompt.txt and paste it to Claude/ChatGPT

# Manage your task queue
nira task list
nira task add "Implement feature X"
nira task start 1
nira task done 1
```

## Templates

Templates are loaded dynamically from the `templates/` folder. nira comes with 5 built-in templates:

### Standard Approaches

- **default** - Full 4-layer template with detailed instructions and examples
- **minimal** - Compact 4-layer version with essential sections only
- **quick** - Ultra-fast template for rapid prototyping

### Alternative Approaches

- **constraints-first** - Start with boundaries and limits, build around constraints
  - Sections: Boundaries & Limits → Interface Contracts → Safe Paths → Risk Register → Tasks → Validation
  - Use when: Working with strict technical constraints, performance requirements, or external API limits
  - Philosophy: Define what you CAN'T do first, then design within those guardrails

- **interface-driven** - Start with contracts and APIs, implementation follows
  - Sections: Public API → Core Data Types → Internal Contracts → Implementation Modules → Integration Points → Testing
  - Use when: Building libraries, SDKs, or when API stability is critical
  - Philosophy: Contracts are the source of truth, implementation is just details

Use `--template` to choose:

```bash
nira init --template minimal
nira init --template constraints-first
nira init --template interface-driven
nira init --list-templates  # Shows all available templates
```

### Adding Custom Templates

Templates are just markdown files in the `templates/` folder. To add your own:

1. Create `templates/my-template.md`
2. Add a description comment at the top:
   ```markdown
   <!-- Description: Your template description here -->
   # Blueprint: {PROJECT_NAME}
   ```
3. Use `{PROJECT_NAME}` and `{DATE}` as placeholders
4. Done! It will automatically appear in `nira init --list-templates`

No code changes or recompilation needed. Just drop a `.md` file in `templates/` and it's ready to use.

## Commands

- `nira init [--template NAME]` - Create a new niraprint.md from template
- `nira serve [FILE]` - Open web editor with live preview and auto-save (defaults to niraprint.md)
- `nira validate [FILE]` - Check blueprint completeness (defaults to niraprint.md)
- `nira prompt [FILE]` - Generate AI-ready instructions (full or for specific task)
- `nira task list [FILE]` - Show all tasks with numbers
- `nira task add "text" [FILE]` - Add a new task
- `nira task start N [FILE]` - Move task N to IN PROGRESS
- `nira task done N [FILE]` - Mark task N as complete

All commands default to `niraprint.md` but you can specify any file.

## The Web Editor

Run `nira serve` to get:

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
nira serve

# Check your work
nira validate

# Generate prompt for the AI
nira prompt --task 1 > task1.txt

# Give task1.txt to Claude/ChatGPT
# AI implements it following your architecture

# Mark it done when AI finishes
nira task done 1

# Repeat for next task
nira prompt --task 2 > task2.txt
```

## What Makes It Different

Other tools help you organize code AFTER it's written. nira helps you design BEFORE code exists.

The blueprint is a living document. Edit it in the web UI while AI works in another window. Your architecture evolves, but you always own it.

## License

MIT

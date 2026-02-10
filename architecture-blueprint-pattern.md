# The Blueprint Pattern
### An Architectural Planning System for Humans Who Use AI Agents

---

> **The Problem:** You use AI to implement code. Then you look at the result and can't trace how pieces connect. You lose the mental map. If the AI hits a limit or goes off-track, you're stuck.
>
> **The Solution:** Before ANY code is written, you create a Blueprint — a living document that is your brain's external memory. The AI implements *from* the Blueprint. You *own* the Blueprint.

---

## How This Pattern Works

The Blueprint has **4 layers**, from zoomed-out to zoomed-in. You fill them in order. Each layer answers one question:

| Layer | Question it answers | When you write it |
|-------|---------------------|-------------------|
| **1. Intent Map** | *What does this thing do and why?* | Before anything else |
| **2. Interface Contract** | *What talks to what, and what do they promise each other?* | After you know the pieces |
| **3. File Skeleton** | *Where does each piece live on disk?* | After contracts are defined |
| **4. Task Queue** | *What do I (or the AI) build next?* | Ongoing, always updated |

**Rule: Never skip a layer.** If you can't fill a layer, you don't understand the project well enough yet.

---

## Layer 1: The Intent Map

This is the "napkin sketch." No code, no types — just boxes and arrows in your head, written down.

### Template

```
PROJECT: [name]
ONE-LINE: [what it does in one sentence, for a human]

ACTORS:
  - [who/what uses this system]
  - [who/what does this system talk to]

CORE FLOWS:
  1. [Actor] does [action] → [what happens] → [end result]
  2. [Actor] does [action] → [what happens] → [end result]

HARD PARTS:
  - [the thing that makes this non-trivial]
  - [the constraint you're working around]
```

### Real Example

```
PROJECT: logpunch
ONE-LINE: CLI tool that tails log files, detects error patterns, and sends alerts to Slack/Discord

ACTORS:
  - Developer running it in terminal
  - Log files being written to by other services
  - Slack/Discord webhook endpoints

CORE FLOWS:
  1. Dev starts logpunch with a config → it watches files → detects patterns → sends alert
  2. Dev adds a new pattern rule → config reloads hot → starts matching immediately

HARD PARTS:
  - File watching needs to handle log rotation (file gets renamed/recreated)
  - Pattern matching needs to be fast enough to not fall behind on high-volume logs
  - Need to debounce alerts so you don't get 500 Slack messages in 10 seconds
```

**Why this matters:** When AI generates code and you get lost, you come back here. This tells you *what the program is supposed to do*. You can always re-orient from this.

---

## Layer 2: The Interface Contract

This is the core of the pattern. **Interfaces are promises between pieces of your system.** This is where most people (including you) get lost — so we're going to be very concrete.

### The 3 Types of Interfaces

Every interface in any program falls into one of these:

```
┌─────────────────────────────────────────────────────┐
│                                                     │
│  TYPE A: "Data Shape"                               │
│  A struct/type/schema that pieces pass around       │
│  Example: a LogEvent, a UserProfile, a Config       │
│                                                     │
│  TYPE B: "Capability"                               │
│  A trait/interface/protocol — "I can do X"          │
│  Example: Watcher, Matcher, Notifier                │
│                                                     │
│  TYPE C: "Boundary"                                 │
│  Where your code meets the outside world            │
│  Example: CLI args, HTTP endpoint, file system      │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### How to Dissect Interfaces (The Cheat Sheet)

When you're looking at code (yours or AI-generated) and can't figure out how things connect, ask these questions in order:

```
1. "What DATA flows through this system?"
   → List every struct/type/object that gets passed between functions
   → These are your Type A interfaces

2. "What BEHAVIORS does the system need?"
   → List every verb: watches, matches, notifies, parses, transforms
   → Each verb is a candidate for a Type B interface

3. "Where does the system touch the REAL WORLD?"
   → CLI input, files, network, database, environment variables
   → These are your Type C interfaces — they need special handling
     (error recovery, validation, timeouts)
```

### Template

For each interface, write a contract using this format:

```
INTERFACE: [Name]
TYPE: A (data) | B (capability) | C (boundary)
PURPOSE: [one line — what it represents]
SHAPE:
  [field/method]: [type] — [what it means]
  [field/method]: [type] — [what it means]
USED BY: [which pieces consume this]
PRODUCED BY: [which pieces create/implement this]
RULES:
  - [invariant — something that must always be true]
  - [constraint — something this must never do]
```

### Real Example (continuing logpunch)

```
INTERFACE: LogEvent
TYPE: A (data)
PURPOSE: A single parsed log line ready for pattern matching
SHAPE:
  timestamp:  DateTime   — when the log line was written
  source:     String     — which file it came from
  level:      Error|Warn|Info|Debug — parsed log level
  message:    String     — the actual log content
  raw:        String     — unparsed original line (for forwarding)
USED BY: Matcher, Notifier (includes raw line in alert)
PRODUCED BY: Watcher (parses raw file lines into this)
RULES:
  - timestamp is always UTC, converted at parse time
  - if level can't be parsed, default to Info (don't crash)

---

INTERFACE: Matcher
TYPE: B (capability)
PURPOSE: Something that can check if a LogEvent matches a pattern
SHAPE:
  matches(event: LogEvent) -> Option<MatchResult>
USED BY: Core pipeline (loops through matchers for each event)
PRODUCED BY: RegexMatcher, GlobMatcher (two implementations)
RULES:
  - Must be stateless — same input always gives same output
  - Must return in < 1ms for a single event

---

INTERFACE: FileSource
TYPE: C (boundary)
PURPOSE: Reads log lines from a file, handling rotation
SHAPE:
  watch(path: Path) -> Stream<RawLine>
  handles rotation:  yes — detects inode change
  handles truncation: yes — resets read position
USED BY: Watcher module
PRODUCED BY: OS-specific implementations
RULES:
  - Never holds file lock (other processes need to write)
  - Emits a "rotated" event when file changes underneath
  - Must work on Linux and macOS (inotify / kqueue)
```

### The Connection Diagram

After writing contracts, draw how they connect. This is the "boxes and arrows" — but now each arrow is labeled with the interface it carries:

```
                    ┌──────────────┐
                    │  Config (A)  │
                    │  loaded from │
                    │  TOML file   │
                    └──────┬───────┘
                           │ configures
              ┌────────────┼────────────┐
              ▼            ▼            ▼
     ┌──────────────┐ ┌────────┐ ┌──────────┐
     │ FileSource(C)│ │Matcher │ │Notifier  │
     │              │ │  (B)   │ │   (B)    │
     │ watches disk │ │patterns│ │Slack/Disc│
     └──────┬───────┘ └────┬───┘ └─────┬────┘
            │              │           │
            │ RawLine      │           │
            ▼              │           │
     ┌──────────────┐      │           │
     │   Watcher    │      │           │
     │ parses lines │      │           │
     └──────┬───────┘      │           │
            │              │           │
            │ LogEvent(A)  │           │
            ▼              │           │
     ┌──────────────────┐  │           │
     │    Pipeline      │◄─┘           │
     │ event + matchers │              │
     │ = match results  │              │
     └──────┬───────────┘              │
            │                          │
            │ MatchResult(A)           │
            ▼                          │
     ┌──────────────────┐             │
     │   Alert Router   │─────────────┘
     │ debounce + send  │  Alert(A)
     └──────────────────┘
```

**This diagram is your lifeline.** When you get lost in code, come back to this. Every box is a module/file. Every arrow is a type being passed. Every label tells you what to look for in the code.

---

## Layer 3: The File Skeleton

Map your interface contracts to actual files on disk. This is your "find things fast" index.

### Template

```
project-root/
├── src/
│   ├── main.rs              ← ENTRY: parses CLI args [Boundary: CLI]
│   ├── config.rs            ← [Data: Config] loads and validates TOML
│   ├── pipeline.rs          ← CORE: orchestrates watch→match→alert loop
│   │
│   ├── watch/
│   │   ├── mod.rs           ← [Capability: Watcher trait]
│   │   ├── file_source.rs   ← [Boundary: FileSource] OS file watching
│   │   └── parser.rs        ← RawLine → LogEvent conversion
│   │
│   ├── match/
│   │   ├── mod.rs           ← [Capability: Matcher trait]
│   │   ├── regex.rs         ← RegexMatcher implementation
│   │   └── glob.rs          ← GlobMatcher implementation
│   │
│   ├── notify/
│   │   ├── mod.rs           ← [Capability: Notifier trait]
│   │   ├── slack.rs         ← [Boundary: Slack webhook]
│   │   └── discord.rs       ← [Boundary: Discord webhook]
│   │
│   └── types.rs             ← ALL shared data types:
│                                LogEvent, MatchResult, Alert, Config
│
├── config.example.toml      ← Example config with comments
└── README.md
```

### Annotation Rules

Every file gets ONE of these tags so you can instantly know what it does:

- **`← ENTRY`** — where execution starts
- **`← CORE`** — the orchestration logic (glue between interfaces)
- **`← [Data: Name]`** — defines a data shape (Type A)
- **`← [Capability: Name]`** — defines a behavior trait/interface (Type B)
- **`← [Boundary: Name]`** — touches the outside world (Type C)

**Why this matters:** When you (or your AI) need to change something, you know exactly which file to open. When AI generates a new file, you annotate it here immediately so you don't lose track.

---

## Layer 4: The Task Queue

This is what keeps you going when the AI hits a limit or when you come back after a break.

### Format

```
## Status: [phase]

### DONE ✓
- [x] Define Config data shape
- [x] Implement FileSource with inotify
- [x] Basic RegexMatcher

### IN PROGRESS →
- [ ] Alert debouncing in AlertRouter
      CONTEXT: needs a sliding window per (source, pattern) pair
      BLOCKED: deciding on time window — 30s? configurable?
      FILES: src/notify/mod.rs (add debounce logic before send)

### NEXT UP
- [ ] Discord notifier (copy Slack pattern, different payload format)
      DEPENDS ON: Notifier trait being stable (it is now)
      FILES: src/notify/discord.rs (new file)
      APPROACH: same HTTP post pattern as slack.rs, different JSON body

### ICEBOX (later)
- [ ] Glob matcher
- [ ] Config hot-reload
- [ ] Metrics/stats endpoint
```

### Rules for the Task Queue

1. **Every task names the FILES it touches** — so you know where to look
2. **Every "in progress" task has CONTEXT** — what you were thinking when you started
3. **Every "next up" task has APPROACH** — a 1-2 sentence plan, not code
4. **Update this BEFORE you close your laptop** — future-you will thank present-you

---

## How to Use This With AI Agents

### Before the AI writes anything:

1. YOU write Layers 1-3 (even roughly)
2. Give the AI your Blueprint as context
3. Say: "Implement [specific task from Layer 4]. Follow the interface contracts in the Blueprint."

### While the AI is working:

- If it creates a new file → immediately add it to Layer 3 with annotations
- If it adds a new type → add it to Layer 2 as a data contract
- If it changes how things connect → update the connection diagram

### When the AI hits a limit or goes off-track:

1. Open Layer 4 → read the CONTEXT of the current task
2. Open Layer 3 → find the exact files involved
3. Open Layer 2 → read the interface contracts for those files
4. You now know: what the code should do, where it lives, and what it promises

### The Recovery Checklist

```
"I'm lost. What do I do?"

□ Read Layer 1 → What is this project even doing?
□ Read Layer 2 → What are the pieces and their promises?
□ Read Layer 3 → Where are the pieces on disk?
□ Read Layer 4 → What was I working on and what's next?
□ Pick up from Layer 4 → give the next task to a new AI session
```

---

## Quick-Reference: "When Do I Use What?"

When you're stuck on how to design something, use this decision tree:

```
"I need to represent some data"
  → Type A: Data Shape
  → Make a struct/type
  → Put it in your shared types file

"I need multiple things that do the same job differently"
  → Type B: Capability (trait/interface)
  → Define the trait with its methods
  → Each implementation goes in its own file

"I need to talk to something outside my program"
  → Type C: Boundary
  → ALWAYS wrap it (never use raw file/network/CLI in core logic)
  → Handle errors at this layer, return clean types to core
  → Put it in its own module

"I don't know if this should be one module or two"
  → If it has one responsibility → one module
  → If you're saying "and" to describe it → split it
  → Example: "parses config AND watches files" → two modules

"I don't know if this needs a trait/interface"
  → Will there ever be 2+ implementations? → yes, trait
  → Do you want to test it in isolation? → yes, trait
  → Is it just one concrete thing? → no trait, just a module
```

---

## Starter Template (Copy-Paste This)

```markdown
# Blueprint: [PROJECT NAME]

## Layer 1: Intent Map

PROJECT: [name]
ONE-LINE: [what it does]

ACTORS:
  -

CORE FLOWS:
  1. [actor] → [action] → [result]

HARD PARTS:
  -

## Layer 2: Interface Contracts

INTERFACE: [Name]
TYPE: A | B | C
PURPOSE:
SHAPE:
USED BY:
PRODUCED BY:
RULES:
  -

[Connection Diagram]

## Layer 3: File Skeleton

project/
├── src/
│   ├── main     ← ENTRY:
│   ├── types    ← ALL shared types
│   └── ...

## Layer 4: Task Queue

### DONE ✓
### IN PROGRESS →
### NEXT UP
### ICEBOX
```

---

*This pattern works because it externalizes the mental model you'd normally keep in your head. Your attention issues mean that model gets dropped — so we put it on paper. The AI implements the details. You own the architecture. That's not fraud — that's engineering.*

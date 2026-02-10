# Blueprint: [PROJECT NAME]

> **Started:** [date]
> **Last updated:** [date]
> **Status:** Planning | In Progress | Maintaining

---

## Layer 1: Intent Map

**PROJECT:** [name]

**ONE-LINE:** [what it does, in one sentence, for a human]

**ACTORS:**
<!-- who/what uses this system, and who/what does it talk to -->
- [actor 1]
- [actor 2]

**CORE FLOWS:**
<!-- the main things that happen, step by step -->
1. [Actor] does [action] → [what happens] → [end result]
2. [Actor] does [action] → [what happens] → [end result]

**HARD PARTS:**
<!-- the things that make this non-trivial — constraints, edge cases, tricky decisions -->
- [hard part 1]
- [hard part 2]

**NON-GOALS:**
<!-- things this project deliberately does NOT do (prevents scope creep) -->
- [non-goal 1]

---

## Layer 2: Interface Contracts

<!-- 
  For each interface, pick a type:
    A = Data Shape    (struct/type that gets passed around)
    B = Capability    (trait/interface — "I can do X")  
    C = Boundary      (where code meets outside world: CLI, files, network, DB)
-->

### Data Shapes (Type A)

#### [DataName1]

| Field | Type | Meaning |
|-------|------|---------|
| | | |
| | | |

- **Used by:** [which modules consume this]
- **Produced by:** [which modules create this]
- **Rules:**
  - [invariant — something that must always be true]

#### [DataName2]

| Field | Type | Meaning |
|-------|------|---------|
| | | |

- **Used by:**
- **Produced by:**
- **Rules:**
  -

---

### Capabilities (Type B)

#### [TraitName1]

**Purpose:** [one line — what behavior this represents]

| Method | Signature | What it does |
|--------|-----------|-------------|
| | | |

- **Implementations:** [list concrete types that implement this]
- **Rules:**
  - [constraint — e.g. must be stateless, must return in <Xms]

#### [TraitName2]

**Purpose:**

| Method | Signature | What it does |
|--------|-----------|-------------|
| | | |

- **Implementations:**
- **Rules:**
  -

---

### Boundaries (Type C)

#### [BoundaryName1]

**Purpose:** [what outside thing this wraps]
**Touches:** [file system | network | CLI | database | env vars | ...]

| Operation | Input → Output | Notes |
|-----------|---------------|-------|
| | | |

- **Error handling:** [what happens when the outside world fails]
- **Rules:**
  - [e.g. never holds file lock, must timeout after Xs]

---

### Connection Diagram

<!-- 
  Draw how interfaces connect. Label every arrow with the data type it carries.
  Use ASCII or paste a mermaid/excalidraw link.
  
  Example:
  
  [FileSource(C)] --RawLine--> [Parser] --LogEvent(A)--> [Pipeline]
                                                             |
                                                     MatchResult(A)
                                                             |
                                                             v
                                                      [Notifier(B)]
-->

```
[replace with your diagram]
```

---

## Layer 3: File Skeleton

<!--
  Annotate every file with ONE tag:
    ← ENTRY:              where execution starts
    ← CORE:               orchestration / glue logic
    ← [Data: Name]        defines a data shape (Type A)
    ← [Capability: Name]  defines a behavior trait (Type B)
    ← [Boundary: Name]    touches outside world (Type C)
-->

```
project-root/
├── src/
│   ├── main              ← ENTRY: [what it does]
│   ├── types             ← [Data: all shared types]
│   ├── [core_module]     ← CORE: [orchestration description]
│   │
│   ├── [module_group_1]/
│   │   ├── mod           ← [Capability: TraitName]
│   │   ├── [impl_1]      ← [implementation description]
│   │   └── [impl_2]      ← [implementation description]
│   │
│   ├── [module_group_2]/
│   │   ├── mod           ← [Capability: TraitName]
│   │   └── [impl]        ← [Boundary: what it wraps]
│   │
│   └── [module_group_3]/
│       ├── mod           ← [Capability: TraitName]
│       └── [impl]        ← [Boundary: what it wraps]
│
├── config/
│   └── [config_file]     ← example/default config
├── tests/
│   └── ...
└── README.md
```

---

## Layer 4: Task Queue

<!-- 
  Rules:
  1. Every task names the FILES it touches
  2. Every "in progress" task has CONTEXT (what you were thinking)
  3. Every "next up" task has APPROACH (1-2 sentence plan)
  4. UPDATE THIS BEFORE YOU STOP WORKING — future you will thank you
-->

### DONE ✓

- [x] [task description]
- [x] [task description]

### IN PROGRESS →

- [ ] **[task description]**
  - **Context:** [what you were thinking / where you left off]
  - **Blocked?** [yes/no — if yes, on what?]
  - **Files:** [exact files being touched]

### NEXT UP

- [ ] **[task description]**
  - **Depends on:** [what needs to be done first, or "nothing"]
  - **Files:** [files to create or modify]
  - **Approach:** [1-2 sentence plan — NOT code, just the idea]

- [ ] **[task description]**
  - **Depends on:**
  - **Files:**
  - **Approach:**

### ICEBOX (later)

- [ ] [idea for later]
- [ ] [idea for later]

---

## AI Agent Instructions

<!--
  Paste this section (or a version of it) into your AI prompt when starting a session.
  It tells the AI to follow YOUR architecture, not invent its own.
-->

```
You are implementing code for this project. Follow these rules:

1. Read the Blueprint above before writing any code.
2. Follow the interface contracts exactly — types, method signatures, rules.
3. Place files according to the File Skeleton. Do not invent new directories.
4. If you need a new type, tell me — I'll add it to Layer 2 first.
5. If you need to change an interface, STOP and explain why before changing it.
6. When you finish a task, tell me:
   - What files you created/modified
   - Any new types or interfaces you introduced
   - What should be updated in the Blueprint
7. Work on ONE task from the Task Queue at a time.
```

---

## Recovery Checklist

<!--
  "I'm lost. The AI hit its limit. I don't know what's going on."
  Go through these in order:
-->

```
□ Read Layer 1 → What is this project even doing?
□ Read Layer 2 → What are the pieces and their promises?
□ Read Layer 3 → Where are the pieces on disk?
□ Read Layer 4 → What was I working on and what's next?
□ Pick up from Layer 4 → give the next task to a fresh AI session
```

---

## Quick Decision Reference

```
"I need to represent some data"
  → Type A → struct/type → add to shared types file

"I need multiple things that do the same job differently"
  → Type B → trait/interface → each impl in its own file

"I need to talk to something outside my program"
  → Type C → Boundary → wrap it, handle errors here, return clean types

"One module or two?"
  → If you say "and" to describe it → two modules

"Does this need a trait?"
  → 2+ implementations? → yes
  → Want to test in isolation? → yes
  → Just one concrete thing? → no, just a module
```

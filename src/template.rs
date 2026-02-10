pub const TEMPLATE: &str = r#"# Blueprint: {PROJECT_NAME}

> **Started:** {DATE}
> **Last updated:** {DATE}
> **Status:** Planning

---

## Layer 1: Intent Map

**PROJECT:** {PROJECT_NAME}

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
│   └── ...
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

### ICEBOX (later)

- [ ] [idea for later]
"#;

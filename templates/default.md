<!-- Description: Full template with detailed instructions and examples -->
# Blueprint: {PROJECT_NAME}

> **Started:** {DATE}
> **Last updated:** {DATE}
> **Status:** Planning

---

## Layer 1: Intent Map

**PROJECT:** {PROJECT_NAME}

**ONE-LINE:** [what it does, in one sentence, for a human]

**ACTORS:**
> Who/what uses this system, and who/what does it talk to
- [actor 1]
- [actor 2]

**CORE FLOWS:**
> The main things that happen, step by step
1. [Actor] does [action] → [what happens] → [end result]
2. [Actor] does [action] → [what happens] → [end result]

**HARD PARTS:**
> Things that make this non-trivial — constraints, edge cases, tricky decisions
- [hard part 1]
- [hard part 2]

**NON-GOALS:**
> Things this project deliberately does NOT do (prevents scope creep)
- [non-goal 1]

---

## Layer 2: Interface Contracts

> For each interface, pick a type:
> A = Data Shape (struct/type that gets passed around)
> B = Capability (trait/interface — "I can do X")
> C = Boundary (where code meets outside world: CLI, files, network, DB)

### Data Shapes (Type A)

#### [DataName1]

**Fields:**
- `field1` (type) — [what it means]
- `field2` (type) — [what it means]

**Used by:** [which modules consume this]

**Produced by:** [which modules create this]

**Rules:**
- [invariant — something that must always be true]

---

### Capabilities (Type B)

#### [TraitName1]

**Purpose:** [one line — what behavior this represents]

**Methods:**
- `method_name(args) -> Result` — [what it does]
- `another_method(args) -> Type` — [what it does]

**Implementations:** [list concrete types that implement this]

**Rules:**
- [constraint — e.g. must be stateless, must return in <Xms]

---

### Boundaries (Type C)

#### [BoundaryName1]

**Purpose:** [what outside thing this wraps]

**Touches:** [file system | network | CLI | database | env vars | ...]

**Operations:**
- `operation_name(input) -> output` — [notes about this operation]
- `another_op(input) -> output` — [notes]

**Error handling:** [what happens when the outside world fails]

**Rules:**
- [e.g. never holds file lock, must timeout after Xs]

---

### Connection Diagram

> Draw how interfaces connect. Label arrows with the data type they carry.

```
[replace with your diagram]
```

---

## Layer 3: File Skeleton

> Annotate every file with ONE tag:
> ENTRY = where execution starts, CORE = orchestration/glue
> Data:Name = defines a data shape, Cap:Name = defines a trait, Boundary:Name = touches outside world

```
project-root/
├── src/
│   ├── main              ← ENTRY: [what it does]
│   ├── types             ← Data: [all shared types]
│   └── ...
```

---

## Layer 4: Task Queue

> Rules: every task names FILES it touches, in-progress tasks have CONTEXT,
> next-up tasks have APPROACH. Update this before you stop working.

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

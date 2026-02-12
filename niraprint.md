<!-- Description: Compact version with essential sections only -->
# Blueprint: TestProject

> **Started:** 2026-02-11
> **Status:** Planning

## Layer 1: Intent Map

**PROJECT:** TestProject

**ONE-LINE:** [what it does, in one sentence]

**ACTORS:**
- [who/what uses this]
- [who/what it talks to]

**CORE FLOWS:**
1. [Actor] does [action] → [result]
2. [Actor] does [action] → [result]

**HARD PARTS:**
- [constraint or tricky thing]

**NON-GOALS:**
- [what this deliberately does NOT do]

---

## Layer 2: Interface Contracts

### Data Shapes (Type A)

**[DataName]**
- field: type — meaning
- Used by: [consumers]
- Produced by: [creators]

### Capabilities (Type B)

**[TraitName]** — [what behavior]
- method(input) -> output — what it does
- Implementations: [concrete types]

### Boundaries (Type C)

**[BoundaryName]** — touches [file system | network | CLI | etc]
- operation: input → output
- Error handling: [what happens when it fails]

---

## Layer 3: File Skeleton

```
project/
├── src/
│   ├── main     ← ENTRY: [starts here]
│   ├── types    ← Data: [shared types]
│   └── ...
```

---

## Layer 4: Task Queue

### DONE ✓
- [x] [completed task]

### IN PROGRESS →
- [ ] **[current task]**
  - **Context:** [where you left off]
  - **Files:** [which files]

### NEXT UP
- [ ] **[next task]**
  - **Depends on:** [prerequisites]
  - **Files:** [which files]
  - **Approach:** [how to do it]

### ICEBOX
- [ ] [idea for later]

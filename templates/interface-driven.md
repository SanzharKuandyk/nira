<!-- Description: Start with contracts and APIs, implementation follows -->
# Interface Blueprint: {PROJECT_NAME}

> **Started:** {DATE}
> **Last updated:** {DATE}
> **Status:** Planning
> **Approach:** Interface-Driven (define contracts first, implement later)

---

## 1. PUBLIC API

<!--
  Start here: what does the outside world see?
  This is your contract with users/callers.
  Everything else flows from this.
-->

**Project:** {PROJECT_NAME}

**What it does (one line):** [the promise to the outside world]

### Entry Points

<!--
  How do users/systems interact with this?
  CLI commands, function calls, HTTP endpoints, etc.
-->

**Entry point 1:**
- Name: `command_name` or `function_name` or `/endpoint`
- Signature: [input params]
- Returns: [output type]
- Purpose: [what it does]

**Entry point 2:**
- Name: [name]
- Signature: [input params]
- Returns: [output type]
- Purpose: [what it does]

**Examples:**
```
[concrete usage examples showing the public API in action]
```

**Stability Promise:**
<!-- Which parts are stable, which might change -->
- **Stable:** [what you commit to not breaking]
- **Unstable:** [what might change, marked experimental]

---

## 2. CORE DATA TYPES

<!--
  The nouns of your system.
  What data flows through the interfaces?
  Define these BEFORE thinking about implementation.
-->

### [TypeName1]

**Purpose:** [what this represents in the domain]

**Shape:**
```
[pseudo-code or actual type definition]
```

**Fields:**
- `field1` (type) — [meaning] — Required: Yes/No — Validated: [validation rule]
- `field2` (type) — [meaning] — Required: Yes/No — Validated: [validation rule]

**Flows through:**
<!-- Which interfaces consume/produce this type -->
- **Created by:** [which API/module]
- **Consumed by:** [which API/module]
- **Transformed by:** [which operations]

**Invariants:**
- [rule that must always be true]
- [another invariant]

---

### [TypeName2]

[repeat structure above for each core type]

---

## 3. INTERNAL CONTRACTS

<!--
  Interfaces between modules INSIDE your system.
  These are your trait definitions, internal APIs, module boundaries.
-->

### Interface: [InterfaceName]

**Purpose:** [what capability this represents]

**Contract:**
```
[trait definition or interface spec]
```

**Methods:**
- `method_name(input) -> output` — Purpose: [what it does] — Errors: [error conditions]
- `another_method(input) -> output` — Purpose: [what it does] — Errors: [error conditions]

**Implementations:**
<!-- Which concrete types provide this interface -->
- `[ConcreteType1]` — [when to use this implementation]
- `[ConcreteType2]` — [when to use this implementation]

**Usage:**
```
[code example showing how this interface is used]
```

---

## 4. IMPLEMENTATION MODULES

<!--
  Now map interfaces to files.
  Each module implements or composes interfaces defined above.
-->

### File Structure

```
project-root/
├── src/
│   ├── main              ← ENTRY: uses [PublicAPI interfaces]
│   │
│   ├── api/              ← PUBLIC API (section 1)
│   │   ├── mod.rs        ← Exports: [list public functions/types]
│   │   └── ...           ← Implements: [which entry points]
│   │
│   ├── types/            ← CORE DATA TYPES (section 2)
│   │   ├── mod.rs        ← Exports: [list types]
│   │   ├── type1.rs      ← Defines: [TypeName1]
│   │   └── type2.rs      ← Defines: [TypeName2]
│   │
│   ├── [module_name]/    ← INTERNAL CONTRACT (section 3)
│   │   ├── mod.rs        ← Exports: [InterfaceName trait]
│   │   ├── impl1.rs      ← Implements: [InterfaceName for ConcreteType1]
│   │   └── impl2.rs      ← Implements: [InterfaceName for ConcreteType2]
│   │
│   └── ...
```

### Module Annotations

**For each module, specify:**

#### `src/[module_name]`

- **Exports:** [which types/functions/traits are public from this module]
- **Implements:** [which interfaces from section 3]
- **Uses types:** [which core types from section 2]
- **Depends on:** [which other modules]
- **Tested via:** [unit tests in same file, or integration tests where]

---

## 5. INTEGRATION POINTS

<!--
  Show how interfaces connect to each other.
  Draw the data flow between modules.
-->

### Flow Diagram

```
[User/Caller]
      |
      | calls
      v
[Public API: entry_point]
      |
      | creates → [CoreType1]
      v
[Internal Interface: InterfaceName]
      |
      | transforms → [CoreType2]
      v
[Concrete Implementation]
      |
      | returns → [Result<CoreType3>]
      v
[Public API: response]
      |
      v
[User/Caller]
```

### Integration Rules

**Data flow constraints:**
- [rule: e.g., "Public API never returns internal types directly"]
- [rule: e.g., "Core types are immutable after creation"]

**Dependency direction:**
- [rule: e.g., "Internal modules never import from public API"]
- [rule: e.g., "Types module has zero dependencies"]

---

## 6. INTERFACE CHANGE PROTOCOL

<!--
  How do we evolve interfaces without breaking things?
-->

### Adding New Features

**To add a new entry point:**
1. [step 1: e.g., "Define input/output types in section 2"]
2. [step 2: e.g., "Add entry point to public API table in section 1"]
3. [step 3: e.g., "Create internal interface if needed in section 3"]
4. [step 4: e.g., "Implement and test"]

**To modify existing interface:**
1. [step 1: e.g., "Check stability promise in section 1"]
2. [step 2: e.g., "If stable, must be backward compatible"]
3. [step 3: e.g., "Update all call sites before merging"]

### Breaking Changes

**When breaking change is necessary:**
- [ ] Document in CHANGELOG
- [ ] Update section 1 stability promise
- [ ] Create migration guide
- [ ] Bump major version

---

## 7. TASK QUEUE (Organized by Interface)

<!--
  Group tasks by which interface they touch.
  This makes it easy to work on one interface at a time.
-->

### DONE ✓

- [x] [task description] — *interface: [which one]*

### IN PROGRESS → Public API Tasks

- [ ] **[task description]**
  - **Interface:** [entry point from section 1]
  - **Changes:** [what's being added/modified in public API]
  - **Files:** [list]
  - **Tests:** [which public API tests need updating]

### IN PROGRESS → Core Type Tasks

- [ ] **[task description]**
  - **Type:** [which core type from section 2]
  - **Changes:** [new fields, validation rules, etc]
  - **Impact:** [which interfaces use this type]
  - **Files:** [list]

### IN PROGRESS → Internal Contract Tasks

- [ ] **[task description]**
  - **Interface:** [which internal contract from section 3]
  - **Changes:** [new methods, implementations]
  - **Files:** [list]

### NEXT UP

- [ ] **[task description]**
  - **Interface:** [which one]
  - **Approach:** [how to implement while respecting contract]
  - **Files:** [list]

### ICEBOX

- [ ] [future interface idea]

---

## 8. TESTING MATRIX

<!--
  For each interface, what needs testing?
-->

**Interface: [API name]**
- Test type: Unit
- What to test: [specific behavior]
- Test file: [file path]

**Interface: [API name]**
- Test type: Integration
- What to test: [interface interaction]
- Test file: [file path]

**Type: [Type name]**
- Test type: Property
- What to test: [invariant to verify]
- Test file: [file path]

**Coverage goals:**
- [ ] All public API entry points have integration tests
- [ ] All core types have validation tests
- [ ] All internal contracts have at least 2 implementations (real + test)
- [ ] All integration points have flow tests

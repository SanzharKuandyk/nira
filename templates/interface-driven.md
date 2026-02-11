<!-- Description: Start with contracts and APIs, implementation follows -->
# Interface Blueprint: {PROJECT_NAME}

> **Started:** {DATE}
> **Last updated:** {DATE}
> **Status:** Planning
> **Approach:** Interface-Driven (define contracts first, implement later)

---

## 1. PUBLIC API

> Start here: what does the outside world see? Everything else flows from this.

**Project:** {PROJECT_NAME}

**What it does (one line):** [the promise to the outside world]

### Entry Points

> How do users/systems interact with this? CLI commands, function calls, HTTP endpoints, etc.

**[command/function/endpoint]**
- Signature: [input params] → [output type]
- Purpose: [what it does]

**[command/function/endpoint]**
- Signature: [input params] → [output type]
- Purpose: [what it does]

**Examples:**
```
[concrete usage examples showing the public API in action]
```

**Stability Promise:**
- Stable: [what you commit to not breaking]
- Unstable: [what might change, marked experimental]

---

## 2. CORE DATA TYPES

> The nouns of your system. Define these BEFORE thinking about implementation.

### [TypeName1]

**Purpose:** [what this represents in the domain]

**Shape:**
```
[pseudo-code or actual type definition]
```

**Fields:**
- field_name: Type — meaning (required/optional, validation rule)
- field_name: Type — meaning (required/optional, validation rule)

**Flows through:**
- Created by: [which API/module]
- Consumed by: [which API/module]
- Transformed by: [which operations]

**Invariants:**
- [rule that must always be true]

---

### [TypeName2]

> Repeat the structure above for each core type.

---

## 3. INTERNAL CONTRACTS

> Interfaces between modules INSIDE your system — trait definitions, internal APIs, module boundaries.

### Interface: [InterfaceName]

**Purpose:** [what capability this represents]

**Contract:**
```
[trait definition or interface spec]
```

**Methods:**
- method_name: input → output — purpose (error conditions)
- method_name: input → output — purpose (error conditions)

**Implementations:**
- `ConcreteType1` — [when to use this implementation]
- `ConcreteType2` — [when to use this implementation]

**Usage:**
```
[code example showing how this interface is used]
```

---

## 4. IMPLEMENTATION MODULES

> Map interfaces to files. Each module implements or composes interfaces defined above.

### File Structure

```
project-root/
├── src/
│   ├── main              ← ENTRY: uses [PublicAPI interfaces]
│   ├── api/              ← PUBLIC API (section 1)
│   │   ├── mod           ← Exports: [list public functions/types]
│   │   └── ...           ← Implements: [which entry points]
│   ├── types/            ← CORE DATA TYPES (section 2)
│   │   ├── mod           ← Exports: [list types]
│   │   └── ...
│   ├── [module_name]/    ← INTERNAL CONTRACT (section 3)
│   │   ├── mod           ← Exports: [InterfaceName trait]
│   │   ├── impl1         ← Implements: [InterfaceName for ConcreteType1]
│   │   └── impl2         ← Implements: [InterfaceName for ConcreteType2]
│   └── ...
```

> For each module, note: what it exports, what interfaces it implements,
> which types it uses, what it depends on, and how it's tested.

---

## 5. INTEGRATION POINTS

> How interfaces connect to each other. Show the data flow between modules.

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

**Adding a new entry point:**
1. Define input/output types in section 2
2. Add entry point to public API in section 1
3. Create internal interface if needed in section 3
4. Implement and test

**Modifying existing interface:**
1. Check stability promise in section 1
2. If stable, must be backward compatible
3. Update all call sites before merging

**Breaking changes checklist:**
- [ ] Document in CHANGELOG
- [ ] Update section 1 stability promise
- [ ] Create migration guide
- [ ] Bump major version

---

## 7. TASK QUEUE (Organized by Interface)

> Group tasks by which interface they touch. Work on one interface at a time.

### DONE ✓

- [x] [task description] — *interface: [which one]*

### IN PROGRESS →

- [ ] **[task description]**
  - **Context:** [where you left off]
  - **Interface:** [which entry point / type / contract from sections above]
  - **Changes:** [what's being added/modified]
  - **Files:** [list]

### NEXT UP

- [ ] **[task description]**
  - **Depends on:** [prerequisites, or "nothing"]
  - **Interface:** [which one]
  - **Files:** [list]
  - **Approach:** [how to implement while respecting contract]

### ICEBOX

- [ ] [future interface idea]

---

## 8. TESTING STRATEGY

> For each interface layer, what needs testing?

**Public API tests:**
- [API name]: [specific behavior to test] — [test file]

**Core type tests:**
- [Type name]: [invariant to verify] — [test file]

**Internal contract tests:**
- [Interface name]: [interaction to test] — [test file]

**Coverage goals:**
- [ ] All public API entry points have integration tests
- [ ] All core types have validation tests
- [ ] All internal contracts have at least 2 implementations (real + test)
- [ ] All integration points have flow tests

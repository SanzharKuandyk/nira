<!-- Description: Start with boundaries and limits, build around constraints -->
# Constraints Blueprint: {PROJECT_NAME}

> **Started:** {DATE}
> **Last updated:** {DATE}
> **Status:** Planning
> **Approach:** Constraints-First (define boundaries, then build within them)

---

## 1. BOUNDARIES & LIMITS

### What We CAN'T Do / What's HARD

<!--
  List the constraints FIRST. These are the guardrails that guide all decisions.
  Examples: "Can't block UI thread >16ms", "Must work offline", "Max 100MB memory"
-->

**Technical Constraints:**
- [hard technical limit — e.g., platform API restrictions, language limitations]
- [performance requirement — e.g., must respond in <Xms, handle N req/sec]

**External Dependencies:**
- [third-party service limits — e.g., API rate limits, quota]
- [environment constraints — e.g., network unreliable, limited disk space]

**Non-Negotiable Rules:**
- [business/security requirement — e.g., must encrypt at rest, GDPR compliant]
- [architectural constraint — e.g., no global state, stateless services]

### System Boundaries

**What This System WILL Touch:**
<!-- Be specific about external systems, I/O, side effects -->
- [ ] File system — [read/write/both, which directories, size limits]
- [ ] Network — [HTTP/WebSocket/etc, which endpoints, timeout requirements]
- [ ] Database — [which DB, transaction requirements, consistency needs]
- [ ] Environment — [env vars, config files, system resources]
- [ ] User input — [CLI args, interactive prompts, validation rules]

**What This System MUST NOT Touch:**
<!-- Explicit forbidden zones to prevent scope creep and violations -->
- [system resource to avoid — e.g., no registry writes, no kernel modules]
- [data to avoid — e.g., never store credentials, never log PII]

---

## 2. INTERFACE CONTRACTS (Designed Around Constraints)

<!--
  Now define interfaces that RESPECT the boundaries above.
  Every interface should reference which constraint it satisfies or avoids.
-->

### Core Data Types

#### [TypeName]

| Field | Type | Constraint it satisfies |
|-------|------|------------------------|
| | | [which boundary/limit does this field respect?] |

**Invariants:**
<!-- Rules that must ALWAYS be true, derived from constraints -->
- [invariant] — *respects: [constraint from section 1]*

---

### Safe Operations

<!--
  Operations designed to stay within boundaries.
  Mark risky operations clearly.
-->

#### [OperationName]

**Purpose:** [what it does]

**Constraints respected:**
- [boundary] — [how this operation respects it]
- [limit] — [how this operation stays within it]

**Signature:**
```
[pseudo-code or actual signature]
```

**Error conditions:**
<!-- What happens when we HIT a constraint -->
- Fails if: [condition that would violate constraint]
- Returns: [error type]
- Fallback: [safe degradation strategy]

---

### Risky Areas

<!--
  Operations that approach constraint boundaries.
  These need extra testing, monitoring, or careful implementation.
-->

**[Risky operation/module name]:**
- **Why risky:** [which constraint does this push against]
- **Mitigation:** [how we stay safe — timeouts, circuit breakers, etc]
- **Monitoring:** [what to measure to detect violations]

---

## 3. SAFE PATHS (Implementation Strategy)

<!--
  Now that we know the constraints and safe interfaces,
  describe the implementation approach that stays within bounds.
-->

### Architecture Decisions

**Decision:** [architectural choice made]
**Reason:** Respects constraints: [list which boundaries this satisfies]
**Trade-off:** [what we give up to respect this constraint]

---

### Module Structure

```
project-root/
├── src/
│   ├── main           ← ENTRY: [constraint: stateless, <X LOC]
│   ├── boundaries/    ← All external I/O (respects: boundary limits)
│   │   ├── fs.rs      ← File system wrapper (constraint: max Y files)
│   │   └── net.rs     ← Network wrapper (constraint: timeout Zms)
│   ├── core/          ← Pure logic (constraint: no I/O, fully testable)
│   └── types/         ← Data shapes (constraint: all serializable)
```

**Annotation Key:**
- Each module lists the CONSTRAINT it respects
- Boundary modules wrap all I/O
- Core logic has ZERO I/O (fully testable, fast)

---

## 4. RISK REGISTER

<!--
  Where might we hit constraints? What needs careful testing?
-->

| Risk | Constraint at risk | Probability | Mitigation | Test strategy |
|------|-------------------|-------------|------------|--------------|
| [what could go wrong] | [which boundary] | High/Med/Low | [how we prevent it] | [how we test for it] |

---

## 5. TASK DECOMPOSITION (By Risk Level)

<!--
  Tasks grouped by risk. Do safe tasks first, risky tasks with extra care.
-->

### DONE ✓

- [x] [task description]

### IN PROGRESS →

- [ ] **[task description]**
  - **Risk level:** [Low/Medium/High]
  - **Constraints:** [which boundaries does this task touch]
  - **Files:** [exact files]
  - **Safety check:** [what to verify before marking done]

### NEXT UP (Safe Tasks)

<!--
  These tasks don't push any constraint boundaries.
  Can be done quickly and safely.
-->

- [ ] **[task description]**
  - **Constraints touched:** [list them, or "none"]
  - **Files:** [list]
  - **Approach:** [how to do it safely]

### NEXT UP (Risky Tasks)

<!--
  These tasks approach constraint boundaries.
  Need extra care, testing, monitoring.
-->

- [ ] **[task description]**
  - **⚠ Risk:** [which constraint this pushes against]
  - **Files:** [list]
  - **Approach:** [safe implementation strategy]
  - **Test plan:** [how to verify we stay within bounds]

### ICEBOX

- [ ] [idea for later]

---

## 6. VALIDATION CHECKLIST

<!--
  Before deploying, verify all constraints are respected.
-->

- [ ] All constraint boundaries documented in section 1
- [ ] Every interface in section 2 references a constraint it respects
- [ ] Risky areas identified and mitigated
- [ ] Tests exist for all constraint violations
- [ ] Monitoring in place for constraint boundaries

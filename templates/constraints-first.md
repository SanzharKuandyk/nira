<!-- Description: Start with boundaries and limits, build around constraints -->
# Constraints Blueprint: {PROJECT_NAME}

> **Started:** {DATE}
> **Last updated:** {DATE}
> **Status:** Planning
> **Approach:** Constraints-First (define boundaries, then build within them)

---

## 1. BOUNDARIES & LIMITS

### What We CAN'T Do / What's HARD

> List the constraints FIRST. These are the guardrails that guide all decisions.

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
> Be specific about external systems, I/O, side effects
- File system — [read/write/both, which directories, size limits]
- Network — [HTTP/WebSocket/etc, which endpoints, timeout requirements]
- Database — [which DB, transaction requirements, consistency needs]
- Environment — [env vars, config files, system resources]
- User input — [CLI args, interactive prompts, validation rules]

**What This System MUST NOT Touch:**
- [system resource to avoid — e.g., no registry writes, no kernel modules]
- [data to avoid — e.g., never store credentials, never log PII]

---

## 2. INTERFACE CONTRACTS (Designed Around Constraints)

> Every interface should reference which constraint it satisfies or avoids.

### Core Data Types

#### [TypeName]

**Fields:**
- `field1` (type) — *respects: [which boundary/limit does this field respect?]*
- `field2` (type) — *respects: [constraint from section 1]*

**Invariants:**
<!-- Rules that must ALWAYS be true, derived from constraints -->
- [invariant] — *respects: [constraint from section 1]*

---

### Safe Operations

> Operations designed to stay within boundaries. Mark risky operations clearly.

**[OperationName]** — [what it does]
- Constraints respected: [boundary] — [how this operation respects it]
- Signature: `[pseudo-code or actual signature]`
- Fails if: [condition that would violate constraint]
- Returns on error: [error type]
- Fallback: [safe degradation strategy]

---

### Risky Areas

> Operations that approach constraint boundaries. Need extra testing/monitoring.

**[Risky operation/module name]**
- Why risky: [which constraint does this push against]
- Mitigation: [how we stay safe — timeouts, circuit breakers, etc]
- Monitoring: [what to measure to detect violations]

---

## 3. SAFE PATHS (Implementation Strategy)

### Architecture Decisions

**Decision:** [architectural choice made]
- Reason: Respects constraints: [list which boundaries this satisfies]
- Trade-off: [what we give up to respect this constraint]

---

### Module Structure

```
project-root/
├── src/
│   ├── main           ← ENTRY: [constraint: stateless, <X LOC]
│   ├── boundaries/    ← All external I/O (respects: boundary limits)
│   │   ├── fs         ← File system wrapper (constraint: max Y files)
│   │   └── net        ← Network wrapper (constraint: timeout Zms)
│   ├── core/          ← Pure logic (constraint: no I/O, fully testable)
│   └── types/         ← Data shapes (constraint: all serializable)
```

> Each module lists the CONSTRAINT it respects.
> Boundary modules wrap all I/O. Core logic has ZERO I/O.

---

## 4. RISK REGISTER

> Where might we hit constraints? What needs careful testing?

**Risk 1:**
- What could go wrong: [describe the risk]
- Constraint at risk: [which boundary]
- Probability: High/Med/Low
- Mitigation: [how we prevent it]
- Test strategy: [how we test for it]

**Risk 2:**
- What could go wrong: [describe the risk]
- Constraint at risk: [which boundary]
- Probability: High/Med/Low
- Mitigation: [how we prevent it]
- Test strategy: [how we test for it]

---

## 5. TASK DECOMPOSITION (By Risk Level)

> Tasks grouped by risk. Do safe tasks first, risky tasks with extra care.

### DONE ✓

- [x] [task description]

### IN PROGRESS →

- [ ] **[task description]**
  - **Context:** [where you left off]
  - **Risk level:** Low/Medium/High
  - **Constraints:** [which boundaries does this task touch]
  - **Files:** [exact files]
  - **Safety check:** [what to verify before marking done]

### NEXT UP (Safe Tasks)

> These tasks don't push any constraint boundaries. Can be done quickly and safely.

- [ ] **[task description]**
  - **Depends on:** [prerequisites, or "nothing"]
  - **Constraints touched:** [list them, or "none"]
  - **Files:** [list]
  - **Approach:** [how to do it safely]

### NEXT UP (Risky Tasks)

> These tasks approach constraint boundaries. Need extra care, testing, monitoring.

- [ ] **[task description]**
  - **Depends on:** [prerequisites, or "nothing"]
  - **Risk:** [which constraint this pushes against]
  - **Files:** [list]
  - **Approach:** [safe implementation strategy]
  - **Test plan:** [how to verify we stay within bounds]

### ICEBOX

- [ ] [idea for later]

---

## 6. VALIDATION CHECKLIST

- [ ] All constraint boundaries documented in section 1
- [ ] Every interface in section 2 references a constraint it respects
- [ ] Risky areas identified and mitigated
- [ ] Tests exist for all constraint violations
- [ ] Monitoring in place for constraint boundaries

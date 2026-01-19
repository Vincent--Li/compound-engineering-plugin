# Compound Engineering: Brownfield Project Manual

> **"The goal is not to rewrite history, but to make the future easier."**

This manual outlines the systematic approach to introducing the Compound Engineering Plugin into a **Brownfield Project** (an existing codebase with legacy code, technical debt, and established patterns).

---

## 🏗 Philosophy: The Compounding Effect

In a brownfield project, every interaction is an opportunity to:
1.  **Understand** the hidden context (Archaeology)
2.  **Fix** the immediate problem (Engineering)
3.  **Document** the solution so it never needs to be "solved" again (Compounding)

We move from **linear effort** (solving the same bug twice) to **compounding knowledge** (solving a class of bugs forever).

---

## 🚀 Phase 1: Zero-Day Setup

Before changing a single line of code, establish the infrastructure for knowledge capture.

### 1.1 Initialization
Create the standard directory structure for knowledge assets. Run this in your project root:

```bash
mkdir -p docs/solutions/{build-errors,test-failures,runtime-errors,performance-issues,database-issues,security-issues,ui-bugs,patterns}
mkdir -p skills/
```

### 1.2 Configuration Check
Ensure `CLAUDE.md` exists. If not, create a basic one:
- **Build Commands**: How to run tests, linting, and build.
- **Style Guidelines**: Brief overview of coding standards.
- **Project Tracker**: Link to Jira/Linear/GitHub Issues.

---

## 🔍 Phase 2: Archaeology & Mapping (Discovery)

Don't code yet. Use AI agents to build a mental map of the territory.

### 2.1 Structural Analysis
Use the `repo-research-analyst` to understand the architecture without reading thousands of files.

```bash
# General architecture overview
claude agent repo-research-analyst "Analyze the repository structure, key architectural patterns, and technology stack."

# Understand conventions
claude agent repo-research-analyst "Identify naming conventions for Services, Controllers, and Models."
```

### 2.2 Historical Analysis
Use `git-history-analyzer` to understand *why* things are the way they are.

```bash
# Identify hotspots (files that change frequently)
claude agent git-history-analyzer "Identify the top 5 most frequently modified files in the last 6 months and their churn pattern."

# Trace a problematic module
claude agent git-history-analyzer "Analyze the evolution of the UserAuthentication module over the last year. Who are the main contributors?"
```

---

## 🔄 Phase 3: The Compounding Loop (Core Workflow)

This is your daily engine. Never let a bug fix go undocumented.

### The Loop:
1.  **Encounter Issue** → 2. **Fix Issue** → 3. **Compound Knowledge**

### How to Execute:

#### 1. Fix the Bug
Use standard tools or `/workflows:work` to fix the issue.

#### 2. Trigger Compounding
**Immediately** after verifying the fix, run:

```bash
/workflows:compound "Fixed N+1 query in DashboardController"
```

The system will:
1.  **Analyze** the conversation history to extract symptoms and root cause.
2.  **Classify** the issue (e.g., `performance-issue`).
3.  **Generate** a structured solution doc in `docs/solutions/`.
4.  **Cross-reference** related issues.

### 💡 The Payoff
Next time you (or an AI agent) plan a feature, the system searches `docs/solutions/`. It will **"remember"** that `DashboardController` is prone to N+1 queries and suggest eager loading *before* you write the bug.

---

## 🛠 Phase 4: Standardized Operations

Replace ad-hoc hacking with disciplined workflows.

### 4.1 Planning (`/workflows:plan`)
Never start coding without a plan.

```bash
/workflows:plan "Refactor the PaymentGateway adapter to support multiple currencies"
```
- **Inputs**: Feature request, bug report.
- **Activities**: Agents research repo patterns, identify risks, check existing solution docs.
- **Output**: A structured markdown plan in `plans/`.

### 4.2 Working (`/workflows:work`)
Execute the plan systematically.

```bash
/workflows:work plans/refactor-payment-gateway.md
```
- **Process**: Reads the plan, breaks it into todos, executes step-by-step, runs tests continuously.
- **Review**: Uses `rust-idiomatic-reviewer` or other reviewers when needed.

---

## 🎓 Phase 5: Institutionalization (Advanced)

As patterns emerge, solidify them into **Critical Patterns** and **Custom Skills**.

### 5.1 Critical Patterns
If a specific mistake happens 3+ times (e.g., "Forgot to close DB connection"):
1.  Run `/workflows:compound` on the latest occurrence.
2.  Select option: **"Add to Required Reading - Promote to critical patterns"**.
3.  This adds it to `cora-critical-patterns.md`. **All future agents will read this before writing code.**

### 5.2 Creating Custom Skills
Does your project have a unique logical language, complex DSL, or specific deployment ritual? Wrap it in a Skill.

```bash
# Initialize a new skill
python3 scripts/init_skill.py legacy-billing-system --path skills/
```

**Fill `skills/legacy-billing-system/SKILL.md` with:**
- **Preconditions**: "Use when modifying the `Billing` module."
- **Checklists**: Steps that *must* be followed.
- **Reference**: Pointers to key files (`references/schema.sql`).

**Effect**: now you can say:
> "Claude, use the **legacy-billing-system** skill to add a new tax rate."

---

## 📝 Cheatsheet

| Context | Action | Process |
| :--- | :--- | :--- |
| **New Feature** | `/workflows:plan` | Research → Plan → Review Plan |
| **Start Coding** | `/workflows:work` | Plan → Todo List → Code → Test |
| **Stuck?** | `claude agent repo-research-analyst` | "How is X usually done here?" |
| **Bug Fixed** | `/workflows:compound` | **Capture Knowledge!** (Non-negotiable) |
| **Complex Logic** | `claude agent pattern-recognition-specialist` | "Is this the simplest way?" |
| **New Domain** | `/create-agent-skill` | Build a skill for a specific domain |

---

## 🛑 Dos and Don'ts for Brownfield Projects

**✅ DO:**
- **Compound immediately.** Do not say "I'll document it later." You won't.
- **Trust the plan.** Spend 80% of energy on `/workflows:plan`. Coding is the easy part.
- **Respect the legacy.** Use `git-history-analyzer` to understand *why* code is weird before changing it.

**❌ DON'T:**
- **Rewrite everything.** Refactor only what you touch.
- **Ignore the "Archaeology".** Changing code without understanding its history is malpractice.
- **Create "Write-Only" Docs.** If `docs/solutions` isn't solving problems for you, you're verifying/tagging them wrong.

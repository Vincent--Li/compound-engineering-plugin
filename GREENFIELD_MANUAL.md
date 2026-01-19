# Compound Engineering: Greenfield Project Manual

> **"Start correct, stay fast. Build the rails before the train."**

This manual outlines the systematic approach to using the Compound Engineering Plugin in a **Greenfield Project** (a brand new project with no legacy code).

In a greenfield context, you have a unique superpower: **Zero Technical Debt**. Your goal is to keep it that way while moving at maximum speed.

---

## 🏗 Philosophy: The Golden Path

In a greenfield project, you are not an Archaeologist; you are an **Architect**.
1.  **Define** the standards (The Constitution).
2.  **Enforce** them automatically (The Guardrails).
3.  **Compound** knowledge from the very first commit (The Asset).

We maximize **velocity** by eliminating decision fatigue.

---

## 🚀 Phase 1: The Constitution (Day 0)

Before writing feature code, establish the "laws" of your universe.

### 1.1 The Source of Truth: `CLAUDE.md`
Create a robust `CLAUDE.md`. This is the single most important file in a new project.

```markdown
# CLAUDE.md

## Build Commands
- Test: `cargo test`
- Lint: `cargo fmt --check && cargo clippy`
- Run: `cargo run`

## Code Style
- **Strict Mode**: No warnings allowed.
- **Architecture**: Domain-Driven Design (DDD).
- **Error Handling**: No `unwrap()`, use `Result`.

## Workflow
- **Project Tracker**: Linear
```

### 1.2 Infrastructure for Knowledge
Create the empty vessels to hold your future wisdom.

```bash
# Standard knowledge structure
mkdir -p docs/solutions/{build-errors,test-failures,patterns,decisions}
mkdir -p skills/
mkdir -p plans/
```

### 1.3 The First Skill: `project-stack`
Don't wait. Create a skill that knows your stack immediately.

```bash
python3 scripts/init_skill.py project-stack --path skills/
```

**Fill `skills/project-stack/SKILL.md` with:**
- "This project uses Rust (Axum), PostgreSQL, and React (Vite)."
- "All DB changes must use `sqlx` migrations."
- "Frontend uses Tailwind CSS exclusively."

---

## ⚡ Phase 2: The Virtuous Build Loop

Speed comes from confidence. Confidence comes from structure.

### 2.1 Planning First (`/workflows:plan`)
In a new project, it's tempting to "just code". **Don't.**
The blank page is dangerous.

```bash
/workflows:plan "Setup basic User Authentication with JWT"
```
- **Why?** It forces you to define the architecture *before* implementing it.
- **Benefit**: The plan becomes the first documentation of the feature.

### 2.2 Deep Work (`/workflows:work`)
Execute the plan with strict discipline.

```bash
/workflows:work plans/feat-user-auth.md
```
- **Constraint**: Enabling `rust-idiomatic-reviewer` from the start ensures your codebase *starts* high-quality and *stays* there.

### 2.3 Visual Verification (UI)
For new UIs, screenshot everything.

```bash
# Capture the first version of the login page
agent-browser snapshot -i
/workflows:compound "Implemented Login UI (v1)"
```

---

## 💎 Phase 3: Compounding from Scratch

In a greenfield project, **Architecture Decisions** are your most valuable knowledge assets.

### 3.1 Documenting Decisions (ADRs)
When you choose a library or pattern, document it immediately using `compound-docs`.

```bash
/workflows:compound "Decision: Use 'sqlx' over 'diesel' for compile-time checking"
```
- Use the `Category Classifier` to put this in `docs/solutions/decisions/`.

### 3.2 Setting the Critical Patterns
You don't have to wait for mistakes to define patterns. Pre-seed them.

Create `docs/solutions/patterns/cora-critical-patterns.md` and add your non-negotiables:

> **Critical Pattern #1: API Responses**
> ❌ WRONG: Returning raw JSON structs.
> ✅ CORRECT: Wrap everything in `ApiResponse<T>`.

Now, every agent referencing this file will follow your API standard automatically.

---

## 🛡 Phase 4: Scaling the Team (and Agents)

As the project grows (lines of code > 1,000), structure prevents chaos.

### 4.1 Agent Specialization
Create specific Reviewer Agents for your domain.

- **Example**: `agent-security-reviewer` that checks your specific JWT implementation.

### 4.2 The "Onboarding" Skill
Create a `project-onboarding` skill.
- **Purpose**: Helps new devs (and agents) understand the project.
- **Content**: Link to `CLAUDE.md`, list of key Architecture Decision Records (ADRs).

---

## 📝 Cheatsheet: The Greenfield Routine

| Goal | Action |
| :--- | :--- |
| **Day 0 Setup** | Create `CLAUDE.md`, `mkdir -p docs/solutions` |
| **Define Stack** | `python3 scripts/init_skill.py project-stack` |
| **First Feature** | `/workflows:plan "Scaffold the application shell"` |
| **Code It** | `/workflows:work plans/feat-scaffold.md` |
| **Make a Choice** | `/workflows:compound "Decision: Use Axum for web framework"` |
| **Enforce Style** | Add pattern to `docs/solutions/patterns/cora-critical-patterns.md` |

---

## 🛑 Dos and Don'ts for Greenfield Projects

**✅ DO:**
- **Over-invest in `CLAUDE.md` early.** It's the DNA of your project.
- **Plan small features.** Break "Build the App" into 20 small plans.
- **Document "Why".** You will forget why you chose that library in 3 weeks. Document it now.

**❌ DON'T:**
- **Skip the Plan.** Coding without a plan is how Greenfield becomes Brownfield in 2 weeks.
- **Relax standards.** "I'll add linting later" = Technical Debt.
- **Copy-paste blindly.** Ensure every snippet fits your Architecture defined in `CLAUDE.md`.

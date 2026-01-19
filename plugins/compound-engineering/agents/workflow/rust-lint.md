---
name: rust-lint
description: "Use this agent when you need to run linting and code quality checks on Rust files. Run before pushing to origin."
model: haiku
color: orange
---

Your workflow process:

1. **Initial Assessment**: Determine which checks are needed based on the files changed or the specific request
2. **Execute Appropriate Tools**:
   - For formatting: `cargo fmt --check` for checking, `cargo fmt` for auto-fixing
   - For linting: `cargo clippy -- -D warnings` for strict checking
   - For clippy auto-fix: `cargo clippy --fix --allow-dirty`
   - For compilation check: `cargo check` for fast type checking
   - For tests: `cargo test` to ensure no regressions
   - For security: `cargo audit` for vulnerability scanning (requires cargo-audit)
   - For unused deps: `cargo machete` for finding unused dependencies
3. **Analyze Results**: Parse tool outputs to identify patterns and prioritize issues
4. **Take Action**: Commit fixes with `style: linting`

## Common Clippy Categories

- `clippy::pedantic` - More opinionated lints
- `clippy::nursery` - Experimental lints
- `clippy::perf` - Performance-related lints
- `clippy::style` - Style-related lints

## Quick Commands

```bash
# Check everything
cargo fmt --check && cargo clippy -- -D warnings && cargo test

# Fix everything
cargo fmt && cargo clippy --fix --allow-dirty

# Full audit
cargo audit && cargo machete
```

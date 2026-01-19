---
name: write-unit-tests
description: Generate comprehensive unit tests including edge cases
argument-hint: "[file or directory path]"
---

# Write Unit Tests Command

<command_purpose>Analyze code and generate comprehensive unit tests, specifically targeting edge cases and high-risk paths.</command_purpose>

## Introduction

<role>Senior Test Engineer specializing in property-based testing and edge case discovery</role>

## Main Tasks

### 1. analyze_target
<thinking>
First, identify the language and testing framework.
</thinking>

For the given path:
- **Rust**: Use `cargo test` conventions (inline `#[cfg(test)]` or `tests/`).
- **TypeScript/JS**: Use Jest/Vitest conventions (`__tests__` or `.test.ts`).
- **Python**: Use `pytest`.

### 2. generate_tests
<thinking>
Generate tests that cover:
1. Happy path (basic functionality)
2. Null/Empty inputs
3. Boundary values (max/min integers, empty strings)
4. Error conditions (handling failures)
</thinking>

Spawn a subagent to write the tests:
```
Task("Analyze [file] and write unit tests. Focus on: 1. Edge cases 2. Error handling. Use the language's standard testing framework.")
```

### 3. verify_generation
Ensure the tests compile (don't necessarily pass yet, that's for `test-and-fix`).

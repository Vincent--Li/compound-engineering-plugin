---
name: test-and-fix
description: Run tests and iteratively fix failures until passing
argument-hint: "[optional: specific test name or path]"
---

# Test and Fix Command

<command_purpose>Run test suites and autonomously fix failures until all tests pass.</command_purpose>

## Introduction

<role>TDD Specialist and Debugging Expert</role>

## Main Tasks

### 1. run_tests_initial
Execute the test suite:
- **Rust**: `cargo test`
- **Node**: `npm test`

If all pass, exit with success.

### 2. fix_loop
While tests are failing (max 5 iterations):

1. **Analyze Output**: Read the failure logs.
2. **Identify Cause**: Is it a logic error, assertion error, or compilation error?
3. **Apply Fix**:
   - If **Compilation Error**: Fix syntax/types.
   - If **Assertion Error**: Check if the test is wrong or the code is wrong. Fix the *code* unless the test expectation is provably incorrect.
4. **Re-run**: Run tests again.

### 3. report
Summary of fixed tests and any remaining failures.

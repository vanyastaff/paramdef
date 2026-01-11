---
name: rust-review
description: Detailed code review focusing on Rust best practices
user-invocable: true
allowed-tools: LSP, Read, Write, Edit, Bash, Grep, Glob, AskUserQuestion
---

# Code Review - Rust Best Practices

Perform a comprehensive code review of the Rust code **using LSP diagnostics and AskUserQuestion tool**.

**IMPORTANT: Use LSP diagnostics first, then AskUserQuestion to understand intent before making suggestions.**

## Review Process with LSP

### Step 1: LSP Diagnostics Check

Before reviewing code, check current diagnostics:

```
For each file being reviewed:
  LSP getDiagnostics: file_path
  -> Identifies errors, warnings, hints
  -> Provides compiler-level feedback
```

**Benefits**:
- Catch compilation errors immediately
- Find warnings before manual review
- Identify clippy lints automatically
- Validate code compiles

### Step 2: Understand Context

Use AskUserQuestion to clarify intent and design decisions.

### Step 3: LSP-Enhanced Analysis

Use LSP tools for deep code understanding:

```
# Type checking
LSP getHover: variable/function
-> See inferred types, trait bounds

# Find usages
LSP findReferences: function/type
-> Understand how code is used

# Navigate dependencies
LSP goToDefinition: type/trait
-> Check implementations

# Get symbol overview
LSP getDocumentSymbols: file
-> See file structure
```

### Step 4: Manual Review

Combine LSP insights with manual review.

## Technical Aspects

### Idiomatic Rust (Use LSP + Manual)
- Idiomatic Rust patterns
- Proper use of Option/Result
- Iterator usage vs loops
- Clone vs references (check with LSP getHover)
- Unsafe code justification
- Panic vs Result
- Lifetime annotations necessity (LSP shows inferred lifetimes)
- Trait bounds optimization (LSP getHover shows bounds)

### Code Quality (LSP getDiagnostics helps)
- Error messages clarity
- Documentation completeness (LSP warns if missing)
- Test coverage
- Performance considerations (LSP hints for unnecessary clones)
- Memory efficiency
- Code duplication (LSP findReferences helps identify)

## Context Questions

For each finding, use AskUserQuestion to ask:
- What's the intent behind this code?
- Are you aware of [idiomatic pattern]?
- Have you benchmarked this approach?
- What's the error handling strategy here?

## Feedback Approach

Be constructive and educational:
- Explain the "why" behind suggestions
- Provide concrete code examples
- Reference Rust guidelines when relevant
- Prioritize feedback (critical vs nice-to-have)

**Continue using AskUserQuestion until you understand all design decisions.**

## Review Document Structure

Output a review document with:
1. Summary of findings
2. Critical issues (with fixes)
3. Improvement suggestions
4. Learning opportunities
5. Positive patterns observed

## paramdef-Specific Checks

When reviewing paramdef code, use LSP extensively:

### Naming Conventions
- Follows conventions (Text, not TextParameter)
- **LSP**: findReferences to check consistency

### Memory Efficiency
- Uses SmartString appropriately
- Arc usage for immutable shared data
- **LSP**: getHover on variables to check types
- **LSP**: getDiagnostics for unnecessary clones

### Architecture
- Proper feature gating
- **LSP**: Check #[cfg] attributes with getDiagnostics
- Three-layer architecture adherence (Schema/Runtime/Value)
- **LSP**: goToDefinition to verify layer separation

### Documentation
- Documentation on public APIs
- **LSP**: getDiagnostics warns about missing docs
- **LSP**: getHover to verify doc comments render correctly

### Error Handling
- thiserror for error types
- **LSP**: findReferences on Error types
- **LSP**: Check error propagation with goToDefinition

### Code Quality
- No wildcard imports (Clippy enforced)
- **LSP**: getDiagnostics catches this automatically

## LSP-Enhanced Review Workflow

### Example: Reviewing a New Node Type

```
Step 1: Check diagnostics
  LSP getDiagnostics: src/types/newtype.rs
  -> Should have 0 errors

Step 2: Verify trait implementations
  LSP getDocumentSymbols: src/types/newtype.rs
  -> Check Node trait is implemented

  LSP findReferences: "Node"
  -> Verify newtype appears in results

Step 3: Check type correctness
  LSP getHover: on Node impl
  -> Verify category trait is correct (Leaf/Container/etc.)

Step 4: Validate usage
  LSP findReferences: newtype name
  -> See where it's used, should have tests

Step 5: Manual review
  Read implementation details
  Check for paramdef-specific patterns
```

### Example: Reviewing Validation Logic

```
Step 1: Diagnostics
  LSP getDiagnostics: src/validation/validators/custom.rs

Step 2: Type checking
  LSP getHover: on validate method
  -> Verify signature matches Validator trait

Step 3: Find trait definition
  LSP goToDefinition: Validator
  -> Confirm correct trait version

Step 4: Check all validators
  LSP findReferences: Validator
  -> See similar patterns for consistency

Step 5: Test coverage
  LSP getWorkspaceSymbols: "test_custom_validator"
  -> Verify tests exist
```

## Review Checklist with LSP

Before approving code:

- [ ] `LSP getDiagnostics` on all modified files = 0 errors
- [ ] `LSP getDiagnostics` warnings addressed or justified
- [ ] `LSP findReferences` shows expected usage
- [ ] `LSP getHover` confirms correct types
- [ ] `LSP goToDefinition` navigates correctly
- [ ] Manual review completed
- [ ] `cargo check --workspace --all-targets` passes
- [ ] `cargo clippy --workspace --all-features -- -D warnings` passes
- [ ] `cargo test --workspace --all-features` passes

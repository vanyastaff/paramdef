---
name: lsp-diagnostics
description: Analyze and fix code issues using LSP diagnostics. Use for debugging compiler errors, fixing warnings, and improving code quality.
user-invocable: true
allowed-tools: LSP, Read, Edit, Bash
---

# LSP Diagnostics & Error Resolution

**Use Language Server Protocol to understand and fix code issues in real-time.**

## Why LSP Diagnostics?

- **Real-time feedback**: Faster than running cargo check
- **Rich context**: Shows type mismatches, missing traits, etc.
- **IDE-quality**: Same diagnostics as VSCode/IntelliJ
- **Actionable**: Often includes quick-fixes
- **Proactive**: Catch issues before compilation

## LSP Diagnostics Operation

### Get Diagnostics

```
Use LSP tool:
  operation: getDiagnostics
  params: {
    file_path: "src/types/text.rs"
  }
```

**Returns**:
- **Errors**: Code won't compile
- **Warnings**: Code compiles but problematic
- **Hints**: Suggestions for improvement
- **Information**: General notices

Each diagnostic includes:
- Severity (Error/Warning/Info/Hint)
- Message describing the issue
- Source (rust-analyzer, clippy, rustc)
- Line and character range
- Optional related information
- Sometimes: suggested fixes

## Diagnostic Workflow

### Step 1: Check Current State

Before making changes:

```
LSP getDiagnostics: file_being_modified.rs
-> See baseline errors/warnings
```

**Why?**
- Avoid introducing new issues
- Understand existing problems
- Prioritize fixes

### Step 2: Make Changes

Use Edit tool to modify code.

### Step 3: Re-check Diagnostics

After changes:

```
LSP getDiagnostics: file_being_modified.rs
-> See new errors/warnings
```

**Compare**:
- New diagnostics added?
- Existing diagnostics fixed?
- Overall count improved?

### Step 4: Fix Issues

Based on diagnostics:
- Read error messages carefully
- Use LSP getHover for type info
- Use LSP goToDefinition for context
- Apply fixes with Edit tool

### Step 5: Validate

```
LSP getDiagnostics: file.rs
-> Should show no errors

cargo check --all-targets
-> Confirm compilation
```

## Diagnostic Types

### Errors (Must Fix)

**Type Mismatch**:
```
error[E0308]: mismatched types
  expected `String`, found `&str`
```

**Resolution**:
- LSP getHover to see actual type
- Convert: `.to_string()`, `.into()`, etc.
- Or change function signature

**Trait Not Implemented**:
```
error[E0277]: the trait bound `Text: Clone` is not satisfied
```

**Resolution**:
- Add `#[derive(Clone)]`
- Or implement trait manually
- Or remove requirement

**Missing Field**:
```
error[E0063]: missing field `value` in initializer
```

**Resolution**:
- Add missing field
- Or use builder pattern
- Or use Default trait

**Cannot Find**:
```
error[E0425]: cannot find function `validate` in this scope
```

**Resolution**:
- LSP getWorkspaceSymbols to find correct name
- Add import
- Check spelling

### Warnings (Should Fix)

**Unused Variable**:
```
warning: unused variable: `data`
```

**Resolution**:
- Use the variable
- Prefix with `_data` if intentional
- Remove if unnecessary

**Unused Import**:
```
warning: unused import: `HashMap`
```

**Resolution**:
- Remove import
- Or use the imported item

**Dead Code**:
```
warning: function `helper` is never used
```

**Resolution**:
- Remove unused code
- Make pub if should be exported
- Add #[cfg(test)] if test helper

### Hints (Nice to Have)

**Unnecessary Parentheses**:
```
hint: unnecessary parentheses around `if` condition
```

**Can Use Method**:
```
hint: this `.into_iter()` call is unnecessary
```

**Type Complexity**:
```
hint: this type is very complex, consider using a type alias
```

## paramdef-Specific Diagnostics

### Common Errors in paramdef

**1. Missing Feature Gate**:
```
error[E0433]: failed to resolve: use of undeclared crate or module `validation`
```

**Resolution**:
- Check Cargo.toml features
- Add to dependencies: `features = ["validation"]`
- Or gate code with `#[cfg(feature = "validation")]`

**2. Trait Bound Issues**:
```
error[E0277]: the trait bound `MyType: Node` is not satisfied
```

**Resolution**:
- Implement Node trait
- Check trait requirements (category traits)
- Verify type is in correct module

**3. Lifetime Errors**:
```
error[E0597]: borrowed value does not live long enough
```

**Resolution**:
- Use Arc for shared ownership
- Clone if needed
- Adjust function signatures

**4. Validation Type Mismatches**:
```
error[E0308]: mismatched types in validation rule
  expected `Value`, found `String`
```

**Resolution**:
- Wrap in Value::text()
- Check validator input types
- Use proper Value variant

### Common Warnings in paramdef

**1. Unused Subtypes**:
```
warning: variants are never constructed: `TextSubtype::Url`
```

**Resolution**:
- Expected if subtype not used yet
- Add test showing usage
- Or remove if truly not needed

**2. Missing Documentation**:
```
warning: missing documentation for public struct
```

**Resolution**:
- Add /// doc comment
- Required by clippy.toml configuration
- Document purpose and usage

**3. Complexity Warnings**:
```
warning: this function has too many arguments (8/7)
```

**Resolution**:
- Use builder pattern
- Group parameters in struct
- Refactor if truly complex

## Diagnostic Patterns

### Before Feature Implementation

```
# Check clean state
LSP getDiagnostics on all files in module
-> Should be zero errors

# Implement feature
Use Edit/Write tools

# Check after each file
LSP getDiagnostics
-> Fix errors immediately

# Prevents error accumulation
```

### During Refactoring

```
# Baseline
LSP getDiagnostics: file.rs
-> Note count: 0 errors, 2 warnings

# Make changes
Edit: file.rs

# Check impact
LSP getDiagnostics: file.rs
-> New count: 3 errors, 1 warning

# Fix new errors
Edit: file.rs based on diagnostics

# Verify improvement
LSP getDiagnostics: file.rs
-> Final: 0 errors, 1 warning (better than baseline)
```

### Cleaning Up Code

```
# Get all warnings
LSP getDiagnostics: src/types/text.rs

# Group by type
- 5x unused imports
- 3x dead code
- 2x missing docs

# Fix systematically
1. Remove unused imports
2. Remove dead code or make public
3. Add documentation

# Verify clean
LSP getDiagnostics
-> 0 warnings
```

### Type Debugging

```
Problem: Type error but unclear why

Step 1: Get diagnostics
  LSP getDiagnostics
  -> "expected X, found Y"

Step 2: Check actual type
  LSP getHover on problematic expression
  -> See inferred type Y

Step 3: Check expected type
  LSP goToDefinition on function
  LSP getHover on parameter
  -> See expected type X

Step 4: Bridge the gap
  Convert Y to X or change signature
```

## Integration with cargo Commands

### Compare LSP vs Cargo

```
# LSP (fast, file-level)
LSP getDiagnostics: src/types/text.rs
-> Instant results for one file

# Cargo (comprehensive, project-level)
cargo check --workspace --all-targets
-> All files, all targets, slower

# Clippy (linting)
cargo clippy --all-features -- -D warnings
-> Additional style checks

# Use LSP during development
# Run cargo before commits
```

### Diagnostic Hierarchy

```
Level 1: LSP getDiagnostics (fastest)
-> Quick feedback during editing

Level 2: cargo check (thorough)
-> Verify compilation across workspace

Level 3: cargo clippy (strict)
-> Enforce style and best practices

Level 4: cargo test (validation)
-> Ensure correctness
```

## Advanced Usage

### Batch Diagnostic Check

```
# Check all files in module
For each file in src/types/:
  LSP getDiagnostics: file
  If errors > 0:
    Log file for attention

# Prioritize fixes
- Files with errors first
- Then warnings
- Finally hints
```

### Diagnostic-Driven Development

```
# Write skeleton implementation
Edit: Add function signature, empty body

# Check what's missing
LSP getDiagnostics
-> Shows missing return value
-> Shows unused parameters

# Fill in based on diagnostics
Edit: Implement based on errors

# Iterate until clean
LSP getDiagnostics
-> 0 errors
```

### Pre-commit Validation

```
# Before committing
For each modified file:
  LSP getDiagnostics: file
  Assert: 0 errors
  Assert: warnings <= baseline

cargo check --workspace
cargo clippy --workspace -- -D warnings
cargo test --workspace

# Only commit if all pass
```

## Troubleshooting

### Diagnostics Not Updating

```
Cause: rust-analyzer hasn't re-analyzed

Solution:
1. Wait 1-2 seconds after Edit
2. LSP restartServer if stuck
3. Check rust-analyzer logs
```

### False Positives

```
LSP shows error but cargo check passes

Reasons:
- Conditional compilation (#[cfg])
- Macro expansion issues
- rust-analyzer bug

Action:
- Trust cargo check over LSP
- Report to rust-analyzer if reproducible
- Use cargo as source of truth
```

### Too Many Warnings

```
Hundreds of warnings in legacy code

Strategy:
1. Focus on errors first
2. Use #[allow] temporarily
3. Fix warnings incrementally
4. Set RUSTFLAGS=-Awarnings for old code
```

## Best Practices

1. **Check diagnostics before committing**
2. **Fix errors immediately, not later**
3. **Zero warnings policy for new code**
4. **Use LSP during development, cargo before commit**
5. **Read error messages carefully - they're helpful**
6. **Use LSP getHover to understand type errors**
7. **Batch similar fixes for efficiency**

## Example: Fixing a Type Error

```
Scenario: Adding validation to Text node

Step 1: Implement trait
  Edit: impl Validatable for Text { ... }

Step 2: Check diagnostics
  LSP getDiagnostics: src/types/text.rs
  -> Error: method `validate` has wrong signature

Step 3: Check expected signature
  LSP goToDefinition: Validatable trait
  LSP getHover: validate method
  -> fn validate(&self, value: &Value, ctx: &ValidationContext) -> Result<()>

Step 4: Fix signature
  Edit: Update method signature to match

Step 5: Implement body
  Edit: Add validation logic

Step 6: Check again
  LSP getDiagnostics: src/types/text.rs
  -> Error: ValidationContext not in scope

Step 7: Add import
  Edit: use crate::validation::ValidationContext;

Step 8: Final check
  LSP getDiagnostics: src/types/text.rs
  -> 0 errors, 0 warnings ✓

Step 9: Validate
  cargo test -p paramdef --all-features
  -> All tests pass ✓
```

## Diagnostic Sources

**rust-analyzer**:
- Type checking
- Borrow checking
- Trait resolution
- Quick fixes

**rustc**:
- Compilation errors
- Precise diagnostics
- Comes from actual compiler

**clippy**:
- Lints and style
- Performance hints
- Idiomatic suggestions

**cargo**:
- Dependency issues
- Feature resolution
- Build problems

## Integration with Skills

Essential for:
- `/lsp-refactor` - Validate refactorings
- `/rust-review` - Check code quality
- `/rust-tdd` - Fix failing tests
- `/rust-patterns` - Ensure correct patterns

**LSP diagnostics = continuous code quality feedback!** ⚠️✅

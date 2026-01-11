---
name: lsp-refactor
description: Safe refactoring using LSP for Rust. Use for renaming symbols, finding references, and ensuring refactorings don't break code.
user-invocable: true
allowed-tools: LSP, Read, Edit, Bash, Grep, Glob
---

# LSP-Powered Safe Refactoring

**Use Language Server Protocol for type-safe, project-wide refactoring.**

## Why LSP for Refactoring?

- **Type-aware**: Understands Rust semantics, not just text
- **Project-wide**: Finds all references across workspace
- **Safe**: Validates changes before applying
- **Fast**: Faster than grep/search for large codebases

## LSP Operations for Refactoring

### 1. Find All References

Before any refactoring, understand the scope:

```
Use LSP tool:
  operation: findReferences
  params: {
    file_path: "src/types/text.rs",
    position: { line: 42, character: 10 }
  }
```

**Returns**: All usages of the symbol across the project.

### 2. Go to Definition

Understand what you're refactoring:

```
Use LSP tool:
  operation: goToDefinition
  params: {
    file_path: "src/runtime/context.rs",
    position: { line: 100, character: 20 }
  }
```

**Returns**: Exact location of definition.

### 3. Get Hover Info

Check types and documentation:

```
Use LSP tool:
  operation: getHover
  params: {
    file_path: "src/core/value.rs",
    position: { line: 50, character: 15 }
  }
```

**Returns**: Type signature, documentation, inferred types.

### 4. Rename Symbol (Safest)

Project-wide rename with validation:

```
Use LSP tool:
  operation: renameSymbol
  params: {
    file_path: "src/types/number.rs",
    position: { line: 30, character: 12 },
    new_name: "NumericParameter"
  }
```

**Returns**: All edits needed across the project.

**IMPORTANT**: LSP renameSymbol is safer than Edit tool for refactoring!

### 5. Get Diagnostics

Validate before and after refactoring:

```
Use LSP tool:
  operation: getDiagnostics
  params: {
    file_path: "src/types/text.rs"
  }
```

**Returns**: Errors, warnings, hints from rust-analyzer.

## Safe Refactoring Workflow

### Step 1: Analyze Impact

```
1. Use LSP goToDefinition to locate the symbol
2. Use LSP findReferences to see all usages
3. Use LSP getHover to understand types
4. Plan the refactoring strategy
```

### Step 2: Check Current State

```
1. Use LSP getDiagnostics on relevant files
2. Ensure no pre-existing errors
3. Run tests: cargo test
```

### Step 3: Perform Refactoring

**Option A: LSP Rename (Recommended)**

```
Use LSP renameSymbol for:
- Renaming structs, enums, traits
- Renaming functions, methods
- Renaming fields, variants
- Renaming modules

Advantages:
- Validates rename is safe
- Updates all references atomically
- Respects visibility rules
- Handles imports automatically
```

**Option B: Manual Edit (For Complex Changes)**

```
When LSP rename isn't enough:
1. Use LSP findReferences to get all locations
2. Use Edit tool on each file
3. Use LSP getDiagnostics after each edit
4. Fix any errors immediately
```

### Step 4: Validate Changes

```
1. Use LSP getDiagnostics on all modified files
2. Check for new errors or warnings
3. Run: cargo check --all-targets
4. Run: cargo test --all-features
5. Run: cargo clippy -- -D warnings
```

### Step 5: Verify Behavior

```
1. Review the changes with git diff
2. Ensure semantics haven't changed
3. Run full test suite
4. Check documentation is updated
```

## Common Refactoring Patterns

### Rename a Type

```
# For paramdef: Renaming a node type
1. LSP findReferences on struct name
2. Check trait implementations
3. LSP renameSymbol with new name
4. LSP getDiagnostics to verify
5. Update documentation
```

### Extract Method

```
# Manual process with LSP validation
1. Identify code block to extract
2. LSP getHover on variables to understand types
3. Create new function with Edit
4. LSP getDiagnostics to check signatures
5. Replace call sites with Edit
6. Verify with LSP getDiagnostics
```

### Move Module

```
# Reorganizing code structure
1. LSP findReferences on module
2. Create new location with Edit/Write
3. Update all imports (may need manual Edit)
4. LSP getDiagnostics on all affected files
5. Remove old file
6. Verify with cargo check
```

### Change Function Signature

```
# Adding/removing parameters
1. LSP findReferences to find all call sites
2. Use LSP getSignatureHelp for current signature
3. Update definition with Edit
4. LSP getDiagnostics shows all broken call sites
5. Fix each call site using LSP errors as guide
6. Verify with cargo check
```

### Trait Refactoring

```
# For paramdef: Modifying traits like Node, Validatable
1. LSP findReferences on trait name
2. See all implementations
3. Update trait definition
4. LSP getDiagnostics shows affected impls
5. Update each impl based on errors
6. Verify with cargo check --all-targets
```

## LSP-Specific Tips for Rust

### Use rust-analyzer Features

```
# Ensure rust-analyzer is installed
rustup component add rust-analyzer

# Features available:
- Inlay hints for types
- Expand macro (getHover on macro usage)
- Show syntax tree
- View HIR (High-level IR)
```

### Handle Large Refactorings

```
For changes affecting many files:
1. Use LSP findReferences to count impact
2. If > 20 files, consider phased approach
3. Group related changes together
4. Use LSP getDiagnostics after each phase
5. Commit intermediate working states
```

### Work with Traits

```
# Finding trait implementations
1. LSP goToDefinition on trait name
2. LSP findReferences shows all impls
3. Use grep for additional context if needed

# For paramdef trait hierarchy:
- Node -> all 23 node types
- Validatable -> nodes with validation
- ValueAccess -> container types
```

### Macro-Heavy Code

```
# LSP works through macros in Rust
1. LSP getHover on macro expansion shows result
2. LSP goToDefinition works inside macros
3. For paramdef: define_number_subtype! macro usage
```

## Verification Commands

After LSP refactoring:

```bash
# Quick check
cargo check --workspace --all-targets

# Full validation
cargo test --workspace --all-features
cargo clippy --workspace --all-features -- -D warnings
cargo fmt --all -- --check

# Documentation
cargo doc --no-deps --all-features

# MSRV check (paramdef uses 1.85)
cargo +1.85 check --workspace
```

## Example: Refactoring in paramdef

### Scenario: Rename TextSubtype variant

```
Goal: Rename TextSubtype::Secret to TextSubtype::Password

Step 1: Find all references
  LSP findReferences on "Secret" in subtype/text.rs

Step 2: Check current diagnostics
  LSP getDiagnostics on subtype/text.rs

Step 3: Rename with LSP
  LSP renameSymbol: Secret -> Password

Step 4: Verify changes
  LSP getDiagnostics on modified files
  Check no new errors

Step 5: Update tests
  LSP findReferences on "secret" (string literals)
  Manual update test cases

Step 6: Validate
  cargo test -p paramdef --all-features
  cargo clippy -p paramdef -- -D warnings
```

## When NOT to Use LSP

LSP is powerful but has limits:

- **String literals**: LSP won't find "TextSubtype::Secret" in strings
- **Comments**: Won't update documentation automatically
- **File names**: Doesn't rename files (use Edit/Bash)
- **Cross-language**: Rust LSP doesn't see FFI usage in C

For these, combine LSP with grep/Edit.

## Troubleshooting

### LSP Server Not Responding

```
Use LSP tool:
  operation: restartServer
```

### Diagnostics Out of Date

```
1. Save all files
2. Wait 1-2 seconds for rust-analyzer
3. Re-run LSP getDiagnostics
```

### False Positives

```
Sometimes rust-analyzer shows errors that compile fine:
1. Run cargo check to verify
2. If cargo check passes, trust that over LSP
3. Report bugs to rust-analyzer if reproducible
```

## Best Practices

1. **Always check diagnostics before refactoring**
2. **Use LSP renameSymbol over manual edits when possible**
3. **Verify with cargo check after LSP changes**
4. **Commit working state before large refactorings**
5. **Use LSP findReferences to estimate impact**
6. **Combine LSP with grep for string literals**
7. **Trust rust-analyzer for Rust semantics**

## Integration with Other Skills

Combine with:
- `/rust-review` - Use LSP diagnostics in reviews
- `/rust-patterns` - Refactor to idiomatic patterns
- `/rust-arch-review` - Restructure with LSP guidance

**LSP makes refactoring safe, fast, and reliable!** 🚀

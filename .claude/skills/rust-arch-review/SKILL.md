---
name: rust-arch-review
description: Deep architecture review for Rust projects
user-invocable: true
allowed-tools: LSP, Read, Write, Edit, Bash, Grep, Glob, AskUserQuestion
---

# Rust Architecture Review

Analyze the Rust codebase and conduct an in-depth architecture review **using LSP for code intelligence and AskUserQuestion tool**.

**IMPORTANT: Use LSP to explore architecture, then AskUserQuestion for design rationale.**

## LSP-Powered Architecture Analysis

### Step 1: Discover Project Structure

```
# Find all public modules
LSP getWorkspaceSymbols: "mod"

# Get overview of key files
For each major module:
  LSP getDocumentSymbols: module_file
  -> See structs, traits, impls
```

### Step 2: Map Trait Hierarchies

```
# Find root traits
LSP getWorkspaceSymbols: "trait"

# For each key trait:
  LSP goToDefinition: trait_name
  LSP findReferences: trait_name
  -> See all implementations

# Understand relationships
  LSP getHover: on impl blocks
  -> See trait bounds, where clauses
```

### Step 3: Analyze Type Relationships

```
# For key types:
  LSP getHover: type_name
  -> See fields, generic parameters

  LSP findReferences: type_name
  -> Understand usage patterns

  LSP goToDefinition: field types
  -> Trace dependencies
```

## Review Aspects

### Trait Design and Hierarchy (Use LSP)
- Trait design and hierarchy
  - **LSP**: getWorkspaceSymbols to find all traits
  - **LSP**: findReferences to see implementations
  - **LSP**: getHover to understand trait bounds
- Ownership and lifetime patterns
  - **LSP**: getHover shows lifetime annotations
  - **LSP**: getDiagnostics catches lifetime errors
- Error handling strategy (Result/Option usage)
  - **LSP**: findReferences on Result/Error types
- Module organization and visibility
  - **LSP**: getDocumentSymbols for structure
  - **LSP**: goToDefinition to navigate modules
- Type safety and zero-cost abstractions
  - **LSP**: getHover for type checking
- Async patterns if applicable
  - **LSP**: findReferences on async/await
- Performance characteristics
  - **LSP**: getDiagnostics for efficiency hints
- Memory allocation patterns
  - **LSP**: findReferences on Box/Arc/Rc

## Probing Questions

For each aspect, ask probing questions using AskUserQuestion:
- What's the rationale for this design?
- Have you considered [alternative approach]?
- How does this scale with [specific scenario]?
- What are the performance implications?
- Are there any lifetime issues we should address?

Be specific about:
- Concrete code locations
- Specific trait implementations
- Potential refactoring opportunities
- Performance bottlenecks

**Continue asking questions using AskUserQuestion until the architecture is fully understood.**

## Review Report Structure

After review, generate a detailed report with:
1. Architecture summary
2. Identified patterns (good and concerning)
3. Specific recommendations with code examples
4. Priority-ranked action items

## Focus Areas for paramdef

When reviewing paramdef specifically, use LSP extensively:

### Three-Layer Architecture Analysis

```
# Schema Layer (Immutable)
LSP getWorkspaceSymbols: "Schema"
LSP findReferences: "Schema"
-> Verify shared via Arc, no mutations

# Runtime Layer (Mutable)
LSP getWorkspaceSymbols: "Context"
LSP getHover: on Context fields
-> Check state management

# Value Layer
LSP findReferences: "Value"
LSP getDocumentSymbols: src/core/value.rs
-> See all variants
```

### Node Trait Hierarchy Exploration

```
# Find base trait
LSP goToDefinition: "Node"
LSP getDocumentSymbols: src/types/node.rs

# Find all implementations (23 types)
LSP findReferences: "Node"
-> See all node types

# Category traits
LSP getWorkspaceSymbols: "Leaf"
LSP getWorkspaceSymbols: "Container"
LSP getWorkspaceSymbols: "Decoration"
-> Map category hierarchy

# For each node type:
LSP getHover: on impl blocks
-> Verify category trait + Node trait
```

### Subtype vs Unit Pattern

```
# Find subtype enums
LSP getWorkspaceSymbols: "Subtype"
-> TextSubtype, NumberSubtype, VectorSubtype

# Check usage
LSP findReferences: "NumberSubtype::Distance"
-> See how subtypes are used

# Compare with units
LSP findReferences: "NumberUnit"
-> Understand separation
```

### Feature Flag Architecture

```
# Find feature-gated code
LSP getDiagnostics on all files
-> Shows #[cfg] issues if any

# Check validation feature
LSP getWorkspaceSymbols: "Validator"
-> Should find types only if validation enabled

# Check visibility feature
LSP getWorkspaceSymbols: "Visibility"
-> Feature-specific traits
```

### Event System Design

```
# Explore EventBus
LSP goToDefinition: "EventBus"
LSP getDocumentSymbols: src/event/bus.rs

# Find event types
LSP getHover: on Event enum
LSP getDocumentSymbols: src/event/event.rs

# Trace event flow
LSP findReferences: "Event::ValueChanged"
-> See emission and handling
```

### Memory Efficiency Patterns

```
# Arc usage
LSP findReferences: "Arc<"
-> Should see Schema, immutable data

# SmartString usage
LSP findReferences: "SmartString"
-> Check for short string optimization

# Check allocations
LSP findReferences: "Box"
LSP findReferences: "Vec"
-> Understand allocation patterns
```

## LSP-Based Architecture Review Workflow

### Example: Reviewing Node Type System

```
Step 1: Map the hierarchy
  LSP getWorkspaceSymbols: "trait Node"
  -> Find base Node trait

Step 2: Find all node types
  LSP findReferences: "Node"
  Filter to "impl Node for"
  -> Should see 23 implementations

Step 3: Analyze categories
  For each category (Leaf, Container, Decoration, Group):
    LSP getWorkspaceSymbols: category_name
    LSP findReferences: category_trait
    -> Count implementations per category

Step 4: Check trait design
  LSP goToDefinition: Node trait
  Read trait definition
  LSP getHover: on associated types
  -> Understand trait requirements

Step 5: Verify implementations
  Pick sample from each category:
    LSP goToDefinition: "impl Node for Text"
    Check implementation completeness
    LSP getDiagnostics: file
    -> Should be error-free

Step 6: Ask design questions
  AskUserQuestion:
    - Why 23 types vs extensibility?
    - Why category traits?
    - Why no own value for Group?
```

### Example: Analyzing Validation Pipeline

```
Step 1: Find entry points
  LSP getWorkspaceSymbols: "validate"
  -> Find Validatable trait, validate methods

Step 2: Trace validation flow
  LSP goToDefinition: Validatable
  Read trait definition

  LSP findReferences: Validatable
  -> See which types implement it

Step 3: Explore validators
  LSP getWorkspaceSymbols: "Validator"
  LSP findReferences: "Validator"
  -> Map all validator implementations

Step 4: Check Rule system
  LSP goToDefinition: "Rule"
  LSP getHover: on Rule fields
  -> Understand declarative validation

Step 5: Trace execution
  LSP findReferences: "validate_all"
  -> See how validation is orchestrated

Step 6: Performance analysis
  LSP findReferences: "thread_local"
  -> Check for caching (regex cache)
```

## Architecture Documentation Template

After LSP exploration, generate architecture document:

1. **Architecture Overview** (from LSP getWorkspaceSymbols)
   - Major modules and their responsibilities
   - Trait hierarchies discovered

2. **Core Abstractions** (from LSP analysis)
   - Key traits and their implementations count
   - Type relationships (from goToDefinition/findReferences)

3. **Design Patterns** (identified via LSP)
   - Builder patterns (findReferences on "builder")
   - Arc patterns (memory sharing)
   - Feature gating strategy

4. **Data Flow** (traced with LSP)
   - Schema → Runtime → Value flow
   - Event propagation paths

5. **Performance Characteristics** (from LSP + manual)
   - Allocation patterns
   - Zero-cost abstractions usage

6. **Extension Points** (found via LSP)
   - Trait implementations users can provide
   - Feature flags for opt-in functionality

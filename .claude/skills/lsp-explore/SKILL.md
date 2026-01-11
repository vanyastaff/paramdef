---
name: lsp-explore
description: Navigate and explore Rust codebase using LSP. Use for understanding trait hierarchies, finding implementations, and code archaeology.
user-invocable: true
allowed-tools: LSP, Read, Grep, Glob
---

# LSP-Powered Code Exploration

**Use Language Server Protocol to understand code structure and relationships.**

## Why LSP for Exploration?

- **Semantic awareness**: Understands Rust types, not just text patterns
- **Jump to definition**: Instant navigation to symbol definitions
- **Find implementations**: See all trait impls, method overrides
- **Workspace-wide search**: Fast symbol lookup across entire project
- **Type information**: Inferred types, documentation, signatures

## LSP Exploration Operations

### 1. Go to Definition

Navigate from usage to definition:

```
Use LSP tool:
  operation: goToDefinition
  params: {
    file_path: "src/runtime/context.rs",
    position: { line: 45, character: 18 }
  }
```

**Use cases**:
- "Where is this function defined?"
- "What module is this type from?"
- "Where is this trait declared?"

**Returns**: File path and position of definition.

### 2. Find References

See all usages of a symbol:

```
Use LSP tool:
  operation: findReferences
  params: {
    file_path: "src/types/text.rs",
    position: { line: 20, character: 12 }
  }
```

**Use cases**:
- "Where is this type used?"
- "What calls this function?"
- "Which files import this module?"

**Returns**: List of all reference locations.

### 3. Get Hover Information

Inspect types and documentation:

```
Use LSP tool:
  operation: getHover
  params: {
    file_path: "src/core/value.rs",
    position: { line: 80, character: 25 }
  }
```

**Returns**:
- Type signature
- Trait bounds
- Documentation comments
- Inferred types for variables

**Use cases**:
- "What type is this variable?"
- "What does this function return?"
- "What are the trait bounds?"

### 4. Get Document Symbols

See all symbols in a file:

```
Use LSP tool:
  operation: getDocumentSymbols
  params: {
    file_path: "src/types/number.rs"
  }
```

**Returns**: Hierarchical list:
- Structs, enums, traits
- Impl blocks
- Functions, methods
- Constants, statics
- Type aliases

**Use cases**:
- "What's in this file?"
- "Overview of module structure"
- "Find all public APIs"

### 5. Get Workspace Symbols

Search symbols across entire project:

```
Use LSP tool:
  operation: getWorkspaceSymbols
  params: {
    query: "Text"
  }
```

**Returns**: All symbols matching query:
- Exact matches prioritized
- Fuzzy matching supported
- Includes file location

**Use cases**:
- "Find all types containing 'Validator'"
- "Locate enum variants"
- "Search for functions by name"

### 6. Get Signature Help

Understand function parameters:

```
Use LSP tool:
  operation: getSignatureHelp
  params: {
    file_path: "src/validation/rules.rs",
    position: { line: 55, character: 30 }
  }
```

**Returns**:
- Parameter names and types
- Current parameter being typed
- Function documentation

**Use cases**:
- "What parameters does this take?"
- "What's the order of arguments?"
- "What type should I pass here?"

## Exploration Workflows

### Understanding a New Module

```
Step 1: Get overview
  LSP getDocumentSymbols on module file
  -> See all structs, traits, functions

Step 2: Explore key types
  LSP getHover on interesting struct names
  -> Read documentation and fields

Step 3: Find implementations
  LSP findReferences on trait names
  -> See all impls across codebase

Step 4: Trace execution
  LSP goToDefinition to follow call chain
  -> Understand data flow
```

### Investigating a Trait Hierarchy

```
For paramdef Node trait hierarchy:

Step 1: Start at root trait
  Read src/types/node.rs
  LSP getDocumentSymbols -> see Node trait

Step 2: Find all implementations
  LSP findReferences on "Node"
  -> Shows all 23 node types

Step 3: Explore category traits
  LSP goToDefinition on "Leaf", "Container", etc.
  -> Navigate to category trait definitions

Step 4: Check specific impl
  LSP getHover on "impl Node for Text"
  -> See how Text implements Node
```

### Finding Feature Implementation

```
Question: "How does validation work?"

Step 1: Search for entry point
  LSP getWorkspaceSymbols: "validate"
  -> Find Validatable trait

Step 2: Read trait definition
  LSP goToDefinition -> src/validation/mod.rs
  Read the trait

Step 3: Find all validators
  LSP findReferences on Validatable
  -> See all implementing types

Step 4: Trace through example
  Pick one impl, LSP goToDefinition on validate method
  -> Follow the implementation
```

### Code Archaeology

```
Question: "Why was this designed this way?"

Step 1: Find original definition
  LSP goToDefinition on symbol

Step 2: Check all usages
  LSP findReferences
  -> See how it's used in practice

Step 3: Look for tests
  LSP getWorkspaceSymbols: "test_<feature>"
  -> Find related tests

Step 4: Check documentation
  LSP getHover on types
  -> Read doc comments
```

## paramdef-Specific Exploration

### Exploring Node Types (23 types)

```
# Start from Node trait
LSP getDocumentSymbols: src/types/node.rs

# Find all implementations
LSP findReferences: "Node"

# Group by category:
1. Group (2): Group, Panel
   LSP goToDefinition -> src/types/group/

2. Decoration (8): Notice, Separator, Link, Code, Image, Html, Video, Progress
   LSP goToDefinition -> src/types/decoration/

3. Container (7): Object, List, Mode, Matrix, Routing, Expirable, Reference
   LSP goToDefinition -> src/types/container/

4. Leaf (6): Text, Number, Boolean, Vector, Select, File
   LSP goToDefinition -> src/types/leaf/
```

### Understanding Subtype System

```
# Explore NumberSubtype
LSP getDocumentSymbols: src/subtype/number.rs
-> See all 60+ subtypes

# Find where subtypes are used
LSP findReferences: "NumberSubtype::Distance"
-> See usage in builders, validation

# Check unit integration
LSP getHover on NumberUnit
-> Understand subtype vs unit pattern
```

### Tracing Event Flow

```
# How events propagate
LSP goToDefinition: "EventBus"
-> src/event/bus.rs

LSP getHover on "Event" enum
-> See all event types

LSP findReferences: "Event::ValueChanged"
-> Find where events are emitted and handled
```

### Exploring Validation Pipeline

```
# Validation flow
LSP getWorkspaceSymbols: "Validator"
-> Find Validator trait

LSP findReferences: "Validator"
-> See all built-in validators

LSP goToDefinition: "Rules"
-> Understand rules composition

LSP findReferences: "validate"
-> Trace validation execution
```

### Feature Flag Architecture

```
# How features are organized
LSP getWorkspaceSymbols: "cfg"
-> Find all feature-gated code

Read Cargo.toml for feature definitions

LSP findReferences: "visibility"
-> See visibility feature usage

LSP findReferences: "validation"
-> See validation feature usage
```

## Navigation Patterns

### Bottom-Up (From Usage)

```
1. Start at call site
2. LSP goToDefinition -> see implementation
3. LSP findReferences on type -> see related usage
4. LSP getHover -> understand context
```

### Top-Down (From Definition)

```
1. Start at trait/struct definition
2. LSP findReferences -> see all impls/usages
3. Pick interesting usage
4. LSP goToDefinition on related types
5. Continue expanding understanding
```

### Cross-Cutting (Feature Exploration)

```
1. LSP getWorkspaceSymbols: "feature_name"
2. Group results by module
3. LSP goToDefinition on key types
4. LSP findReferences to see connections
```

## Combining LSP with Other Tools

### LSP + Read

```
1. LSP getDocumentSymbols -> get overview
2. Read file for detailed implementation
3. LSP getHover for specific type info
```

### LSP + Grep

```
1. LSP for semantic search (types, functions)
2. Grep for string patterns (comments, literals)
3. Combine results for complete picture
```

### LSP + Glob

```
1. Glob to find all files in module: "src/types/**/*.rs"
2. LSP getDocumentSymbols on each file
3. Build mental map of module structure
```

## Performance Tips

### Fast Symbol Lookup

```
# Instead of grep across codebase
LSP getWorkspaceSymbols: "Symbol"
-> Much faster for type/function names
```

### Batch Exploration

```
# Get all symbols once
LSP getDocumentSymbols
-> Cache results mentally

# Then use for multiple lookups
Rather than multiple Read calls
```

### Workspace-Wide Understanding

```
# Start broad, narrow down
1. LSP getWorkspaceSymbols: broad query
2. Filter interesting results
3. LSP goToDefinition on each
4. Build comprehensive picture
```

## Example Exploration: "How does Context work?"

```
Step 1: Find the Context type
  LSP getWorkspaceSymbols: "Context"
  -> src/runtime/context.rs

Step 2: See what's in Context
  LSP getDocumentSymbols: src/runtime/context.rs
  -> Context struct, methods, associated types

Step 3: Read Context struct definition
  Read src/runtime/context.rs:30-50
  -> See fields: schema, state, event_bus, etc.

Step 4: Explore key methods
  LSP getHover on "set" method
  -> See signature and documentation

  LSP goToDefinition on "set"
  -> Read implementation

Step 5: Find all Context usage
  LSP findReferences: "Context"
  -> See where it's created and used

Step 6: Understand integrations
  LSP goToDefinition on "EventBus"
  -> See how events integrate

  LSP goToDefinition on "Schema"
  -> See how schema connects
```

## Advanced Techniques

### Discovering Hidden Relationships

```
# Find trait bounds
LSP getHover on generic type parameter
-> See "T: Node + Validatable"

# Trace type inference
LSP getHover on let binding
-> Rust analyzer shows inferred type

# Explore macro expansions
LSP getHover on macro usage
-> See expanded code
```

### Understanding Compiler Errors

```
# After failed build
LSP getDiagnostics: file_with_error.rs

# Click on error
LSP getHover at error position
-> See detailed type information

# Follow suggested fixes
LSP goToDefinition on mentioned types
-> Understand what compiler wants
```

### Reverse Engineering Intent

```
# Start from tests
LSP getWorkspaceSymbols: "test_"
-> Find all test functions

# Read test
LSP goToDefinition on types in test
-> Understand API usage

# Trace to implementation
LSP goToDefinition on functions called
-> See actual implementation
```

## Troubleshooting

### Symbol Not Found

```
Possible causes:
1. Not in current workspace
2. LSP server indexing
3. Conditional compilation (#[cfg])

Solutions:
- LSP restartServer
- Check feature flags
- Use grep as fallback
```

### Too Many Results

```
LSP findReferences returns hundreds:
1. Filter by file pattern
2. Group by module mentally
3. Focus on public API usage first
4. Ignore test files for understanding
```

### Incomplete Information

```
If LSP getHover shows partial info:
1. Read source file directly
2. Check for proc macros
3. Look at expanded macro (cargo expand)
```

## Best Practices

1. **Start with getWorkspaceSymbols for new features**
2. **Use getDocumentSymbols for module overview**
3. **Combine goToDefinition + findReferences for full picture**
4. **Use getHover liberally for type info**
5. **Trust LSP for Rust semantics over grep**
6. **Restart LSP server if results seem stale**
7. **Use Read for implementation details after LSP navigation**

## Integration with Skills

Works great with:
- `/sequential-thinking` - Structured exploration
- `/rust-arch-review` - Understand architecture via LSP
- `/api-design` - Explore existing API patterns
- `/rust-patterns` - Find pattern examples in codebase

**LSP transforms code exploration from searching to navigating!** 🧭

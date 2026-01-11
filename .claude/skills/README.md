# Claude Code Skills for paramdef

This directory contains specialized skills for working with the paramdef Rust project.

## Workflow Skills

These skills help with planning, design, and review:

- **`/plan`** - Interactive planning session for spec-based development
- **`/interview`** - Interview process to create detailed specifications
- **`/rust-arch-review`** - Deep architecture review for Rust projects
- **`/api-design`** - Comprehensive API design review and discussion
- **`/rust-review`** - Code review focusing on Rust best practices
- **`/security-audit`** - Isolated security audit (runs in fork context)
- **`/deep-review`** - Thorough code review using Opus 4.5

## LSP-Powered Skills (NEW!)

These skills leverage Language Server Protocol for IDE-quality code intelligence:

- **`/lsp-refactor`** ⭐ - Safe refactoring with type-aware analysis
- **`/lsp-explore`** ⭐ - Navigate trait hierarchies and code structure
- **`/lsp-diagnostics`** ⭐ - Real-time error detection and fixes

## Rust Development Skills

These skills provide guidance for Rust development (LSP-enhanced):

- **`/rust-tdd`** - Test-Driven Development workflow
- **`/rust-patterns`** - Design patterns and LSP pattern discovery
- **`/rust-error-handling`** - Error handling patterns and best practices
- **`/rust-performance`** - Performance optimization techniques
- **`/rust-safety`** - Safety patterns and secure coding
- **`/rust-security`** - Security best practices
- **`/rust-async`** - Async/await patterns with Tokio
- **`/rust-docs`** - Documentation generation and improvement
- **`/rust-tracing`** - Tracing and structured logging

## General Skills

- **`/sequential-thinking`** - Structured reasoning for complex problems
- **`/context7-docs`** - Up-to-date library documentation lookup
- **`/filesystem`** - Filesystem operations and codebase navigation

## Recommended Workflow

### For New Features

1. **Plan First**: Use `/plan` or `/interview` to create a detailed spec
2. **Start New Session**: Provide the spec and implement in a fresh session
3. **Explore Codebase**: Use `/lsp-explore` to understand related code
4. **Use TDD**: Follow `/rust-tdd` workflow during implementation
5. **Check Diagnostics**: Use `/lsp-diagnostics` during development
6. **Review**: Use `/rust-review` (with LSP) before committing

### For Refactoring

1. **Explore**: Use `/lsp-explore` to map code dependencies
2. **Check Impact**: Use LSP findReferences to see usage
3. **Refactor Safely**: Use `/lsp-refactor` for type-safe changes
4. **Validate**: Use `/lsp-diagnostics` to catch errors early
5. **Test**: Run full test suite

### For Architecture Work

1. **Analyze**: Use `/rust-arch-review` (with LSP) to understand current state
2. **Design**: Use `/api-design` for public API changes
3. **Implement**: Follow established patterns from `/rust-patterns`
4. **Review**: Use `/deep-review` for comprehensive analysis

### For Debugging

1. **Get Diagnostics**: Use `/lsp-diagnostics` for compiler errors
2. **Navigate Code**: Use `/lsp-explore` to trace execution
3. **Fix Issues**: Use LSP getHover for type information
4. **Verify**: Re-check diagnostics after fixes

### For Security

1. **Audit**: Use `/security-audit` for isolated security review
2. **Fix**: Apply recommendations from audit report
3. **Verify**: Re-run audit to confirm fixes

## Skill Features

All skills:
- ✅ Auto-reload on changes (hot-reload enabled in v2.1.0)
- ✅ User-invocable from slash command menu
- ✅ Context-aware for paramdef project structure
- ✅ Follow current best practices (updated for Claude Code v2.1.0+)

Special features:
- **LSP skills** ⭐: Use rust-analyzer for IDE-quality code intelligence
  - Real-time diagnostics
  - Type-aware refactoring
  - Project-wide navigation
- `security-audit`: Runs in isolated context (`context: fork`)
- `deep-review`: Uses Opus 4.5 for maximum analysis quality
- `plan`, `interview`: Use AskUserQuestion for interactive sessions

## LSP Setup

To use LSP-powered skills, ensure rust-analyzer is installed:

```bash
# Install rust-analyzer
rustup component add rust-analyzer

# Verify installation
rust-analyzer --version
```

LSP skills will automatically use rust-analyzer for:
- Type checking and inference
- Finding references and definitions
- Real-time diagnostics (errors, warnings, hints)
- Workspace-wide symbol search

## Usage

Simply type `/` in Claude Code to see available skills, or invoke by name:

```
/plan
/rust-review
/security-audit
```

## Customization

Feel free to:
- Modify skills to match your workflow
- Add project-specific checks
- Combine multiple skills
- Create new skills following the same pattern

## More Information

- [Claude Code Documentation](https://code.claude.com/docs)
- [Skills Documentation](https://github.com/anthropics/claude-code/blob/main/CHANGELOG.md)

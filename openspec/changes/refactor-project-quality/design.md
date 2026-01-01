## Context
This change is primarily about developer experience and long-term maintainability:
- Users need a trustworthy, minimal path: README → crate-level docs → examples → deep docs.
- Contributors need a consistent module structure and test strategy.
- Existing `docs/` contain high-value design material, but some pages drifted from the code.

## Goals / Non-Goals
### Goals
- Make documentation accurate and navigable.
- Make the public API surface intentional and stable.
- Provide “golden path” examples that compile in CI.
- Improve code organization without unnecessary abstraction.

### Non-Goals
- No new runtime behavior/capabilities as part of this change.
- No large-scale renaming of public types unless explicitly approved.

## Decisions
### Decision: Avoid “clever” builder helper styles
We will prefer explicit builder methods and straightforward `if let` patterns over “option-aware” builder helpers (`maybe_*`) unless a specific use case demands it.

### Decision: Examples are first-class
We will add `examples/` and keep them compiling under supported feature sets.

### Decision: Documentation is treated as an API
Outdated docs are worse than missing docs; we will update or clearly mark stale pages.

## Risks / Trade-offs
- Updating docs may expose small API inconsistencies; we will decide whether to fix code or adjust docs case-by-case.
- Module/file moves can create noisy diffs; we will batch related moves and keep commits scoped.

## Migration Plan
- Start with docs/examples (no API risk).
- Then refactor internal module layout in a non-breaking way.
- Any breaking change requires explicit listing, migration note, and approval.

## Open Questions
- Are breaking changes allowed at all in this refactor?
- Should we target `docs.rs` as the canonical documentation surface, or keep `docs/` as primary?



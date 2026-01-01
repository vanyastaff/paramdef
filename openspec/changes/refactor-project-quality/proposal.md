# Change: Refactor Project Quality (Docs, Examples, Structure, Tests)

## Why
The library is already feature-rich, but the developer experience is held back by inconsistent public-facing documentation (README vs `docs/` vs rustdoc), and by drift between design docs and the actual source layout. This makes the project harder to adopt, review, and contribute to.

## What Changes
- Align documentation with the current codebase:
  - README becomes a real entry point (quick start, feature matrix, links).
  - `docs/` references are updated to match actual types, module names, and APIs.
  - Remove or clearly mark outdated docs (if any) to prevent misinformation.
- Add a first-class `examples/` directory with compile-tested examples.
- Standardize module/file organization and naming conventions to match modern Rust crate patterns:
  - Reduce boilerplate and inconsistencies across node/container/builders.
  - Introduce a stable “public surface” and keep internals private where possible.
- Improve test organization:
  - Keep fast unit tests colocated.
  - Add integration tests that verify public API and end-to-end workflows.

## Impact
- Affected specs: `project-quality` (new)
- Affected code: crate-wide (docs + examples + module layout + tests)
- **Breaking changes:** avoid by default; any required breakage must be explicitly marked and justified in this change.

## Non-Goals
- No new runtime features or behavior changes (unless required for correctness).
- No new external dependencies without strong justification.



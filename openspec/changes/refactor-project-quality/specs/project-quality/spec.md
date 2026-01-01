## ADDED Requirements

### Requirement: Documentation Accuracy
Project documentation MUST accurately reflect the current public API and architecture.

#### Scenario: Architecture docs match code
- **WHEN** a developer reads `docs/01-ARCHITECTURE.md` and `docs/19-PROJECT-STRUCTURE.md`
- **THEN** names, module paths, and described components match the actual `src/` layout and exported APIs

#### Scenario: Quick reference matches enums and types
- **WHEN** a developer uses `docs/15-QUICK-REFERENCE.md` as a lookup
- **THEN** referenced enum variants and examples compile against the current codebase

---

### Requirement: Readme as Entry Point
`README.md` MUST provide a minimal, correct path to success for new users.

#### Scenario: New user quick start
- **WHEN** a new user reads `README.md`
- **THEN** they can build a minimal schema/value example with correct imports and feature guidance

---

### Requirement: Compile-Tested Examples
The project SHALL provide an `examples/` directory with compile-tested examples that demonstrate common workflows.

#### Scenario: Examples compile on CI
- **WHEN** CI runs the project’s test suite under supported feature sets
- **THEN** examples compile successfully (at minimum: default features and `full`)

---

### Requirement: Intentional Public Surface
The crate root MUST expose an intentional public API surface that is documented and stable.

#### Scenario: Crate-root imports are stable
- **WHEN** a user imports common types from the crate root
- **THEN** the import paths remain stable across minor releases, unless explicitly marked as breaking



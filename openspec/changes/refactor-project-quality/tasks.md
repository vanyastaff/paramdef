## 1. Baseline & Audit
- [ ] 1.1 Inventory public API surface (crate-root re-exports, module docs, feature flags)
- [ ] 1.2 Identify documentation drift (README vs `docs/` vs rustdoc examples)
- [ ] 1.3 Identify file/module layout drift (`docs/19-PROJECT-STRUCTURE.md` vs `src/`)
- [ ] 1.4 Decide “public surface” policy (what is stable and re-exported)

## 2. Documentation Overhaul (Non-breaking)
- [ ] 2.1 Expand `README.md` into a real entry point (quick start, links to docs, feature matrix)
- [ ] 2.2 Ensure `src/lib.rs` crate-level docs compile and show modern usage (doctests)
- [ ] 2.3 Update `docs/` to match current code (remove outdated NodeKind/value snippets, fix module paths)
- [ ] 2.4 Add `docs/README.md` navigation page (what to read first, “start here”)

## 3. Examples (Compile-tested)
- [ ] 3.1 Add `examples/` with 3–5 focused examples (schema build, context usage, containers, feature-gated examples)
- [ ] 3.2 Ensure examples compile under the intended feature sets (at least `default` and `full`)

## 4. Structure & Organization (Non-breaking if possible)
- [ ] 4.1 Normalize module layout (prefer modern `foo.rs` + `foo/` style where it helps clarity)
- [ ] 4.2 Reduce repeated patterns where it improves readability (without introducing “clever” builder styles)
- [ ] 4.3 Add internal “style guide” doc for contributors (small file, actionable rules)

## 5. Tests
- [ ] 5.1 Add integration tests that validate crate-root API and common workflows
- [ ] 5.2 Add feature-gated tests where behavior differs

## 6. Validation
- [ ] 6.1 Run `cargo fmt --all`
- [ ] 6.2 Run `cargo clippy --workspace --all-features -- -D warnings`
- [ ] 6.3 Run `cargo test --workspace --all-features`
- [ ] 6.4 Run `cargo doc --no-deps --all-features`



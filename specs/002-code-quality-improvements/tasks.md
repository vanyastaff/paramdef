# Tasks: Code Quality Improvements

**Feature**: 002-code-quality-improvements  
**Branch**: `002-code-quality-improvements`  
**Status**: Planning Complete → Implementation Ready  
**Based on**: spec.md, plan.md, data-model.md

---

## Task Format

Each task follows: `- [ ] T### [Markers] Description`

**Markers**:
- `[P]` = Parallelizable (can work on simultaneously with other [P] tasks)
- `[US1]` = User Story 1 (Immutability Fixes)
- `[US2]` = User Story 2 (API Ergonomics)
- `[US3]` = User Story 3 (Performance)
- `[US4]` = User Story 4 (Documentation)
- `[CRITICAL]` = Blocking task, must complete before dependents
- `[TEST]` = Test task (TDD - write before implementation)

---

## Progress Summary

**Total Tasks**: 7 / 92  
**Phase 1 (Setup)**: 2 / 2 ✅  
**Phase 2 (Foundation)**: 5 / 5 ✅  
**Phase 3 (US1 - Immutability)**: 0 / 15  
**Phase 4 (US2 - Ergonomics)**: 0 / 28  
**Phase 5 (US3 - Performance)**: 0 / 16  
**Phase 6 (US4 - Documentation)**: 0 / 18  
**Phase 7 (Polish)**: 0 / 8  

---

## Phase 1: Setup and Preparation

**Goal**: Create branch, verify baseline, document current state

### T001 [CRITICAL] Create feature branch ✅
- Create branch `002-code-quality-improvements` from `main`
- Verify all tests pass on baseline: `cargo nextest run --workspace --all-features`
- Commit baseline: "chore: create feature branch for code quality improvements"

### T002 [P] Document current metrics baseline ✅
- Count mutable schema fields: `grep -r "pub.*mut" src/types/`
- Count `&mut self` methods on schema types
- Measure doc example success rate: `cargo test --doc 2>&1 | grep -c "test result"`
- Run baseline benchmarks if they exist
- Document in `specs/002-code-quality-improvements/BASELINE.md`

---

## Phase 2: Foundational Infrastructure

**Goal**: Build core infrastructure needed by all user stories

### T003 [CRITICAL] [TEST] Create UiStateManager tests (src/context/ui_state.rs) ✅
**TDD Red Phase**: Write failing tests for UiStateManager
- Test: `test_ui_state_creation_and_defaults()`
- Test: `test_panel_collapsed_set_get()`
- Test: `test_panel_state_interaction_timestamp()`
- Test: `test_multiple_panels_independent_state()`
- Test: `test_ui_state_clear_and_len()`
- Expected: All tests fail (module doesn't exist yet)

### T004 [CRITICAL] Implement UiStateManager (src/context/ui_state.rs) ✅
**TDD Green Phase**: Implement to pass T003 tests
- Create new file `src/context/ui_state.rs`
- Implement `UiStateManager` struct with `FxHashMap<Key, PanelState>`
- Implement `PanelState` struct with `collapsed: bool`, `last_interaction: Option<Instant>`
- Implement methods: `new()`, `with_capacity()`, `set_panel_collapsed()`, `is_panel_collapsed()`
- Implement methods: `get_panel_state()`, `get_or_create_panel_state()`, `clear()`, `len()`, `is_empty()`
- Run tests: All T003 tests should pass
- File: `src/context/ui_state.rs`

### T005 [P] [TEST] Add UiStateManager serialization tests (with serde feature) ✅
**TDD Red Phase**: Test serde support
- Test: `test_ui_state_serialize_deserialize()` (with #[cfg(feature = "serde")])
- Test: `test_ui_state_last_interaction_not_serialized()`
- Expected: Tests fail (serde impl doesn't exist)

### T006 [P] Implement UiStateManager serialization (with serde feature) ✅
**TDD Green Phase**: Add serde support
- Add `#[cfg(feature = "serde")]` module with Serialize/Deserialize impls
- Serialize as map of `Key -> bool` (collapsed state only, skip last_interaction)
- Run T005 tests: Should pass
- File: `src/context/ui_state.rs`

### T007 [CRITICAL] Integrate UiStateManager into Context ✅
- Add `ui_state: UiStateManager` field to Context struct
- Initialize in `Context::new()` and `Context::with_event_bus()`
- Add convenience methods: `ui_state()`, `ui_state_mut()`, `is_panel_collapsed()`, `set_panel_collapsed()`
- Update Context Debug impl to include ui_state
- File: `src/context/mod.rs`

---

## Phase 3: User Story 1 - Immutability Fixes

**Goal**: Eliminate all schema mutation, establish proper runtime state management

**Success Criteria**:
- ✅ Zero mutable fields in schema structs
- ✅ Zero `&mut self` methods on schema types (except builders)
- ✅ Panel UI state managed in Context
- ✅ Backward compatibility via deprecation warnings

### T008 [US1] [TEST] Write Panel immutability verification tests ✅
**TDD Red Phase**: Verify Panel is truly immutable
- Test: `test_panel_schema_is_immutable()` - verify no mutable fields
- Test: `test_panel_runtime_state_in_context()` - verify state in Context
- Test: `test_multiple_contexts_independent_panel_state()` - verify isolation
- Test: `test_panel_schema_shareable_across_threads()` - verify Send+Sync
- Expected: Tests fail (Panel still has collapsed field)
- File: `tests/immutability_tests.rs`

### T009 [US1] [CRITICAL] Remove Panel::collapsed field ✅
**TDD Green Phase**: Fix schema immutability
- Remove `collapsed: bool` field from Panel struct
- Remove `collapsed()` getter method
- Update Panel Debug impl (remove collapsed field)
- File: `src/types/group/panel.rs`

### T010 [US1] Update PanelBuilder to use initial state hint ✅
- Modify `PanelBuilder::collapsed()` to store hint (not mutate schema)
- Pass collapsed hint to Context during initialization (via metadata or separate mechanism)
- Document that `.collapsed()` is an initial UI state hint, not schema state
- File: `src/types/group/panel.rs`

### T011 [US1] [TEST] Write Layout trait tests for set_collapsed deprecation ✅
**TDD Red Phase**: Verify set_collapsed is gone
- Test: `test_layout_trait_no_set_collapsed()` - verify method removed from trait
- Test: `test_panel_set_collapsed_via_context()` - verify Context API works
- Expected: Compilation fails if set_collapsed still exists on trait
- File: `tests/immutability_tests.rs`

### T012 [US1] Remove set_collapsed from Layout trait ✅
**TDD Green Phase**: Remove mutable method from trait
- Remove `set_collapsed(&mut self, collapsed: bool)` from Layout trait
- Remove `is_collapsed(&mut self)` from Layout trait
- Remove implementation from Panel
- File: `src/types/traits/category.rs`

### T013 [US1] [TEST] Write visibility mutation tests (23 node types)
**TDD Red Phase**: Verify set_visibility_rule deprecation
- Test: `test_no_set_visibility_after_construction()` for each node type
- Test: `test_visibility_set_via_builder_only()`
- Expected: Tests fail if mutable visibility setters exist
- File: `tests/immutability_tests.rs`

### T014 [US1] Deprecate set_visibility_rule on Group type
- Add `#[deprecated(since = "0.4.0", note = "Set visibility via builder. Use .visible_when() or .visibility() during construction.")]`
- Add deprecation to `set_visibility_rule(&mut self)` method
- File: `src/types/group/group.rs`

### T015 [US1] [P] Deprecate set_visibility_rule on Panel type
- Add `#[deprecated]` attribute with migration message
- File: `src/types/group/panel.rs`

### T016 [US1] [P] Deprecate set_visibility_rule on all Leaf types (6 types)
- Text (`src/types/leaf/text.rs`)
- Number (`src/types/leaf/number.rs`)
- Boolean (`src/types/leaf/boolean.rs`)
- Vector (`src/types/leaf/vector.rs`)
- Select (`src/types/leaf/select.rs`)
- File (`src/types/leaf/file.rs`)
- Add `#[deprecated]` to each

### T017 [US1] [P] Deprecate set_visibility_rule on all Container types (7 types)
- Object (`src/types/container/object.rs`)
- List (`src/types/container/list.rs`)
- Mode (`src/types/container/mode.rs`)
- Matrix (`src/types/container/matrix.rs`)
- Routing (`src/types/container/routing.rs`)
- Workflow (`src/types/container/workflow.rs`)
- Dataflow (`src/types/container/dataflow.rs`)
- Add `#[deprecated]` to each

### T018 [US1] [P] Deprecate set_visibility_rule on all Decoration types (8 types)
- Notice (`src/types/decoration/notice.rs`)
- Separator (`src/types/decoration/separator.rs`)
- Link (`src/types/decoration/link.rs`)
- Code (`src/types/decoration/code.rs`)
- Image (`src/types/decoration/image.rs`)
- Progress (`src/types/decoration/progress.rs`)
- Badge (`src/types/decoration/badge.rs`)
- Spacer (`src/types/decoration/spacer.rs`)
- Add `#[deprecated]` to each

### T019 [US1] Update all builder types to emphasize build-time visibility
- Add doc comments clarifying visibility must be set during construction
- Update examples to show `.visible_when()` in builder chain
- Files: All 23 builder implementations

### T020 [US1] [TEST] Write comprehensive immutability integration tests
**TDD Red Phase**: Full system verification
- Test: `test_schema_shareable_across_10_contexts()`
- Test: `test_arc_schema_no_mutex_needed()`
- Test: `test_context_ui_state_independent_of_schema()`
- Expected: All pass if previous tasks completed correctly
- File: `tests/immutability_tests.rs`

### T021 [US1] Create migration guide for immutability changes
- Document Panel.collapsed → Context.ui_state migration
- Document set_visibility_rule deprecation with examples
- Show before/after code samples
- File: `docs/MIGRATION-0.3-to-0.4.md`

### T022 [US1] Run immutability verification audit
- Run: `cargo clippy --workspace --all-features -- -D warnings`
- Verify: Zero mutable fields in schema types: `grep -r "pub.*mut" src/types/`
- Verify: Zero `&mut self` on Node trait impls (except builders)
- Document results in BASELINE.md

---

## Phase 4: User Story 2 - API Ergonomics

**Goal**: Reduce boilerplate by 40-50%, improve developer experience

**Success Criteria**:
- ✅ Context::from_schema() available
- ✅ Required field constructors (1 line vs 4)
- ✅ Validation shortcuts (`.validate_email()` vs verbose Rules)
- ✅ Error recovery methods (`get_text_or()`)
- ✅ ValidationError includes full paths
- ✅ Object builder has `.fields()` bulk method

### T023 [US2] [TEST] Write Context::from_schema tests
**TDD Red Phase**: Test convenience constructor
- Test: `test_context_from_schema_auto_wraps_arc()`
- Test: `test_context_from_schema_equivalent_to_new()`
- Expected: Tests fail (method doesn't exist)
- File: `tests/ergonomics_tests.rs`

### T024 [US2] [CRITICAL] Implement Context::from_schema
**TDD Green Phase**: Add convenience constructor
- Add `pub fn from_schema(schema: Schema) -> Self`
- Implementation: `Self::new(Arc::new(schema))`
- Add doc comment with example
- File: `src/context/mod.rs`

### T025 [US2] [TEST] Write Text::required tests
**TDD Red Phase**: Test shorthand constructor
- Test: `test_text_required_creates_valid_node()`
- Test: `test_text_required_has_required_flag()`
- Test: `test_text_required_has_label()`
- Expected: Tests fail (method doesn't exist)
- File: `tests/ergonomics_tests.rs`

### T026 [US2] Implement Text::required shorthand
**TDD Green Phase**: Add constructor
- Add `pub fn required(key: impl Into<Key>, label: impl Into<SmartStr>) -> Self`
- Implementation: `Self::builder(key).label(label).required().build()`
- Add doc comment with before/after example
- File: `src/types/leaf/text.rs`

### T027 [US2] [P] [TEST] Write Number::required tests
**TDD Red Phase**: Test shorthand constructor
- Test: `test_number_required_creates_valid_node()`
- Test: `test_number_required_has_required_flag()`
- Expected: Tests fail
- File: `tests/ergonomics_tests.rs`

### T028 [US2] [P] Implement Number::required shorthand
**TDD Green Phase**: Add constructor
- Add `pub fn required(key: impl Into<Key>, label: impl Into<SmartStr>) -> Self`
- File: `src/types/leaf/number.rs`

### T029 [US2] [P] [TEST] Write Boolean::required tests
**TDD Red Phase**: Test shorthand constructor
- Test: `test_boolean_required_creates_valid_node()`
- Expected: Tests fail
- File: `tests/ergonomics_tests.rs`

### T030 [US2] [P] Implement Boolean::required shorthand
**TDD Green Phase**: Add constructor
- Add `pub fn required(key: impl Into<Key>, label: impl Into<SmartStr>) -> Self`
- File: `src/types/leaf/boolean.rs`

### T031 [US2] [TEST] Write TextBuilder validation shortcut tests
**TDD Red Phase**: Test builder shortcuts (requires validation feature)
- Test: `test_text_builder_validate_required()` with #[cfg(feature = "validation")]
- Test: `test_text_builder_validate_email()`
- Test: `test_text_builder_validate_min_length()`
- Test: `test_text_builder_validate_max_length()`
- Expected: Tests fail (methods don't exist)
- File: `tests/ergonomics_tests.rs`

### T032 [US2] Implement TextBuilder validation shortcuts
**TDD Green Phase**: Add validation methods
- Add `#[cfg(feature = "validation")]` section to TextBuilder
- Implement `validate_required(self) -> Self`
- Implement `validate_email(self) -> Self`
- Implement `validate_min_length(self, min: usize) -> Self`
- Implement `validate_max_length(self, max: usize) -> Self`
- Each method adds appropriate Rule to rules collection
- File: `src/types/leaf/text.rs`

### T033 [US2] [P] [TEST] Write NumberBuilder validation shortcut tests
**TDD Red Phase**: Test builder shortcuts
- Test: `test_number_builder_validate_required()`
- Test: `test_number_builder_validate_min()`
- Test: `test_number_builder_validate_max()`
- Expected: Tests fail
- File: `tests/ergonomics_tests.rs`

### T034 [US2] [P] Implement NumberBuilder validation shortcuts
**TDD Green Phase**: Add validation methods
- Add `#[cfg(feature = "validation")]` section
- Implement `validate_required()`, `validate_min()`, `validate_max()`
- File: `src/types/leaf/number.rs`

### T035 [US2] [P] [TEST] Write BooleanBuilder validation shortcut tests
**TDD Red Phase**: Test builder shortcuts
- Test: `test_boolean_builder_validate_required()`
- Expected: Tests fail
- File: `tests/ergonomics_tests.rs`

### T036 [US2] [P] Implement BooleanBuilder validation shortcuts
**TDD Green Phase**: Add validation methods
- Add `#[cfg(feature = "validation")]` section
- Implement `validate_required()`
- File: `src/types/leaf/boolean.rs`

### T037 [US2] [TEST] Write Context error recovery tests
**TDD Red Phase**: Test get_*_or methods
- Test: `test_context_get_text_or_with_existing_value()`
- Test: `test_context_get_text_or_with_missing_key()`
- Test: `test_context_get_text_or_with_wrong_type()`
- Test: `test_context_get_int_or()`, `test_context_get_bool_or()`
- Expected: Tests fail (methods don't exist)
- File: `tests/ergonomics_tests.rs`

### T038 [US2] Implement Context error recovery methods
**TDD Green Phase**: Add fallback getters
- Add `pub fn get_text_or<'a>(&'a self, key: &str, default: &'a str) -> &'a str`
- Add `pub fn get_int_or(&self, key: &str, default: i64) -> i64`
- Add `pub fn get_bool_or(&self, key: &str, default: bool) -> bool`
- Add `pub fn get_float_or(&self, key: &str, default: f64) -> f64`
- Implementation: Call existing getter, return default on error
- File: `src/context/mod.rs`

### T039 [US2] [TEST] Write ValidationError path tests
**TDD Red Phase**: Test enhanced ValidationError
- Test: `test_validation_error_with_path()`
- Test: `test_validation_error_simple_top_level()`
- Test: `test_validation_error_nested_object_path()`
- Test: `test_validation_error_constructors()` (required, min_length, etc.)
- Expected: Tests fail (fields don't exist)
- File: `tests/ergonomics_tests.rs`

### T040 [US2] [CRITICAL] Enhance ValidationError with path field
**TDD Green Phase**: Add path support
- Add `path: SmartStr` field to ValidationError struct
- Add `field: SmartStr` field to ValidationError struct
- Update constructors: `new()` (4 args), `simple()` (3 args, path=field)
- Add specific constructors: `required()`, `min_length()`, `max_length()`
- Update Display impl to show path when relevant
- File: Depends on where ValidationError is defined (likely `src/event/types.rs` based on plan)

### T041 [US2] Update all validation error creation sites to include paths
- Find all places creating ValidationError: `grep -r "ValidationError::new" src/`
- Update to use 4-argument form or `simple()` for top-level fields
- For nested object validation, construct proper dot-separated paths
- Files: Multiple (wherever ValidationError is created)

### T042 [US2] [TEST] Write ObjectBuilder.fields() tests
**TDD Red Phase**: Test bulk field addition
- Test: `test_object_builder_fields_bulk_add()`
- Test: `test_object_builder_fields_empty_iter()`
- Test: `test_object_builder_fields_mixed_types()`
- Expected: Tests fail (method doesn't exist)
- File: `tests/ergonomics_tests.rs`

### T043 [US2] Implement ObjectBuilder.fields() bulk method
**TDD Green Phase**: Add bulk field addition
- Add `pub fn fields<I>(mut self, pairs: I) -> Self where I: IntoIterator<Item = (impl Into<Key>, Arc<dyn Node>)>`
- Implementation: Iterate and call existing `child()` method for each pair
- Add doc comment with example
- File: `src/types/container/object.rs`

### T044 [US2] [TEST] Write ValueBuilder tests
**TDD Red Phase**: Test ergonomic Value::Object construction
- Test: `test_value_builder_empty_object()`
- Test: `test_value_builder_single_field()`
- Test: `test_value_builder_multiple_fields_chained()`
- Test: `test_value_builder_fields_bulk()`
- Test: `test_value_builder_field_if_conditional()`
- Test: `test_value_builder_nested_objects()`
- Expected: Tests fail (ValueBuilder doesn't exist)
- File: `tests/ergonomics_tests.rs`

### T045 [US2] Create ValueBuilder implementation
**TDD Green Phase**: Implement builder
- Create new file `src/core/value/builder.rs`
- Implement ValueBuilder struct with `fields: IndexMap<Key, Value>`
- Implement methods: `new()`, `with_capacity()`, `field()`, `fields()`, `field_if()`, `build()`
- Implement `From<ValueBuilder> for Value`
- Add to `src/core/value/mod.rs` module
- File: `src/core/value/builder.rs`

### T046 [US2] Add Value::object() and Value::object_with_capacity()
**TDD Green Phase**: Add builder constructors to Value
- Add `pub fn object() -> ValueBuilder` method to Value impl
- Add `pub fn object_with_capacity(capacity: usize) -> ValueBuilder`
- Add doc comments with examples
- File: `src/core/value/mod.rs`

### T047 [US2] Create ergonomics cookbook examples
- Document before/after for all ergonomic improvements
- Required field creation (4 lines → 1 line)
- Validation setup (5 lines → 2 lines)
- Context creation (2 lines → 1 line)
- Error recovery (manual unwrap_or → get_*_or)
- File: `specs/002-code-quality-improvements/COOKBOOK_ERGONOMICS.md`

### T048 [US2] Measure and document boilerplate reduction
- Count lines in common patterns before/after
- Calculate percentage reduction (target: 40-50%)
- Document in BASELINE.md
- Update SUCCESS_CRITERIA.md

### T049 [US2] Update all examples to use new ergonomic APIs
- Update examples in doc comments to use new shortcuts
- Update integration tests to demonstrate new APIs
- Update README.md examples if applicable
- Files: Multiple (doc comments throughout codebase)

### T050 [US2] Run ergonomics integration tests
- Run: `cargo test --workspace --all-features ergonomics`
- Verify: All new APIs work correctly
- Verify: No regressions in existing functionality
- Document any issues in ISSUES.md

---

## Phase 5: User Story 3 - Performance Optimizations

**Goal**: Reduce cloning by 66%, improve throughput by 20-30%

**Success Criteria**:
- ✅ Event uses Arc<Value> instead of cloning
- ✅ RollbackStorage uses stack buffer for <8 fields
- ✅ Context::get_many() bulk getter implemented
- ✅ Benchmarks show 20-30% improvement

### T051 [US3] [TEST] Write Event Arc<Value> tests
**TDD Red Phase**: Test Arc-based value sharing
- Test: `test_event_value_changed_uses_arc()`
- Test: `test_event_value_changing_uses_arc()`
- Test: `test_event_validated_no_clone()`
- Test: `test_multiple_subscribers_share_arc()`
- Expected: Tests fail (Event still clones)
- File: `tests/performance_tests.rs`

### T052 [US3] [CRITICAL] Change Event to use Arc<Value>
**TDD Green Phase**: Update Event enum
- Update `ValueChanging` variant: `old_value: Option<Arc<Value>>`, `new_value: Arc<Value>`
- Update `ValueChanged` variant: `old_value: Option<Arc<Value>>`, `new_value: Arc<Value>`
- Update other Event variants that carry values
- File: `src/event/types.rs`

### T053 [US3] Update Context to emit Arc<Value> in events
- Modify `Context::set()` to wrap values in Arc before emitting events
- Store values as Arc internally if event_bus is enabled
- Update event emission calls throughout Context
- File: `src/context/mod.rs`

### T054 [US3] Update event subscribers in tests/examples
- Update all code that matches on Event variants
- Change from `Value` to `Arc<Value>` in patterns
- Files: Multiple (tests, examples, docs)

### T055 [US3] [TEST] Write RollbackStorage tests
**TDD Red Phase**: Test stack/heap optimization
- Test: `test_rollback_storage_small_uses_stack()` (1-8 fields)
- Test: `test_rollback_storage_large_uses_heap()` (9+ fields)
- Test: `test_rollback_storage_upgrade_small_to_large()`
- Test: `test_rollback_storage_iter()`
- Test: `test_rollback_storage_clear()`
- Expected: Tests fail (RollbackStorage doesn't exist)
- File: `tests/performance_tests.rs`

### T056 [US3] Implement RollbackStorage enum
**TDD Green Phase**: Create optimized storage
- Create new file `src/context/rollback.rs`
- Implement enum: `Small { buffer: [(Key, Option<Value>); 8], count: usize } | Large(FxHashMap)`
- Implement methods: `new()`, `with_capacity()`, `store()`, `iter()`, `len()`, `clear()`
- Implement automatic upgrade from Small to Large when >8 fields
- File: `src/context/rollback.rs`

### T057 [US3] [TEST] Write Context::set_many_transactional tests
**TDD Red Phase**: Test transactional bulk updates
- Test: `test_set_many_transactional_success()`
- Test: `test_set_many_transactional_rollback_on_validation_error()`
- Test: `test_set_many_transactional_small_storage()` (<8 fields)
- Test: `test_set_many_transactional_large_storage()` (>8 fields)
- Expected: Tests fail (method doesn't exist or uses wrong storage)
- File: `tests/performance_tests.rs`

### T058 [US3] Implement Context::set_many_transactional
**TDD Green Phase**: Add transactional bulk setter
- Add `pub fn set_many_transactional<I, K, V>(&mut self, updates: I) -> Result<()>`
- Use RollbackStorage based on update count
- Store old values, apply changes, rollback on any error
- Add doc comment with example
- File: `src/context/mod.rs`

### T059 [US3] [TEST] Write Context::get_many tests
**TDD Red Phase**: Test bulk getter
- Test: `test_context_get_many_existing_keys()`
- Test: `test_context_get_many_mixed_existing_missing()`
- Test: `test_context_get_many_empty_keys()`
- Test: `test_context_get_many_single_lookup()` (verify only 1 batch operation)
- Expected: Tests fail (method doesn't exist)
- File: `tests/performance_tests.rs`

### T060 [US3] Implement Context::get_many bulk getter
**TDD Green Phase**: Add bulk getter
- Add `pub fn get_many<'a, I>(&'a self, keys: I) -> impl Iterator<Item = (&'a Key, Option<&'a Value>)>`
- Implementation: Convert keys to Key, look up in nodes map, return iterator
- Optimize for single-pass iteration
- File: `src/context/mod.rs`

### T061 [US3] [TEST] Document zero-copy values() iterator
- Verify existing `Context::values()` returns references (not clones)
- Add test: `test_context_values_zero_copy()`
- Add documentation example showing zero-copy pattern
- File: `src/context/mod.rs` (doc comments)

### T062 [US3] Create event cloning benchmark
**TDD Red Phase**: Measure before/after
- Create `benches/event_cloning.rs`
- Benchmark: Set 1000 values with events enabled (before Arc<Value>)
- Benchmark: Set 1000 values with events enabled (after Arc<Value>)
- Measure: Clone count reduction (target: 66% = 3 clones → 1 clone per set)
- File: `benches/event_cloning.rs`

### T063 [US3] Run event cloning benchmark and verify 66% reduction
- Run: `cargo bench --bench event_cloning`
- Verify: Clone count reduced from ~3000 to ~1000 (66% reduction)
- Document results in BASELINE.md
- If not meeting target, investigate and optimize

### T064 [US3] Create transactional update benchmark
- Create `benches/transactional.rs`
- Benchmark: Small transactions (1-8 fields) with stack buffer
- Benchmark: Large transactions (9-100 fields) with heap
- Measure: Allocations (target: 0 for small, 1 for large)
- File: `benches/transactional.rs`

### T065 [US3] Run transactional benchmark and verify zero allocations (small)
- Run: `cargo bench --bench transactional`
- Verify: Zero heap allocations for transactions ≤8 fields
- Verify: Single allocation for transactions >8 fields
- Document results in BASELINE.md

### T066 [US3] Create overall throughput benchmark
- Create `benches/throughput.rs` or extend existing benchmarks
- Benchmark: High-frequency updates (1000-10000 per second)
- Measure: Updates/second before and after all optimizations
- Target: 20-30% throughput improvement
- File: `benches/throughput.rs`

---

## Phase 6: User Story 4 - Documentation

**Goal**: 95%+ examples compile, comprehensive cookbook, helpful errors

**Success Criteria**:
- ✅ All doc examples compile successfully
- ✅ COOKBOOK.md with 10+ recipes
- ✅ Error hints provide actionable guidance
- ✅ Type count corrected (14→23)

### T067 [US4] Audit all doc comment examples
- Run: `cargo test --doc --all-features 2>&1 | tee doc_test_results.txt`
- Identify all failing examples
- Create list of files needing fixes
- Document in `specs/002-code-quality-improvements/DOC_AUDIT.md`

### T068 [US4] Fix doc examples in core/ module
- Review all doc comments in `src/core/`
- Fix compilation errors (missing imports, wrong types, etc.)
- Remove `ignore` markers where possible
- Add `no_run` only if example requires external resources
- Run: `cargo test --doc --package paramdef --lib core`
- Files: `src/core/*.rs`

### T069 [US4] [P] Fix doc examples in types/ module
- Review all doc comments in `src/types/`
- Fix compilation errors
- Update examples to use new ergonomic APIs where applicable
- Run: `cargo test --doc --package paramdef --lib types`
- Files: `src/types/**/*.rs`

### T070 [US4] [P] Fix doc examples in context/ module
- Review all doc comments in `src/context/`
- Fix compilation errors
- Update examples to show new convenience methods
- Run: `cargo test --doc --package paramdef --lib context`
- Files: `src/context/*.rs`

### T071 [US4] [P] Fix doc examples in validation/ module (if validation feature)
- Review all doc comments in `src/validation/`
- Fix compilation errors
- Files: `src/validation/**/*.rs`

### T072 [US4] [P] Fix doc examples in event/ module (if events feature)
- Review all doc comments in `src/event/`
- Fix compilation errors
- Update Event examples to use Arc<Value>
- Files: `src/event/*.rs`

### T073 [US4] Verify 95%+ doc example success rate
- Run: `cargo test --doc --all-features`
- Count: Total examples vs passing examples
- Calculate success rate
- Target: ≥95% passing
- Document in SUCCESS_CRITERIA.md

### T074 [US4] [TEST] Write Error::hint() tests
**TDD Red Phase**: Test error hint system
- Test: `test_error_type_mismatch_hint()`
- Test: `test_error_validation_hint()`
- Test: `test_error_not_found_hint()`
- Test: `test_error_custom_hint()`
- Test: `test_error_with_hint_formatting()`
- Expected: Tests fail (hint system doesn't exist)
- File: `tests/ergonomics_tests.rs`

### T075 [US4] Implement Error::hint() system
**TDD Green Phase**: Add error hints
- Add optional `hint: Option<String>` field to relevant Error variants
- Add `#[cfg_attr(feature = "serde", serde(skip))]` to hint fields
- Implement `pub fn hint(&self) -> Option<&str>` method
- Implement `pub fn with_hint(&self) -> String` formatter
- Implement `pub fn with_custom_hint(self, hint: impl Into<String>) -> Self`
- File: `src/core/error.rs`

### T076 [US4] Add default hints for all Error variants
- TypeMismatch: "Use get_{expected_type}() for {expected} values"
- NotFound: "Available keys: {keys}. Did you mean '{suggestion}'?"
- Validation: Error-specific hints (email format, length, range, etc.)
- MissingRequired: "Provide a value or remove the REQUIRED flag"
- OutOfRange: "Check min/max constraints"
- File: `src/core/error.rs`

### T077 [US4] Create COOKBOOK.md with 10+ recipes
- Recipe 1: Creating a simple form with required fields
- Recipe 2: Adding validation with shortcuts
- Recipe 3: Handling validation errors with paths
- Recipe 4: Building complex objects with ValueBuilder
- Recipe 5: Transactional bulk updates
- Recipe 6: Event-driven reactive updates
- Recipe 7: Conditional field visibility
- Recipe 8: Nested object structures
- Recipe 9: Custom validation logic
- Recipe 10: Error recovery patterns
- File: `docs/COOKBOOK.md`

### T078 [US4] Create error handling guide
- Document all Error variants with examples
- Show how to handle each error type
- Explain recoverable vs non-recoverable errors
- Show hint() usage patterns
- Add to Error module documentation
- File: `src/core/error.rs` (module-level docs)

### T079 [US4] Correct type count documentation (14→23)
- Find all references to "14 types": `grep -ri "14 types" .`
- Update to "23 types" or "23 node types"
- List all 23 types in documentation:
  - 2 Group (Group, Panel)
  - 8 Decoration (Notice, Separator, Link, Code, Image, Progress, Badge, Spacer)
  - 7 Container (Object, List, Mode, Matrix, Routing, Workflow, Dataflow)
  - 6 Leaf (Text, Number, Boolean, Vector, Select, File)
- Files: README.md, docs/02-TYPE-SYSTEM.md, CLAUDE.md, etc.

### T080 [US4] Update architecture documentation
- Update docs/01-ARCHITECTURE.md with UiStateManager
- Document runtime state separation (schema vs UI state)
- Update diagrams if they exist
- File: `docs/01-ARCHITECTURE.md`

### T081 [US4] Update design decisions documentation
- Add decision: Why Arc<Value> in events (performance)
- Add decision: Why RollbackStorage enum (stack optimization)
- Add decision: Why validation shortcuts (ergonomics)
- File: `docs/17-DESIGN-DECISIONS.md`

### T082 [US4] Update CLAUDE.md with new patterns
- Document new convenience constructors (Text::required(), etc.)
- Document new Context methods (from_schema(), get_*_or(), get_many())
- Document ValueBuilder usage
- Update examples to show new APIs
- File: `CLAUDE.md`

### T083 [US4] Create complete migration guide
- Combine all migration notes from previous phases
- Before/after examples for all breaking changes
- Timeline for deprecations (v0.4.0 deprecate, v0.6.0 remove)
- Tool support (cargo fix compatibility where possible)
- File: `docs/MIGRATION-0.3-to-0.4.md`

### T084 [US4] Review and polish all public API documentation
- Review all `pub` items for missing docs
- Ensure all public methods have examples
- Check for broken links in docs
- Run: `cargo doc --no-deps --all-features --open`
- Verify documentation renders correctly

---

## Phase 7: Polish and Validation

**Goal**: Ensure quality, verify success criteria, prepare for merge

### T085 Run full test suite with all feature combinations
- Run: `cargo nextest run --workspace --no-default-features`
- Run: `cargo nextest run --workspace --features visibility`
- Run: `cargo nextest run --workspace --features validation`
- Run: `cargo nextest run --workspace --features serde`
- Run: `cargo nextest run --workspace --features events`
- Run: `cargo nextest run --workspace --all-features`
- Verify: All tests pass

### T086 Run all benchmarks and document performance gains
- Run: `cargo bench --workspace`
- Collect results: Event cloning, transactional, throughput
- Verify: 66% clone reduction, 20-30% throughput gain
- Document final results in SUCCESS_CRITERIA.md

### T087 Verify all success criteria met
- SC-001: Zero mutable fields in schema structs ✅
- SC-002: Zero `&mut self` on schema types ✅
- SC-003: Schema types remain Send + Sync ✅
- SC-005: 40-50% boilerplate reduction ✅
- SC-006: 95%+ doc examples compile ✅
- SC-007: 100% error variants have hints ✅
- SC-010: 66% fewer clones ✅
- SC-013: 20-30% throughput improvement ✅
- SC-015: COOKBOOK.md with 10+ recipes ✅
- Document verification in SUCCESS_CRITERIA.md

### T088 Run linting and formatting
- Run: `cargo fmt --all -- --check`
- Run: `cargo clippy --workspace --all-features -- -D warnings`
- Fix any issues found
- Verify: Zero warnings

### T089 Update CHANGELOG.md
- Add v0.4.0 section
- List all breaking changes with migration notes
- List all new features
- List all performance improvements
- List all documentation improvements
- Follow Keep a Changelog format

### T090 Create PR description
- Summarize feature: "Code Quality Improvements"
- List 4 user stories with acceptance criteria
- Reference spec.md and tasks.md
- Include before/after examples
- List breaking changes prominently
- Request reviews from maintainers

### T091 Final review and self-merge check
- Review all changed files
- Verify no debug code left behind (println!, dbg!, etc.)
- Verify no TODOs or FIXMEs added
- Check for any accidental commits
- Run final: `cargo nextest run --workspace --all-features`

### T092 Merge to main and tag release
- Merge PR to main
- Tag as v0.4.0: `git tag -a v0.4.0 -m "Release v0.4.0: Code Quality Improvements"`
- Push tag: `git push origin v0.4.0`
- Close related issues

---

## Implementation Notes

### TDD Discipline

**CRITICAL**: Follow Red-Green-Refactor cycle strictly:
1. **Red**: Write failing test first (T###[TEST] tasks)
2. **Green**: Write minimal code to pass test
3. **Refactor**: Clean up implementation

**Never** write implementation before tests for new functionality.

### Parallel Work

Tasks marked `[P]` can be worked on simultaneously:
- Different files = safe to parallelize
- Same file = must be sequential
- Test tasks can run parallel to unrelated implementation

### Feature Flags

Always respect feature gates:
- `#[cfg(feature = "validation")]` for validation shortcuts
- `#[cfg(feature = "events")]` for event-related changes
- `#[cfg(feature = "serde")]` for serialization

### Backward Compatibility

Breaking changes strategy:
- v0.4.0: Add `#[deprecated]` with helpful messages
- v0.5.0: Keep deprecated methods (warnings only)
- v0.6.0: Remove deprecated methods

### Performance Measurement

Benchmark requirements:
- Run on clean system (no background load)
- Run multiple times (5+ iterations)
- Accept 5% variance as noise
- Document hardware/environment

---

## Dependencies

**Critical Path**:
```
T001,T002 → T003→T004 → T007 [FOUNDATION]
         ↓
T008→T009→T010→T011→T012 [IMMUTABILITY] → T020
         ↓
T023→T024, T025→T026, T031→T032, T037→T038, T039→T040, T044→T046 [ERGONOMICS]
         ↓
T051→T052→T053, T055→T056→T058, T059→T060 [PERFORMANCE]
         ↓
T067→T068,T069,T070 → T073 [DOCS]
         ↓
T085→T086→T087 [VALIDATION]
```

**Estimated Timeline**:
- Phase 1 (Setup): 0.5 days
- Phase 2 (Foundation): 1 day
- Phase 3 (Immutability): 3 days
- Phase 4 (Ergonomics): 5 days
- Phase 5 (Performance): 3 days
- Phase 6 (Documentation): 4 days
- Phase 7 (Polish): 1 day

**Total**: ~17.5 days (3.5 weeks with buffer)

---

## Completion Checklist

Before marking feature complete:

- [ ] All 92 tasks completed
- [ ] All tests pass (all feature combinations)
- [ ] All benchmarks show expected improvements
- [ ] Zero clippy warnings
- [ ] All doc examples compile (95%+)
- [ ] CHANGELOG.md updated
- [ ] Migration guide complete
- [ ] Success criteria verified and documented
- [ ] PR created and reviewed
- [ ] Merged to main
- [ ] Tagged as v0.4.0

---

**Document Status**: Complete and ready for implementation  
**Next Step**: Begin Phase 1 (T001 - Create feature branch)  
**Estimated Completion**: 3.5 weeks from start

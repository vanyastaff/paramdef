## 1. Project Structure

- [ ] 1.1 Create `src/schema/mod.rs` module
- [ ] 1.2 Create `src/runtime/mod.rs` module
- [ ] 1.3 Create `src/runtime/state.rs` for ParameterState
- [ ] 1.4 Create `src/runtime/node.rs` for RuntimeNode
- [ ] 1.5 Create `src/context/mod.rs` module
- [ ] 1.6 Update `src/lib.rs` to export modules
- [ ] 1.7 Run `cargo check` to verify structure compiles

## 2. Schema Implementation

- [ ] 2.1 Write failing test: `test_schema_builder`
- [ ] 2.2 Implement Schema struct with Vec<Arc<dyn Node>>
- [ ] 2.3 Implement SchemaBuilder
- [ ] 2.4 Run test to verify it passes
- [ ] 2.5 Add get_parameter(key) method
- [ ] 2.6 Add parameters() iterator
- [ ] 2.7 Add documentation
- [ ] 2.8 Commit: `feat(schema): add Schema and builder`

## 3. ParameterState Implementation

- [ ] 3.1 Write failing test: `test_parameter_state_init`
- [ ] 3.2 Implement ParameterState with StateFlags, errors, modified_at
- [ ] 3.3 Run test to verify it passes
- [ ] 3.4 Add is_dirty(), is_touched(), is_valid() methods
- [ ] 3.5 Commit: `feat(runtime): add ParameterState`

## 4. RuntimeNode Implementation

- [ ] 4.1 Write failing test: `test_runtime_node_create`
- [ ] 4.2 Implement RuntimeNode<T: Node> with node: Arc<T>, state: ParameterState
- [ ] 4.3 Run test to verify it passes
- [ ] 4.4 Write failing test: `test_runtime_node_set_value`
- [ ] 4.5 Implement set_value with transform → validate → store → mark dirty
- [ ] 4.6 Run test to verify it passes
- [ ] 4.7 Add get_value() method
- [ ] 4.8 Add documentation
- [ ] 4.9 Commit: `feat(runtime): add RuntimeNode generic wrapper`

## 5. Context Implementation

- [ ] 5.1 Write failing test: `test_context_from_schema`
- [ ] 5.2 Implement Context with schema: Arc<Schema>, runtime_nodes: HashMap
- [ ] 5.3 Run test to verify it passes
- [ ] 5.4 Write failing test: `test_context_set_value`
- [ ] 5.5 Implement set_value with pipeline
- [ ] 5.6 Run test to verify it passes
- [ ] 5.7 Add get_value(), validate_all() methods
- [ ] 5.8 Add collect_values(), collect_dirty_values()
- [ ] 5.9 Add documentation
- [ ] 5.10 Commit: `feat(context): add Context runtime manager`

## 6. Integration Tests

- [ ] 6.1 Write integration test: `test_complete_workflow`
- [ ] 6.2 Test Schema → Context → set_value → get_value cycle
- [ ] 6.3 Run test to verify it passes
- [ ] 6.4 Write integration test: `test_validation_pipeline`
- [ ] 6.5 Test transform → validate → store flow
- [ ] 6.6 Run test to verify it passes
- [ ] 6.7 Commit: `test: add schema-runtime integration tests`

## 7. Final Verification

- [ ] 7.1 Run `cargo fmt --all`
- [ ] 7.2 Run `cargo clippy --workspace --all-features -- -D warnings`
- [ ] 7.3 Run `cargo test --workspace`
- [ ] 7.4 Run `cargo doc --no-deps --all-features`
- [ ] 7.5 Run `cargo +1.85 check --workspace` (MSRV)
- [ ] 7.6 Verify test coverage is 90%+
- [ ] 7.7 Commit: `chore: verify schema-runtime complete`

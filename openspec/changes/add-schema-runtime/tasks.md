## 1. Project Structure

- [x] 1.1 Create `src/schema/mod.rs` module
- [x] 1.2 Create `src/runtime/mod.rs` module
- [x] 1.3 Create `src/runtime/state.rs` for State
- [x] 1.4 Create `src/runtime/node.rs` for RuntimeNode
- [x] 1.5 Create `src/context/mod.rs` module
- [x] 1.6 Update `src/lib.rs` to export modules
- [x] 1.7 Run `cargo check` to verify structure compiles

## 2. Schema Implementation

- [x] 2.1 Write failing test: `test_schema_builder`
- [x] 2.2 Implement Schema struct with HashMap<Key, Arc<dyn Node>>
- [x] 2.3 Implement SchemaBuilder
- [x] 2.4 Run test to verify it passes
- [x] 2.5 Add get(key) method
- [x] 2.6 Add iter() and keys() iterators
- [x] 2.7 Add documentation

## 3. State Implementation

- [x] 3.1 Write failing test: `test_state_init`
- [x] 3.2 Implement State with StateFlags, errors, modified_at
- [x] 3.3 Run test to verify it passes
- [x] 3.4 Add is_dirty(), is_touched(), is_valid() methods

## 4. RuntimeNode Implementation

- [x] 4.1 Write failing test: `test_runtime_node_create`
- [x] 4.2 Implement RuntimeNode with node: Arc<dyn Node>, state: State
- [x] 4.3 Run test to verify it passes
- [x] 4.4 Write failing test: `test_runtime_node_set_value`
- [x] 4.5 Implement set_value with store → mark dirty
- [x] 4.6 Run test to verify it passes
- [x] 4.7 Add value() method
- [x] 4.8 Add documentation

## 5. Context Implementation

- [x] 5.1 Write failing test: `test_context_from_schema`
- [x] 5.2 Implement Context with schema: Arc<Schema>, nodes: HashMap
- [x] 5.3 Run test to verify it passes
- [x] 5.4 Write failing test: `test_context_set_value`
- [x] 5.5 Implement set() and get() methods
- [x] 5.6 Run test to verify it passes
- [x] 5.7 Add node(), node_mut() methods
- [x] 5.8 Add collect_values(), collect_dirty_values()
- [x] 5.9 Add documentation

## 6. Final Verification

- [x] 6.1 Run `cargo fmt --all`
- [x] 6.2 Run `cargo clippy --workspace --all-features -- -D warnings`
- [x] 6.3 Run `cargo test --workspace`
- [ ] 6.4 Run `cargo doc --no-deps --all-features`
- [ ] 6.5 Commit changes

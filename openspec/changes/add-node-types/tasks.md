## 1. Module Structure Setup

- [x] 1.1 Create `src/nodes/mod.rs`
- [x] 1.2 Create `src/nodes/traits.rs` for Node, ValueAccess, GroupNode, Layout, Decoration, Container, Leaf traits
- [x] 1.3 Create `src/nodes/kind.rs` for NodeKind enum
- [x] 1.4 Create `src/nodes/group.rs`
- [x] 1.5 Create `src/nodes/panel.rs`
- [x] 1.6 Create `src/nodes/notice.rs`
- [x] 1.7 Create `src/nodes/text.rs`
- [x] 1.8 Create `src/nodes/number.rs`
- [x] 1.9 Create `src/nodes/boolean.rs`
- [x] 1.10 Create `src/nodes/vector.rs`
- [x] 1.11 Create `src/nodes/select.rs`
- [x] 1.12 Create `src/nodes/object.rs`
- [x] 1.13 Create `src/nodes/list.rs`
- [x] 1.14 Create `src/nodes/mode.rs`
- [x] 1.15 Create `src/nodes/routing.rs`
- [x] 1.16 Create `src/nodes/expirable.rs`
- [x] 1.17 Create `src/nodes/reference.rs` (Ref)
- [x] 1.18 Update `src/lib.rs` to export nodes module
- [x] 1.19 Run `cargo check` to verify structure compiles

## 2. Node Base Trait

- [x] 2.1 Write failing test: `test_node_trait`
- [x] 2.2 Run test to verify it fails
- [x] 2.3 Define `Node` trait with metadata(), kind(), key()
- [x] 2.4 Run test to verify it passes
- [x] 2.5 Add Send + Sync bounds
- [x] 2.6 Add documentation
- [x] 2.7 Commit: `feat(nodes): add Node base trait`

## 3. NodeKind Enum

- [x] 3.1 Write failing test: `test_node_kind_variants`
- [x] 3.2 Implement NodeKind enum with 14 variants
- [x] 3.3 Run test to verify it passes
- [x] 3.4 Add documentation
- [x] 3.5 Commit: `feat(nodes): add NodeKind enum`

## 4. ValueAccess Trait

- [x] 4.1 Write failing test: `test_value_access_trait`
- [x] 4.2 Run test to verify it fails
- [x] 4.3 Define `ValueAccess` trait with collect_values, get_value, set_value
- [x] 4.4 Run test to verify it passes
- [x] 4.5 Add documentation
- [x] 4.6 Commit: `feat(nodes): add ValueAccess trait`

## 5. Category Traits

- [x] 5.1 Write failing test: `test_group_node_trait`
- [x] 5.2 Define `GroupNode: Node + ValueAccess` trait
- [x] 5.3 Run test to verify it passes
- [x] 5.4 Write failing test: `test_layout_trait`
- [x] 5.5 Define `Layout: Node + ValueAccess` trait
- [x] 5.6 Run test to verify it passes
- [x] 5.7 Write failing test: `test_decoration_trait`
- [x] 5.8 Define `Decoration: Node` trait (no ValueAccess)
- [x] 5.9 Run test to verify it passes
- [x] 5.10 Write failing test: `test_container_trait`
- [x] 5.11 Define `Container: Node + ValueAccess` trait with to_value, from_value
- [x] 5.12 Run test to verify it passes
- [x] 5.13 Write failing test: `test_leaf_trait`
- [x] 5.14 Define `Leaf: Node` trait with associated ValueType
- [x] 5.15 Run test to verify it passes
- [x] 5.16 Add documentation
- [x] 5.17 Commit: `feat(nodes): add category traits`

## 6. Text Leaf Implementation

- [x] 6.1 Write failing test: `test_text_creation`
- [x] 6.2 Implement Text struct with metadata, subtype fields
- [x] 6.3 Run test to verify it passes
- [x] 6.4 Write failing test: `test_text_builder`
- [x] 6.5 Implement TextBuilder
- [x] 6.6 Run test to verify it passes
- [x] 6.7 Write failing test: `test_text_to_value`
- [x] 6.8 Implement Leaf trait for Text
- [x] 6.9 Run test to verify it passes
- [x] 6.10 Implement Node trait for Text
- [x] 6.11 Add min_length, max_length, pattern fields
- [x] 6.12 Write tests for constraints
- [x] 6.13 Add documentation
- [x] 6.14 Commit: `feat(nodes): add Text leaf type`

## 7. Number Leaf Implementation

- [x] 7.1 Write failing test: `test_number_integer`
- [x] 7.2 Implement Number<T, S> struct with generics
- [x] 7.3 Run test to verify it passes
- [x] 7.4 Write failing test: `test_number_float`
- [x] 7.5 Verify Number<f64> works
- [x] 7.6 Run test to verify it passes
- [x] 7.7 Write failing test: `test_number_builder`
- [x] 7.8 Implement NumberBuilder
- [x] 7.9 Run test to verify it passes
- [x] 7.10 Write failing test: `test_number_to_value_int`
- [x] 7.11 Implement Leaf trait returning Value::Int for integers
- [x] 7.12 Run test to verify it passes
- [x] 7.13 Write failing test: `test_number_to_value_float`
- [x] 7.14 Implement Leaf trait returning Value::Float for floats
- [x] 7.15 Run test to verify it passes
- [x] 7.16 Add hard_min, hard_max, soft_min, soft_max, step, unit fields
- [x] 7.17 Add documentation
- [x] 7.18 Commit: `feat(nodes): add Number leaf type`

## 8. Boolean Leaf Implementation

- [x] 8.1 Write failing test: `test_boolean_creation`
- [x] 8.2 Implement Boolean struct
- [x] 8.3 Run test to verify it passes
- [x] 8.4 Write failing test: `test_boolean_builder`
- [x] 8.5 Implement BooleanBuilder
- [x] 8.6 Run test to verify it passes
- [x] 8.7 Write failing test: `test_boolean_to_value`
- [x] 8.8 Implement Leaf trait for Boolean
- [x] 8.9 Run test to verify it passes
- [x] 8.10 Add documentation
- [x] 8.11 Commit: `feat(nodes): add Boolean leaf type`

## 9. Vector Leaf Implementation

- [x] 9.1 Write failing test: `test_vector_creation`
- [x] 9.2 Implement Vector<T, N, S> struct with const generics
- [x] 9.3 Run test to verify it passes
- [x] 9.4 Write failing test: `test_vector_builder`
- [x] 9.5 Implement VectorBuilder
- [x] 9.6 Run test to verify it passes
- [x] 9.7 Write failing test: `test_vector_subtype_constraint`
- [x] 9.8 Verify VectorSubtype<N> constraint works
- [x] 9.9 Run test to verify it passes
- [x] 9.10 Write failing test: `test_vector_to_value`
- [x] 9.11 Implement Leaf trait for Vector
- [x] 9.12 Run test to verify it passes
- [x] 9.13 Add component_units field
- [x] 9.14 Add documentation
- [x] 9.15 Commit: `feat(nodes): add Vector leaf type`

## 10. Select Leaf Implementation

- [x] 10.1 Write failing test: `test_select_single`
- [x] 10.2 Implement Select struct with SelectionMode, OptionSource
- [x] 10.3 Run test to verify it passes
- [x] 10.4 Write failing test: `test_select_multiple`
- [x] 10.5 Verify multiple selection works
- [x] 10.6 Run test to verify it passes
- [x] 10.7 Write failing test: `test_select_static_options`
- [x] 10.8 Implement static_options() builder method
- [x] 10.9 Run test to verify it passes
- [x] 10.10 Write failing test: `test_select_dynamic_options`
- [x] 10.11 Implement dynamic_options() with OptionLoader trait
- [x] 10.12 Run test to verify it passes
- [x] 10.13 Implement SelectOption struct
- [x] 10.14 Write failing test: `test_select_to_value_single`
- [x] 10.15 Implement Leaf trait for single select (Value::Text)
- [x] 10.16 Run test to verify it passes
- [x] 10.17 Write failing test: `test_select_to_value_multiple`
- [x] 10.18 Implement Leaf trait for multiple select (Value::Array)
- [x] 10.19 Run test to verify it passes
- [x] 10.20 Add searchable, creatable fields
- [x] 10.21 Add documentation
- [x] 10.22 Commit: `feat(nodes): add Select leaf type`

## 11. Object Container Implementation

- [x] 11.1 Write failing test: `test_object_creation`
- [x] 11.2 Implement Object struct
- [x] 11.3 Run test to verify it passes
- [x] 11.4 Write failing test: `test_object_builder`
- [x] 11.5 Implement ObjectBuilder with field() method
- [x] 11.6 Run test to verify it passes
- [x] 11.7 Write failing test: `test_object_value_access`
- [x] 11.8 Implement ValueAccess for Object
- [x] 11.9 Run test to verify it passes
- [x] 11.10 Write failing test: `test_object_to_value`
- [x] 11.11 Implement Container trait for Object
- [x] 11.12 Run test to verify it passes
- [x] 11.13 Add documentation
- [x] 11.14 Commit: `feat(nodes): add Object container type`

## 12. List Container Implementation

- [x] 12.1 Write failing test: `test_list_creation`
- [x] 12.2 Implement List struct with item_template
- [x] 12.3 Run test to verify it passes
- [x] 12.4 Write failing test: `test_list_builder`
- [x] 12.5 Implement ListBuilder
- [x] 12.6 Run test to verify it passes
- [x] 12.7 Write failing test: `test_list_constraints`
- [x] 12.8 Add min_items, max_items, unique, sortable fields
- [x] 12.9 Run test to verify it passes
- [x] 12.10 Write failing test: `test_list_to_value`
- [x] 12.11 Implement Container trait for List
- [x] 12.12 Run test to verify it passes
- [x] 12.13 Add documentation
- [x] 12.14 Commit: `feat(nodes): add List container type`

## 13. Mode Container Implementation

- [x] 13.1 Write failing test: `test_mode_creation`
- [x] 13.2 Implement Mode struct with ModeVariant
- [x] 13.3 Run test to verify it passes
- [x] 13.4 Write failing test: `test_mode_builder`
- [x] 13.5 Implement ModeBuilder with variant() method
- [x] 13.6 Run test to verify it passes
- [x] 13.7 Write failing test: `test_mode_to_value`
- [x] 13.8 Implement Container trait producing { mode, value } object
- [x] 13.9 Run test to verify it passes
- [x] 13.10 Write failing test: `test_mode_default_variant`
- [x] 13.11 Implement default_variant() method
- [x] 13.12 Run test to verify it passes
- [x] 13.13 Add documentation
- [x] 13.14 Commit: `feat(nodes): add Mode container type`

## 14. Routing Container Implementation

- [x] 14.1 Write failing test: `test_routing_creation`
- [x] 14.2 Implement Routing struct with RoutingOptions
- [x] 14.3 Run test to verify it passes
- [x] 14.4 Write failing test: `test_routing_builder`
- [x] 14.5 Implement RoutingBuilder
- [x] 14.6 Run test to verify it passes
- [x] 14.7 Write failing test: `test_routing_to_value`
- [x] 14.8 Implement Container trait for Routing
- [x] 14.9 Run test to verify it passes
- [x] 14.10 Add documentation
- [x] 14.11 Commit: `feat(nodes): add Routing container type`

## 15. Expirable Container Implementation

- [x] 15.1 Write failing test: `test_expirable_creation`
- [x] 15.2 Implement Expirable struct with ExpirableOptions
- [x] 15.3 Run test to verify it passes
- [x] 15.4 Write failing test: `test_expirable_builder`
- [x] 15.5 Implement ExpirableBuilder with ttl methods
- [x] 15.6 Run test to verify it passes
- [x] 15.7 Write failing test: `test_expirable_to_value`
- [x] 15.8 Implement Container trait producing { value, expires_at, created_at }
- [x] 15.9 Run test to verify it passes
- [x] 15.10 Add chrono feature gate for timestamp handling
- [x] 15.11 Add documentation
- [x] 15.12 Commit: `feat(nodes): add Expirable container type`

## 16. Ref Container Implementation

- [x] 16.1 Write failing test: `test_ref_creation`
- [x] 16.2 Implement Ref struct with target Key
- [x] 16.3 Run test to verify it passes
- [x] 16.4 Write failing test: `test_ref_builder`
- [x] 16.5 Implement RefBuilder
- [x] 16.6 Run test to verify it passes
- [x] 16.7 Implement Node trait (delegates to target)
- [x] 16.8 Add documentation
- [x] 16.9 Commit: `feat(nodes): add Ref container type`

## 17. Notice Decoration Implementation

- [x] 17.1 Write failing test: `test_notice_creation`
- [x] 17.2 Implement Notice struct with NoticeType enum
- [x] 17.3 Run test to verify it passes
- [x] 17.4 Write failing test: `test_notice_builder`
- [x] 17.5 Implement NoticeBuilder
- [x] 17.6 Run test to verify it passes
- [x] 17.7 Implement Decoration trait
- [x] 17.8 Write failing test: `test_notice_no_value`
- [x] 17.9 Verify Notice does NOT implement Leaf or Container
- [x] 17.10 Add documentation
- [x] 17.11 Commit: `feat(nodes): add Notice decoration type`

## 18. Panel Layout Implementation

- [x] 18.1 Write failing test: `test_panel_creation`
- [x] 18.2 Implement Panel struct
- [x] 18.3 Run test to verify it passes
- [x] 18.4 Write failing test: `test_panel_builder`
- [x] 18.5 Implement PanelBuilder with child() method
- [x] 18.6 Run test to verify it passes
- [x] 18.7 Write failing test: `test_panel_value_access`
- [x] 18.8 Implement ValueAccess for Panel
- [x] 18.9 Run test to verify it passes
- [x] 18.10 Implement Layout trait
- [x] 18.11 Add PanelDisplayType enum
- [x] 18.12 Add documentation
- [x] 18.13 Commit: `feat(nodes): add Panel layout type`

## 19. Group Implementation

- [x] 19.1 Write failing test: `test_group_creation`
- [x] 19.2 Implement Group struct
- [x] 19.3 Run test to verify it passes
- [x] 19.4 Write failing test: `test_group_builder`
- [x] 19.5 Implement GroupBuilder with child() method
- [x] 19.6 Run test to verify it passes
- [x] 19.7 Write failing test: `test_group_value_access`
- [x] 19.8 Implement ValueAccess for Group
- [x] 19.9 Run test to verify it passes
- [x] 19.10 Implement GroupNode trait
- [x] 19.11 Write failing test: `test_group_contains_panel`
- [x] 19.12 Verify Group can contain Panel
- [x] 19.13 Run test to verify it passes
- [x] 19.14 Add GroupLayout enum
- [x] 19.15 Add documentation
- [x] 19.16 Commit: `feat(nodes): add Group type`

## 20. Final Verification

- [x] 20.1 Run `cargo fmt --all`
- [x] 20.2 Run `cargo clippy --workspace --all-features -- -D warnings`
- [x] 20.3 Run `cargo test --workspace`
- [x] 20.4 Run `cargo doc --no-deps --all-features`
- [x] 20.5 Run `cargo +1.85 check --workspace` (MSRV)
- [x] 20.6 Verify test coverage is 90%+
- [x] 20.7 Commit: `chore: verify Phase 3 complete`

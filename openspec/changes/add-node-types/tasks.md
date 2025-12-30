## 1. Module Structure Setup

- [ ] 1.1 Create `src/nodes/mod.rs`
- [ ] 1.2 Create `src/nodes/traits.rs` for Node, ValueAccess, GroupNode, Layout, Decoration, Container, Leaf traits
- [ ] 1.3 Create `src/nodes/kind.rs` for NodeKind enum
- [ ] 1.4 Create `src/nodes/group.rs`
- [ ] 1.5 Create `src/nodes/panel.rs`
- [ ] 1.6 Create `src/nodes/notice.rs`
- [ ] 1.7 Create `src/nodes/text.rs`
- [ ] 1.8 Create `src/nodes/number.rs`
- [ ] 1.9 Create `src/nodes/boolean.rs`
- [ ] 1.10 Create `src/nodes/vector.rs`
- [ ] 1.11 Create `src/nodes/select.rs`
- [ ] 1.12 Create `src/nodes/object.rs`
- [ ] 1.13 Create `src/nodes/list.rs`
- [ ] 1.14 Create `src/nodes/mode.rs`
- [ ] 1.15 Create `src/nodes/routing.rs`
- [ ] 1.16 Create `src/nodes/expirable.rs`
- [ ] 1.17 Create `src/nodes/reference.rs` (Ref)
- [ ] 1.18 Update `src/lib.rs` to export nodes module
- [ ] 1.19 Run `cargo check` to verify structure compiles

## 2. Node Base Trait

- [ ] 2.1 Write failing test: `test_node_trait`
- [ ] 2.2 Run test to verify it fails
- [ ] 2.3 Define `Node` trait with metadata(), kind(), key()
- [ ] 2.4 Run test to verify it passes
- [ ] 2.5 Add Send + Sync bounds
- [ ] 2.6 Add documentation
- [ ] 2.7 Commit: `feat(nodes): add Node base trait`

## 3. NodeKind Enum

- [ ] 3.1 Write failing test: `test_node_kind_variants`
- [ ] 3.2 Implement NodeKind enum with 14 variants
- [ ] 3.3 Run test to verify it passes
- [ ] 3.4 Add documentation
- [ ] 3.5 Commit: `feat(nodes): add NodeKind enum`

## 4. ValueAccess Trait

- [ ] 4.1 Write failing test: `test_value_access_trait`
- [ ] 4.2 Run test to verify it fails
- [ ] 4.3 Define `ValueAccess` trait with collect_values, get_value, set_value
- [ ] 4.4 Run test to verify it passes
- [ ] 4.5 Add documentation
- [ ] 4.6 Commit: `feat(nodes): add ValueAccess trait`

## 5. Category Traits

- [ ] 5.1 Write failing test: `test_group_node_trait`
- [ ] 5.2 Define `GroupNode: Node + ValueAccess` trait
- [ ] 5.3 Run test to verify it passes
- [ ] 5.4 Write failing test: `test_layout_trait`
- [ ] 5.5 Define `Layout: Node + ValueAccess` trait
- [ ] 5.6 Run test to verify it passes
- [ ] 5.7 Write failing test: `test_decoration_trait`
- [ ] 5.8 Define `Decoration: Node` trait (no ValueAccess)
- [ ] 5.9 Run test to verify it passes
- [ ] 5.10 Write failing test: `test_container_trait`
- [ ] 5.11 Define `Container: Node + ValueAccess` trait with to_value, from_value
- [ ] 5.12 Run test to verify it passes
- [ ] 5.13 Write failing test: `test_leaf_trait`
- [ ] 5.14 Define `Leaf: Node` trait with associated ValueType
- [ ] 5.15 Run test to verify it passes
- [ ] 5.16 Add documentation
- [ ] 5.17 Commit: `feat(nodes): add category traits`

## 6. Text Leaf Implementation

- [ ] 6.1 Write failing test: `test_text_creation`
- [ ] 6.2 Implement Text struct with metadata, subtype fields
- [ ] 6.3 Run test to verify it passes
- [ ] 6.4 Write failing test: `test_text_builder`
- [ ] 6.5 Implement TextBuilder
- [ ] 6.6 Run test to verify it passes
- [ ] 6.7 Write failing test: `test_text_to_value`
- [ ] 6.8 Implement Leaf trait for Text
- [ ] 6.9 Run test to verify it passes
- [ ] 6.10 Implement Node trait for Text
- [ ] 6.11 Add min_length, max_length, pattern fields
- [ ] 6.12 Write tests for constraints
- [ ] 6.13 Add documentation
- [ ] 6.14 Commit: `feat(nodes): add Text leaf type`

## 7. Number Leaf Implementation

- [ ] 7.1 Write failing test: `test_number_integer`
- [ ] 7.2 Implement Number<T, S> struct with generics
- [ ] 7.3 Run test to verify it passes
- [ ] 7.4 Write failing test: `test_number_float`
- [ ] 7.5 Verify Number<f64> works
- [ ] 7.6 Run test to verify it passes
- [ ] 7.7 Write failing test: `test_number_builder`
- [ ] 7.8 Implement NumberBuilder
- [ ] 7.9 Run test to verify it passes
- [ ] 7.10 Write failing test: `test_number_to_value_int`
- [ ] 7.11 Implement Leaf trait returning Value::Int for integers
- [ ] 7.12 Run test to verify it passes
- [ ] 7.13 Write failing test: `test_number_to_value_float`
- [ ] 7.14 Implement Leaf trait returning Value::Float for floats
- [ ] 7.15 Run test to verify it passes
- [ ] 7.16 Add hard_min, hard_max, soft_min, soft_max, step, unit fields
- [ ] 7.17 Add documentation
- [ ] 7.18 Commit: `feat(nodes): add Number leaf type`

## 8. Boolean Leaf Implementation

- [ ] 8.1 Write failing test: `test_boolean_creation`
- [ ] 8.2 Implement Boolean struct
- [ ] 8.3 Run test to verify it passes
- [ ] 8.4 Write failing test: `test_boolean_builder`
- [ ] 8.5 Implement BooleanBuilder
- [ ] 8.6 Run test to verify it passes
- [ ] 8.7 Write failing test: `test_boolean_to_value`
- [ ] 8.8 Implement Leaf trait for Boolean
- [ ] 8.9 Run test to verify it passes
- [ ] 8.10 Add documentation
- [ ] 8.11 Commit: `feat(nodes): add Boolean leaf type`

## 9. Vector Leaf Implementation

- [ ] 9.1 Write failing test: `test_vector_creation`
- [ ] 9.2 Implement Vector<T, N, S> struct with const generics
- [ ] 9.3 Run test to verify it passes
- [ ] 9.4 Write failing test: `test_vector_builder`
- [ ] 9.5 Implement VectorBuilder
- [ ] 9.6 Run test to verify it passes
- [ ] 9.7 Write failing test: `test_vector_subtype_constraint`
- [ ] 9.8 Verify VectorSubtype<N> constraint works
- [ ] 9.9 Run test to verify it passes
- [ ] 9.10 Write failing test: `test_vector_to_value`
- [ ] 9.11 Implement Leaf trait for Vector
- [ ] 9.12 Run test to verify it passes
- [ ] 9.13 Add component_units field
- [ ] 9.14 Add documentation
- [ ] 9.15 Commit: `feat(nodes): add Vector leaf type`

## 10. Select Leaf Implementation

- [ ] 10.1 Write failing test: `test_select_single`
- [ ] 10.2 Implement Select struct with SelectionMode, OptionSource
- [ ] 10.3 Run test to verify it passes
- [ ] 10.4 Write failing test: `test_select_multiple`
- [ ] 10.5 Verify multiple selection works
- [ ] 10.6 Run test to verify it passes
- [ ] 10.7 Write failing test: `test_select_static_options`
- [ ] 10.8 Implement static_options() builder method
- [ ] 10.9 Run test to verify it passes
- [ ] 10.10 Write failing test: `test_select_dynamic_options`
- [ ] 10.11 Implement dynamic_options() with OptionLoader trait
- [ ] 10.12 Run test to verify it passes
- [ ] 10.13 Implement SelectOption struct
- [ ] 10.14 Write failing test: `test_select_to_value_single`
- [ ] 10.15 Implement Leaf trait for single select (Value::Text)
- [ ] 10.16 Run test to verify it passes
- [ ] 10.17 Write failing test: `test_select_to_value_multiple`
- [ ] 10.18 Implement Leaf trait for multiple select (Value::Array)
- [ ] 10.19 Run test to verify it passes
- [ ] 10.20 Add searchable, creatable fields
- [ ] 10.21 Add documentation
- [ ] 10.22 Commit: `feat(nodes): add Select leaf type`

## 11. Object Container Implementation

- [ ] 11.1 Write failing test: `test_object_creation`
- [ ] 11.2 Implement Object struct
- [ ] 11.3 Run test to verify it passes
- [ ] 11.4 Write failing test: `test_object_builder`
- [ ] 11.5 Implement ObjectBuilder with field() method
- [ ] 11.6 Run test to verify it passes
- [ ] 11.7 Write failing test: `test_object_value_access`
- [ ] 11.8 Implement ValueAccess for Object
- [ ] 11.9 Run test to verify it passes
- [ ] 11.10 Write failing test: `test_object_to_value`
- [ ] 11.11 Implement Container trait for Object
- [ ] 11.12 Run test to verify it passes
- [ ] 11.13 Add documentation
- [ ] 11.14 Commit: `feat(nodes): add Object container type`

## 12. List Container Implementation

- [ ] 12.1 Write failing test: `test_list_creation`
- [ ] 12.2 Implement List struct with item_template
- [ ] 12.3 Run test to verify it passes
- [ ] 12.4 Write failing test: `test_list_builder`
- [ ] 12.5 Implement ListBuilder
- [ ] 12.6 Run test to verify it passes
- [ ] 12.7 Write failing test: `test_list_constraints`
- [ ] 12.8 Add min_items, max_items, unique, sortable fields
- [ ] 12.9 Run test to verify it passes
- [ ] 12.10 Write failing test: `test_list_to_value`
- [ ] 12.11 Implement Container trait for List
- [ ] 12.12 Run test to verify it passes
- [ ] 12.13 Add documentation
- [ ] 12.14 Commit: `feat(nodes): add List container type`

## 13. Mode Container Implementation

- [ ] 13.1 Write failing test: `test_mode_creation`
- [ ] 13.2 Implement Mode struct with ModeVariant
- [ ] 13.3 Run test to verify it passes
- [ ] 13.4 Write failing test: `test_mode_builder`
- [ ] 13.5 Implement ModeBuilder with variant() method
- [ ] 13.6 Run test to verify it passes
- [ ] 13.7 Write failing test: `test_mode_to_value`
- [ ] 13.8 Implement Container trait producing { mode, value } object
- [ ] 13.9 Run test to verify it passes
- [ ] 13.10 Write failing test: `test_mode_default_variant`
- [ ] 13.11 Implement default_variant() method
- [ ] 13.12 Run test to verify it passes
- [ ] 13.13 Add documentation
- [ ] 13.14 Commit: `feat(nodes): add Mode container type`

## 14. Routing Container Implementation

- [ ] 14.1 Write failing test: `test_routing_creation`
- [ ] 14.2 Implement Routing struct with RoutingOptions
- [ ] 14.3 Run test to verify it passes
- [ ] 14.4 Write failing test: `test_routing_builder`
- [ ] 14.5 Implement RoutingBuilder
- [ ] 14.6 Run test to verify it passes
- [ ] 14.7 Write failing test: `test_routing_to_value`
- [ ] 14.8 Implement Container trait for Routing
- [ ] 14.9 Run test to verify it passes
- [ ] 14.10 Add documentation
- [ ] 14.11 Commit: `feat(nodes): add Routing container type`

## 15. Expirable Container Implementation

- [ ] 15.1 Write failing test: `test_expirable_creation`
- [ ] 15.2 Implement Expirable struct with ExpirableOptions
- [ ] 15.3 Run test to verify it passes
- [ ] 15.4 Write failing test: `test_expirable_builder`
- [ ] 15.5 Implement ExpirableBuilder with ttl methods
- [ ] 15.6 Run test to verify it passes
- [ ] 15.7 Write failing test: `test_expirable_to_value`
- [ ] 15.8 Implement Container trait producing { value, expires_at, created_at }
- [ ] 15.9 Run test to verify it passes
- [ ] 15.10 Add chrono feature gate for timestamp handling
- [ ] 15.11 Add documentation
- [ ] 15.12 Commit: `feat(nodes): add Expirable container type`

## 16. Ref Container Implementation

- [ ] 16.1 Write failing test: `test_ref_creation`
- [ ] 16.2 Implement Ref struct with target Key
- [ ] 16.3 Run test to verify it passes
- [ ] 16.4 Write failing test: `test_ref_builder`
- [ ] 16.5 Implement RefBuilder
- [ ] 16.6 Run test to verify it passes
- [ ] 16.7 Implement Node trait (delegates to target)
- [ ] 16.8 Add documentation
- [ ] 16.9 Commit: `feat(nodes): add Ref container type`

## 17. Notice Decoration Implementation

- [ ] 17.1 Write failing test: `test_notice_creation`
- [ ] 17.2 Implement Notice struct with NoticeType enum
- [ ] 17.3 Run test to verify it passes
- [ ] 17.4 Write failing test: `test_notice_builder`
- [ ] 17.5 Implement NoticeBuilder
- [ ] 17.6 Run test to verify it passes
- [ ] 17.7 Implement Decoration trait
- [ ] 17.8 Write failing test: `test_notice_no_value`
- [ ] 17.9 Verify Notice does NOT implement Leaf or Container
- [ ] 17.10 Add documentation
- [ ] 17.11 Commit: `feat(nodes): add Notice decoration type`

## 18. Panel Layout Implementation

- [ ] 18.1 Write failing test: `test_panel_creation`
- [ ] 18.2 Implement Panel struct
- [ ] 18.3 Run test to verify it passes
- [ ] 18.4 Write failing test: `test_panel_builder`
- [ ] 18.5 Implement PanelBuilder with child() method
- [ ] 18.6 Run test to verify it passes
- [ ] 18.7 Write failing test: `test_panel_value_access`
- [ ] 18.8 Implement ValueAccess for Panel
- [ ] 18.9 Run test to verify it passes
- [ ] 18.10 Implement Layout trait
- [ ] 18.11 Add PanelDisplayType enum
- [ ] 18.12 Add documentation
- [ ] 18.13 Commit: `feat(nodes): add Panel layout type`

## 19. Group Implementation

- [ ] 19.1 Write failing test: `test_group_creation`
- [ ] 19.2 Implement Group struct
- [ ] 19.3 Run test to verify it passes
- [ ] 19.4 Write failing test: `test_group_builder`
- [ ] 19.5 Implement GroupBuilder with child() method
- [ ] 19.6 Run test to verify it passes
- [ ] 19.7 Write failing test: `test_group_value_access`
- [ ] 19.8 Implement ValueAccess for Group
- [ ] 19.9 Run test to verify it passes
- [ ] 19.10 Implement GroupNode trait
- [ ] 19.11 Write failing test: `test_group_contains_panel`
- [ ] 19.12 Verify Group can contain Panel
- [ ] 19.13 Run test to verify it passes
- [ ] 19.14 Add GroupLayout enum
- [ ] 19.15 Add documentation
- [ ] 19.16 Commit: `feat(nodes): add Group type`

## 20. Final Verification

- [ ] 20.1 Run `cargo fmt --all`
- [ ] 20.2 Run `cargo clippy --workspace --all-features -- -D warnings`
- [ ] 20.3 Run `cargo test --workspace`
- [ ] 20.4 Run `cargo doc --no-deps --all-features`
- [ ] 20.5 Run `cargo +1.85 check --workspace` (MSRV)
- [ ] 20.6 Verify test coverage is 90%+
- [ ] 20.7 Commit: `chore: verify Phase 3 complete`

# Capability: Node Trait System

## ADDED Requirements

### Requirement: Base Node Trait
The system SHALL provide a Node trait as the foundation for all 14 node types, defining metadata(), key(), and kind() methods.

#### Scenario: Get node metadata
- **WHEN** calling metadata() on any Node
- **THEN** it returns a reference to the node's Metadata

#### Scenario: Get node key
- **WHEN** calling key() on any Node
- **THEN** it returns the unique Key identifier

#### Scenario: Determine node kind
- **WHEN** calling kind() on any Node
- **THEN** it returns the NodeKind enum variant (Group, Layout, Decoration, Container, or Leaf)

### Requirement: ValueAccess Trait for Child Access
The system SHALL provide a ValueAccess trait for nodes that can access child values (Group, Layout, Container), with methods collect_values(), set_values(), get_value(), and set_value().

#### Scenario: Collect all child values
- **WHEN** calling collect_values() on a node with children
- **THEN** it returns a HashMap<Key, Value> with all descendant values

#### Scenario: Set a specific child value
- **WHEN** calling set_value(key, value) on a parent node
- **THEN** the value is propagated to the correct child by key

### Requirement: GroupNode Trait for Root Aggregators
The system SHALL provide a GroupNode trait for the Group type, which can contain Layout, Decoration, Container, and Leaf nodes.

#### Scenario: Access group children
- **WHEN** calling children() on a GroupNode
- **THEN** it returns all child nodes as &[Arc<dyn Node>]

#### Scenario: Group has ValueAccess
- **WHEN** using a GroupNode
- **THEN** it implements ValueAccess to collect values from all children

### Requirement: Layout Trait for UI Organization
The system SHALL provide a Layout trait for the Panel type, which organizes UI elements without its own value.

#### Scenario: Access panel children
- **WHEN** calling children() on a Layout (Panel)
- **THEN** it returns children (Decoration, Container, Leaf only - NOT Group or Layout)

#### Scenario: Layout has ValueAccess
- **WHEN** using a Layout
- **THEN** it implements ValueAccess but has no own value

### Requirement: Decoration Trait for Display-Only Nodes
The system SHALL provide a Decoration trait for the Notice type, which has no value and no children.

#### Scenario: Get decoration type
- **WHEN** calling decoration_type() on a Decoration (Notice)
- **THEN** it returns Info, Warning, Error, or Success

#### Scenario: Check if dismissible
- **WHEN** calling is_dismissible() on a Decoration
- **THEN** it returns a boolean indicating if the notice can be dismissed

### Requirement: Container Trait for Data Structures
The system SHALL provide a Container trait for Object, List, Mode, Routing, Expirable, and Ref types, which have their own value AND children.

#### Scenario: Convert container to value
- **WHEN** calling to_value() on a Container
- **THEN** it returns Value::Object or Value::Array representing the container's data

#### Scenario: Populate container from value
- **WHEN** calling from_value(value) on a Container
- **THEN** it updates the container state and validates the structure

#### Scenario: Validate container
- **WHEN** calling validate() on a Container (with validation feature)
- **THEN** it returns ValidationResult for both the container and its children

### Requirement: Leaf Trait for Terminal Values
The system SHALL provide a Leaf trait for Text, Number, Boolean, Vector, and Select types, which have a value but no children.

#### Scenario: Get typed value
- **WHEN** calling get_value() on a Leaf
- **THEN** it returns Option<&ValueType> with the strongly-typed value

#### Scenario: Set typed value
- **WHEN** calling set_value(value) on a Leaf
- **THEN** it validates and stores the typed value

#### Scenario: Convert to Value enum
- **WHEN** calling to_value() on a Leaf
- **THEN** it returns the unified Value enum representation

### Requirement: Visibility Trait (Feature-Gated)
The system SHALL provide a Visibility trait (when visibility feature is enabled) for all 14 node types to support conditional visibility based on Expr evaluation.

#### Scenario: Check visibility expression
- **WHEN** calling visibility() on any Node (with visibility feature)
- **THEN** it returns Option<&Expr> if visibility conditions are defined

#### Scenario: Evaluate visibility
- **WHEN** calling is_visible(context) on any Node
- **THEN** it evaluates the Expr and returns bool

#### Scenario: Get visibility dependencies
- **WHEN** calling dependencies() on a Node with visibility
- **THEN** it returns &[Key] of all parameters referenced in the Expr

### Requirement: Validatable Trait (Feature-Gated)
The system SHALL provide a Validatable trait (when validation feature is enabled) for Container and Leaf types (10 out of 14 nodes) that have their own value.

#### Scenario: Validate synchronously
- **WHEN** calling validate_sync(value) on a Validatable node
- **THEN** it runs all sync validators and returns Result<(), Error>

#### Scenario: Validate asynchronously
- **WHEN** calling validate_async(value) on a Validatable node
- **THEN** it runs all async validators (debounced) and returns Result<(), Error>

#### Scenario: Check if value is empty
- **WHEN** calling is_empty(value) on a Validatable node
- **THEN** it returns bool for empty check (required validation)

### Requirement: Trait Hierarchy Invariants
The system SHALL enforce strict trait implementation rules: Group and Layout have NO own Value but HAVE ValueAccess; Decoration has NO Value and NO ValueAccess; Container and Leaf have own Value, only Container has ValueAccess.

#### Scenario: Group implements ValueAccess without own value
- **WHEN** using a Group
- **THEN** it implements ValueAccess but calling to_value() is not available

#### Scenario: Container implements both ValueAccess and has own value
- **WHEN** using a Container (Object, List, Mode)
- **THEN** it implements ValueAccess AND can call to_value()

#### Scenario: Leaf has value but not ValueAccess
- **WHEN** using a Leaf (Text, Number, Boolean)
- **THEN** it can call to_value() but NOT collect_values()

#### Scenario: Decoration implements neither
- **WHEN** using a Decoration (Notice)
- **THEN** it implements neither ValueAccess nor to_value()

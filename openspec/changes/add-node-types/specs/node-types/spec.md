## ADDED Requirements

### Requirement: Node Base Trait
The system SHALL provide a `Node` trait as the base for all 14 node types.

#### Scenario: Node provides metadata
- **WHEN** any node's metadata() is called
- **THEN** it returns the node's Metadata struct

#### Scenario: Node provides kind
- **WHEN** any node's kind() is called
- **THEN** it returns the NodeKind enum variant

#### Scenario: Node provides key
- **WHEN** any node's key() is called
- **THEN** it returns the node's unique Key identifier

---

### Requirement: ValueAccess Trait
The system SHALL provide a `ValueAccess` trait for nodes that can contain children (Group, Layout, Container).

#### Scenario: Collect all values
- **WHEN** collect_values() is called on a node with children
- **THEN** it returns a HashMap of all descendant values

#### Scenario: Get specific value
- **WHEN** get_value(key) is called
- **THEN** it returns Option<&Value> for the specified key

#### Scenario: Set specific value
- **WHEN** set_value(key, value) is called
- **THEN** it updates the value at the specified key

---

### Requirement: Group Node Type
The system SHALL provide a `Group` type as the root aggregator that can contain all other node types.

#### Scenario: Group contains Panel
- **WHEN** a Group is created with Panel children
- **THEN** the Panels are accessible via children()

#### Scenario: Group contains Container
- **WHEN** a Group is created with Container children
- **THEN** the Containers are accessible via children()

#### Scenario: Group has no own value
- **WHEN** a Group's to_value() is called
- **THEN** it returns None (delegates to children via ValueAccess)

#### Scenario: Group builder pattern
- **WHEN** Group::builder("settings").child(...).build() is called
- **THEN** it creates a configured Group

---

### Requirement: Panel Layout Type
The system SHALL provide a `Panel` type for UI organization (tabs, sections).

#### Scenario: Panel contains Leaf nodes
- **WHEN** a Panel is created with Leaf children
- **THEN** the Leaves are accessible via children()

#### Scenario: Panel cannot contain Group
- **WHEN** a Panel tries to contain a Group
- **THEN** compilation fails (type system prevents it)

#### Scenario: Panel has no own value
- **WHEN** a Panel's to_value() is called
- **THEN** it returns None (delegates to children)

---

### Requirement: Notice Decoration Type
The system SHALL provide a `Notice` type for display-only messages (info, warning, error, success).

#### Scenario: Notice types
- **WHEN** a Notice is created with NoticeType::Warning
- **THEN** notice_type() returns Warning

#### Scenario: Notice has no value
- **WHEN** a Notice exists
- **THEN** it does not implement to_value() or ValueAccess

#### Scenario: Notice is dismissible
- **WHEN** a Notice is created with dismissible(true)
- **THEN** is_dismissible() returns true

---

### Requirement: Text Leaf Type
The system SHALL provide a `Text` type for string-based parameters.

#### Scenario: Text with subtype
- **WHEN** Text is created with subtype(TextSubtype::Email)
- **THEN** the subtype is accessible

#### Scenario: Text produces Value::Text
- **WHEN** Text's to_value() is called with a value
- **THEN** it returns Value::Text

#### Scenario: Text builder with constraints
- **WHEN** Text::builder("name").min_length(1).max_length(100).build() is called
- **THEN** it creates a Text with length constraints

---

### Requirement: Number Leaf Type
The system SHALL provide a generic `Number<T, S>` type for numeric parameters with compile-time subtype safety.

#### Scenario: Number with integer type
- **WHEN** Number::<i64>::builder("count").build() is called
- **THEN** it creates an integer Number

#### Scenario: Number with float type
- **WHEN** Number::<f64>::builder("opacity").build() is called
- **THEN** it creates a float Number

#### Scenario: Number produces correct Value variant
- **WHEN** Number<i64>'s to_value() is called
- **THEN** it returns Value::Int

#### Scenario: Number produces Float for f64
- **WHEN** Number<f64>'s to_value() is called
- **THEN** it returns Value::Float

---

### Requirement: Boolean Leaf Type
The system SHALL provide a `Boolean` type for true/false parameters.

#### Scenario: Boolean default value
- **WHEN** Boolean::builder("enabled").default(true).build() is called
- **THEN** the default is true

#### Scenario: Boolean produces Value::Bool
- **WHEN** Boolean's to_value() is called
- **THEN** it returns Value::Bool

---

### Requirement: Vector Leaf Type
The system SHALL provide a generic `Vector<T, N, S>` type for fixed-size numeric arrays with compile-time size safety.

#### Scenario: Vector3 creation
- **WHEN** Vector::<f64, 3>::builder("position").build() is called
- **THEN** it creates a 3-component vector

#### Scenario: Vector subtype constraint
- **WHEN** Position3D subtype is used with Vector::<f64, 3>
- **THEN** compilation succeeds

#### Scenario: Vector produces Value::Array
- **WHEN** Vector's to_value() is called
- **THEN** it returns Value::Array with N elements

---

### Requirement: Select Leaf Type
The system SHALL provide a unified `Select` type for single and multiple selection with static or dynamic options.

#### Scenario: Single static select
- **WHEN** Select::builder("method").single().static_options(...).build() is called
- **THEN** it creates a single-selection dropdown

#### Scenario: Multiple select
- **WHEN** Select::builder("tags").multiple(1, Some(5)).build() is called
- **THEN** it creates a multi-selection with min 1, max 5

#### Scenario: Select single produces Value::Text
- **WHEN** single Select's to_value() is called
- **THEN** it returns Value::Text

#### Scenario: Select multiple produces Value::Array
- **WHEN** multiple Select's to_value() is called
- **THEN** it returns Value::Array

---

### Requirement: Object Container Type
The system SHALL provide an `Object` type for nested structures with named fields.

#### Scenario: Object with fields
- **WHEN** Object::builder("address").field("street", Text::new("street")).build() is called
- **THEN** it creates an Object with the street field

#### Scenario: Object produces Value::Object
- **WHEN** Object's to_value() is called
- **THEN** it returns Value::Object with all field values

#### Scenario: Object field access
- **WHEN** Object's get_value("street") is called
- **THEN** it returns the street field's value

---

### Requirement: List Container Type
The system SHALL provide a `List` type for dynamic arrays from a template.

#### Scenario: List with item template
- **WHEN** List::builder("items").item_template(Text::new("item")).build() is called
- **THEN** it creates a List that can hold Text items

#### Scenario: List constraints
- **WHEN** List is created with min_items(1).max_items(10)
- **THEN** the constraints are enforced

#### Scenario: List produces Value::Array
- **WHEN** List's to_value() is called
- **THEN** it returns Value::Array with all item values

---

### Requirement: Mode Container Type
The system SHALL provide a `Mode` type for discriminated unions (branching).

#### Scenario: Mode with variants
- **WHEN** Mode::builder("auth").variant("none", ...).variant("basic", ...).build() is called
- **THEN** it creates a Mode with two variants

#### Scenario: Mode produces discriminated object
- **WHEN** Mode's to_value() is called with variant "basic"
- **THEN** it returns Value::Object with { mode: "basic", value: {...} }

#### Scenario: Mode default variant
- **WHEN** Mode is created with default_variant("none")
- **THEN** the initial variant is "none"

---

### Requirement: Routing Container Type
The system SHALL provide a `Routing` type for workflow connection wrappers.

#### Scenario: Routing with connection options
- **WHEN** Routing::builder("input").connection_required(true).build() is called
- **THEN** it creates a Routing that requires a connection

#### Scenario: Routing wraps child
- **WHEN** Routing is created with a child Object
- **THEN** the child is accessible

---

### Requirement: Expirable Container Type
The system SHALL provide an `Expirable` type for TTL-wrapped values.

#### Scenario: Expirable with TTL
- **WHEN** Expirable::builder("token").ttl_hours(1).build() is called
- **THEN** it creates an Expirable with 1-hour TTL

#### Scenario: Expirable produces timestamped object
- **WHEN** Expirable's to_value() is called
- **THEN** it returns Value::Object with { value, expires_at, created_at }

---

### Requirement: Ref Container Type
The system SHALL provide a `Ref` type for referencing template nodes.

#### Scenario: Ref targets template
- **WHEN** Ref::builder("billing").target("address_template").build() is called
- **THEN** it references the address_template node

#### Scenario: Ref has own visibility
- **WHEN** Ref is created with visible_when(...)
- **THEN** it can be shown/hidden independently of target

---

### Requirement: NodeKind Enum
The system SHALL provide a `NodeKind` enum identifying all 14 node types.

#### Scenario: NodeKind variants
- **WHEN** NodeKind is inspected
- **THEN** it has exactly 14 variants: Group, Panel, Notice, Object, List, Mode, Routing, Expirable, Ref, Text, Number, Boolean, Vector, Select

# Capability: Schema and Runtime System

## ADDED Requirements

### Requirement: Schema for Immutable Parameter Definitions
The system SHALL provide a Schema struct that holds immutable parameter definitions shared via Arc, with methods to access parameters by key and iterate all parameters.

#### Scenario: Create schema with builder
- **WHEN** creating Schema::builder().parameter(text_param).parameter(number_param).build()
- **THEN** it creates a schema containing both parameters

#### Scenario: Get parameter by key
- **WHEN** calling schema.get_parameter("username")
- **THEN** it returns Some(&Arc<dyn Node>) for the parameter

### Requirement: RuntimeNode Generic Wrapper
The system SHALL provide a RuntimeNode<T: Node> struct that wraps any node type with runtime state (StateFlags, validation errors, modified timestamp).

#### Scenario: Create runtime node from schema
- **WHEN** creating RuntimeNode::new(Arc::clone(&text_node))
- **THEN** it wraps the node with initial state (not dirty, not touched, valid)

#### Scenario: Track dirty state
- **WHEN** setting a value on a RuntimeNode
- **THEN** the DIRTY flag is set in StateFlags

### Requirement: Context for Runtime Parameter Trees
The system SHALL provide a Context struct that manages a complete parameter tree at runtime with value storage, state tracking, and event emission.

#### Scenario: Create context from schema
- **WHEN** creating Context::new(schema)
- **THEN** it instantiates RuntimeNode for each parameter in the schema

#### Scenario: Set value with pipeline
- **WHEN** calling context.set_value("price", Value::Float(99.99))
- **THEN** it runs transform → validate → store → notify pipeline

### Requirement: Transform-Validate-Notify Pipeline
The system SHALL execute transformers before validators, then store value, then emit events when setting values through Context or RuntimeNode.

#### Scenario: Transform before validate
- **WHEN** setting "  hello  " on Text with trim() and min_length(3)
- **THEN** transformation produces "hello", then validation passes

#### Scenario: Emit change event on success
- **WHEN** successfully setting a value
- **THEN** Context emits ParameterEvent::ValueChanged to EventBus

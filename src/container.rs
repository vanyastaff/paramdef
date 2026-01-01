//! Container types for structured data (schema definitions).
//!
//! This module provides the six container types that hold child nodes:
//!
//! - [`Object`] - Named fields
//! - [`List`] - Dynamic arrays
//! - [`Mode`] - Discriminated unions
//! - [`Routing`] - Connection wrapper for workflow nodes
//! - [`Expirable`] - TTL wrapper with expiration metadata
//! - [`Reference`] - Reference to a template node (Ref)
//!
//! # Design Philosophy
//!
//! Container nodes define the **schema** for structured data. They can contain
//! child nodes and implement the [`Container`](crate::node::Container) trait.
//!
//! Runtime values are managed separately by `RuntimeParameter<T>` or `Context`.
//!
//! # Example
//!
//! ```ignore
//! use paramdef::container::{Object, List, Mode};
//! use paramdef::types::leaf::{Text, Number};
//!
//! // Create an object with named fields
//! let address = Object::builder("address")
//!     .field("street", Text::builder("street").build())
//!     .field("city", Text::builder("city").build())
//!     .field("zip", Text::builder("zip").build())
//!     .build();
//!
//! // Create a list of items
//! let tags = List::builder("tags")
//!     .item_template(Text::builder("tag").build())
//!     .max_items(10)
//!     .build();
//!
//! // Create a discriminated union
//! let auth = Mode::builder("auth")
//!     .variant("none", "No Auth", Object::empty("none_config"))
//!     .variant("basic", "Basic Auth", Object::builder("basic_config")
//!         .field("username", Text::builder("username").build())
//!         .field("password", Text::builder("password").build())
//!         .build())
//!     .default_variant("none")
//!     .build();
//! ```

mod expirable;
mod list;
mod mode;
mod object;
mod reference;
mod routing;

pub use expirable::{Expirable, ExpirableBuilder, ExpirableOptions};
pub use list::{List, ListBuilder};
pub use mode::{Mode, ModeBuilder, ModeVariant};
pub use object::{Object, ObjectBuilder};
pub use reference::{Reference, ReferenceBuilder};
pub use routing::{Routing, RoutingBuilder, RoutingOptions};

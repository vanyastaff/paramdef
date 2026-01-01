//! Container parameter types for structured data.
//!
//! Container types have BOTH their own value AND children. They represent
//! structured data like objects, lists, and discriminated unions.
//!
//! # Types
//!
//! - [`Object`] - Named field collection (like structs)
//! - [`List`] - Dynamic array with item template
//! - [`Mode`] - Discriminated union (sum type / tagged union)
//! - [`Routing`] - Connection/reference wrapper
//! - [`Expirable`] - TTL-based wrapper for temporary data
//! - [`Reference`] - Template reference for reusable definitions
//!
//! # Example
//!
//! ```
//! use paramdef::types::container::Object;
//! use paramdef::types::leaf::Text;
//!
//! // Create an object with named fields
//! let address = Object::builder("address")
//!     .field("street", Text::builder("street").required().build())
//!     .field("city", Text::builder("city").required().build())
//!     .field("country", Text::builder("country").build())
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

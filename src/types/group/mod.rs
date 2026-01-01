//! Group and Layout types for parameter organization.
//!
//! Group and Layout types organize other parameters without having
//! their own values. They provide structure and UI organization.
//!
//! # Types
//!
//! - [`Group`] - Root aggregator with layout options
//! - [`Panel`] - UI panel for organizing sections (technically Layout)
//!
//! # Example
//!
//! ```
//! use paramdef::types::group::{Group, Panel};
//! use paramdef::types::leaf::Text;
//!
//! // Create a group with parameters
//! let settings = Group::builder("settings")
//!     .child(Text::builder("username").build())
//!     .child(Text::builder("email").build())
//!     .build();
//!
//! // Create a collapsible panel
//! let advanced = Panel::builder("advanced_settings")
//!     .label("Advanced Settings")
//!     .collapsed(true)
//!     .child(Text::builder("api_key").build())
//!     .build();
//! ```

mod panel;
mod root;

pub use panel::{Panel, PanelBuilder, PanelDisplayType};
pub use root::{Group, GroupBuilder, GroupLayout};

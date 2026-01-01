//! Group and layout types for UI organization.
//!
//! This module provides:
//!
//! - [`Group`] - Root aggregator that can contain Layout, Container, Leaf, and Decoration nodes
//! - [`Panel`] - Layout for UI organization (tabs, sections)
//!
//! # Design Philosophy
//!
//! These types organize the UI structure:
//!
//! - **Group** is the root container, can hold everything including Panels
//! - **Panel** organizes UI into sections/tabs, cannot contain other Panels or Groups
//!
//! For decoration types (Notice, Separator, Link, Code, Image), see the
//! [`decoration`](crate::decoration) module.
//!
//! # Example
//!
//! ```ignore
//! use paramdef::group::{Group, Panel};
//! use paramdef::decoration::Notice;
//! use paramdef::container::Object;
//! use paramdef::types::leaf::Text;
//!
//! let config = Group::builder("settings")
//!     .child(Panel::builder("general")
//!         .label("General")
//!         .child(Notice::info("welcome", "Configure your settings below."))
//!         .child(Text::builder("name").build())
//!         .build())
//!     .child(Panel::builder("advanced")
//!         .label("Advanced")
//!         .collapsed(true)
//!         .child(Object::builder("options").build())
//!         .build())
//!     .build();
//! ```

mod panel;
mod root;

pub use panel::{Panel, PanelBuilder, PanelDisplayType};
pub use root::{Group, GroupBuilder, GroupLayout};

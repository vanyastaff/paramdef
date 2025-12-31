//! Decoration types for display-only UI elements.
//!
//! Decorations are nodes with no value and no children - purely informational.
//! This module provides 5 built-in decoration types:
//!
//! - [`Notice`] - Info, warning, error, success, tip messages
//! - [`Separator`] - Visual dividers between sections
//! - [`Link`] - Clickable references to documentation
//! - [`Code`] - Syntax-highlighted code snippets
//! - [`Image`] - Static image display
//!
//! # Key Invariants
//!
//! - NO own Value (decorations don't produce data)
//! - NO children (terminal display elements)
//! - CAN have visibility expressions (feature-gated)
//!
//! # Custom Decorations
//!
//! The `Decoration` trait is a marker trait, so you can create your own
//! decoration types by implementing `Node` with `kind() -> NodeKind::Decoration`
//! and the empty `Decoration` trait.
//!
//! # Example
//!
//! ```ignore
//! use paramdef::decoration::{Notice, Separator, Link, Code, Image};
//! use paramdef::node::NoticeType;
//!
//! // Info notice
//! let info = Notice::info("welcome", "Configure your settings below.");
//!
//! // Section separator with label
//! let sep = Separator::thick("advanced_sep")
//!     .label("Advanced Settings")
//!     .build();
//!
//! // Documentation link
//! let docs = Link::documentation("api_docs", "API Reference")
//!     .url("https://docs.example.com/api")
//!     .build();
//!
//! // Code example
//! let example = Code::json("payload", r#"{"status": "ok"}"#);
//!
//! // Image
//! let screenshot = Image::from_url("step1", "https://example.com/step1.png")
//!     .alt_text("Step 1: Click Connect")
//!     .build();
//! ```

mod code;
mod image;
mod link;
mod notice;
mod separator;

pub use code::{Code, CodeBuilder};
pub use image::{Image, ImageAlignment, ImageBuilder, ImageSource};
pub use link::{Link, LinkBuilder};
pub use notice::{Notice, NoticeBuilder};
pub use separator::{Separator, SeparatorBuilder};

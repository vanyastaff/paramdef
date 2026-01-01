//! Decoration types for display-only elements.
//!
//! Decorations are UI elements with NO value and NO children.
//! They exist purely for display and organization purposes.
//!
//! # Types
//!
//! - [`Notice`] - Info, warning, error, and success messages
//! - [`Separator`] - Visual dividers between sections
//! - [`Link`] - Clickable references to docs/external resources
//! - [`Code`] - Syntax-highlighted code snippets
//! - [`Image`] - Static image display
//! - [`Html`] - Rich HTML content with sanitization options
//! - [`Video`] - Embedded video content (YouTube/Vimeo/direct URL)
//! - [`Progress`] - Progress bars, spinners, and step indicators
//!
//! # Example
//!
//! ```
//! use paramdef::types::decoration::{Notice, Separator};
//! use paramdef::types::kind::NoticeType;
//!
//! // Info message
//! let notice = Notice::builder("help_text")
//!     .notice_type(NoticeType::Info)
//!     .message("Enter your credentials to continue")
//!     .build();
//!
//! // Visual separator
//! let sep = Separator::builder("divider").build();
//! ```

mod code;
mod html;
mod image;
mod link;
mod notice;
mod progress;
mod separator;
mod video;

pub use code::{Code, CodeBuilder};
pub use html::{Html, HtmlBuilder, SanitizeLevel};
pub use image::{Image, ImageAlignment, ImageBuilder, ImageSource};
pub use link::{Link, LinkBuilder};
pub use notice::{Notice, NoticeBuilder};
pub use progress::{Progress, ProgressBuilder, ProgressOptions, ProgressSource, ProgressStyle};
pub use separator::{Separator, SeparatorBuilder};
pub use video::{Video, VideoBuilder, VideoOptions, VideoSize, VideoSource};

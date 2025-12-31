//! Image decoration for static image display.
//!
//! Image displays static images from various sources.

use std::any::Any;
use std::path::PathBuf;

use crate::core::{Flags, Key, Metadata};
use crate::node::{Decoration, Node, NodeKind};

/// The source of an image.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImageSource {
    /// URL to an image.
    Url(String),
    /// Base64-encoded image data.
    Base64(String),
    /// Path to an image file.
    Path(PathBuf),
}

impl ImageSource {
    /// Creates a URL source.
    #[must_use]
    pub fn url(url: impl Into<String>) -> Self {
        Self::Url(url.into())
    }

    /// Creates a Base64 source.
    #[must_use]
    pub fn base64(data: impl Into<String>) -> Self {
        Self::Base64(data.into())
    }

    /// Creates a path source.
    #[must_use]
    pub fn path(path: impl Into<PathBuf>) -> Self {
        Self::Path(path.into())
    }
}

/// Image alignment options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ImageAlignment {
    /// Align to the left.
    Left,
    /// Center the image.
    #[default]
    Center,
    /// Align to the right.
    Right,
}

impl ImageAlignment {
    /// Returns the name of this alignment.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Center => "center",
            Self::Right => "right",
        }
    }
}

/// A static image decoration.
///
/// Image displays an image from a URL, Base64 data, or file path.
/// It has no value and cannot contain children.
///
/// # Example
///
/// ```ignore
/// use paramdef::decoration::{Image, ImageSource, ImageAlignment};
///
/// // Image from URL
/// let screenshot = Image::builder("step1")
///     .source(ImageSource::url("https://example.com/step1.png"))
///     .alt_text("Click the connect button")
///     .width(400)
///     .alignment(ImageAlignment::Center)
///     .build();
///
/// // Image from file path
/// let logo = Image::from_path("logo", "./assets/logo.png")
///     .alt_text("Company Logo")
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct Image {
    metadata: Metadata,
    flags: Flags,
    source: ImageSource,
    alt_text: String,
    width: Option<u32>,
    height: Option<u32>,
    alignment: ImageAlignment,
}

impl Image {
    /// Creates a new builder for an Image.
    #[must_use]
    pub fn builder(key: impl Into<Key>) -> ImageBuilder {
        ImageBuilder::new(key)
    }

    /// Creates an image from a URL.
    #[must_use]
    pub fn from_url(key: impl Into<Key>, url: impl Into<String>) -> ImageBuilder {
        Self::builder(key).source(ImageSource::url(url))
    }

    /// Creates an image from a file path.
    #[must_use]
    pub fn from_path(key: impl Into<Key>, path: impl Into<PathBuf>) -> ImageBuilder {
        Self::builder(key).source(ImageSource::path(path))
    }

    /// Returns the flags for this image.
    #[must_use]
    pub fn flags(&self) -> Flags {
        self.flags
    }

    /// Returns the image source.
    #[must_use]
    pub fn source(&self) -> &ImageSource {
        &self.source
    }

    /// Returns the alt text.
    #[must_use]
    pub fn alt_text(&self) -> &str {
        &self.alt_text
    }

    /// Returns the width, if specified.
    #[must_use]
    pub fn width(&self) -> Option<u32> {
        self.width
    }

    /// Returns the height, if specified.
    #[must_use]
    pub fn height(&self) -> Option<u32> {
        self.height
    }

    /// Returns the alignment.
    #[must_use]
    pub fn alignment(&self) -> ImageAlignment {
        self.alignment
    }
}

impl Node for Image {
    fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    fn key(&self) -> &Key {
        self.metadata.key()
    }

    fn kind(&self) -> NodeKind {
        NodeKind::Decoration
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Decoration for Image {}

// =============================================================================
// Builder
// =============================================================================

/// Builder for [`Image`].
#[derive(Debug)]
pub struct ImageBuilder {
    key: Key,
    flags: Flags,
    source: Option<ImageSource>,
    alt_text: String,
    width: Option<u32>,
    height: Option<u32>,
    alignment: ImageAlignment,
}

impl ImageBuilder {
    /// Creates a new builder with the given key.
    ///
    /// Note: You must call `.source()` before `.build()`, or use
    /// `Image::from_url()` / `Image::from_path()` constructors.
    #[must_use]
    pub fn new(key: impl Into<Key>) -> Self {
        Self {
            key: key.into(),
            flags: Flags::empty(),
            source: None,
            alt_text: String::new(),
            width: None,
            height: None,
            alignment: ImageAlignment::Center,
        }
    }

    /// Sets the flags.
    #[must_use]
    pub fn flags(mut self, flags: Flags) -> Self {
        self.flags = flags;
        self
    }

    /// Sets the image source (required).
    #[must_use]
    pub fn source(mut self, source: ImageSource) -> Self {
        self.source = Some(source);
        self
    }

    /// Sets the alt text.
    #[must_use]
    pub fn alt_text(mut self, alt_text: impl Into<String>) -> Self {
        self.alt_text = alt_text.into();
        self
    }

    /// Sets the width in pixels.
    #[must_use]
    pub fn width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets the height in pixels.
    #[must_use]
    pub fn height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    /// Sets both width and height.
    #[must_use]
    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    /// Sets the alignment.
    #[must_use]
    pub fn alignment(mut self, alignment: ImageAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Builds the Image.
    ///
    /// # Errors
    ///
    /// Returns an error if the source was not specified.
    pub fn build(self) -> crate::core::Result<Image> {
        let source = self
            .source
            .ok_or_else(|| crate::core::Error::missing_required("source"))?;

        Ok(Image {
            metadata: Metadata::new(self.key),
            flags: self.flags,
            source,
            alt_text: self.alt_text,
            width: self.width,
            height: self.height,
            alignment: self.alignment,
        })
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_from_url() {
        let image = Image::from_url("screenshot", "https://example.com/img.png")
            .alt_text("Screenshot")
            .width(400)
            .build()
            .unwrap();

        assert_eq!(image.key().as_str(), "screenshot");
        assert_eq!(
            image.source(),
            &ImageSource::Url("https://example.com/img.png".into())
        );
        assert_eq!(image.alt_text(), "Screenshot");
        assert_eq!(image.width(), Some(400));
        assert!(image.height().is_none());
    }

    #[test]
    fn test_image_from_path() {
        let image = Image::from_path("logo", "./assets/logo.png")
            .alt_text("Logo")
            .build()
            .unwrap();

        assert!(matches!(image.source(), ImageSource::Path(_)));
    }

    #[test]
    fn test_image_alignment() {
        let left = Image::from_url("l", "#")
            .alignment(ImageAlignment::Left)
            .build()
            .unwrap();
        assert_eq!(left.alignment(), ImageAlignment::Left);

        let center = Image::from_url("c", "#").build().unwrap();
        assert_eq!(center.alignment(), ImageAlignment::Center); // default

        let right = Image::from_url("r", "#")
            .alignment(ImageAlignment::Right)
            .build()
            .unwrap();
        assert_eq!(right.alignment(), ImageAlignment::Right);
    }

    #[test]
    fn test_image_size() {
        let image = Image::from_url("img", "#").size(800, 600).build().unwrap();

        assert_eq!(image.width(), Some(800));
        assert_eq!(image.height(), Some(600));
    }

    #[test]
    fn test_image_source_constructors() {
        let url = ImageSource::url("https://example.com");
        assert!(matches!(url, ImageSource::Url(_)));

        let base64 = ImageSource::base64("data:image/png;base64,...");
        assert!(matches!(base64, ImageSource::Base64(_)));

        let path = ImageSource::path("./image.png");
        assert!(matches!(path, ImageSource::Path(_)));
    }

    #[test]
    fn test_image_alignment_names() {
        assert_eq!(ImageAlignment::Left.name(), "left");
        assert_eq!(ImageAlignment::Center.name(), "center");
        assert_eq!(ImageAlignment::Right.name(), "right");
    }

    #[test]
    fn test_image_kind() {
        let image = Image::from_url("test", "#").build().unwrap();

        assert_eq!(image.kind(), NodeKind::Decoration);
    }

    #[test]
    fn test_image_invariants() {
        let image = Image::from_url("test", "#").build().unwrap();

        assert!(!image.kind().has_own_value());
        assert!(!image.kind().has_value_access());
        assert!(!image.kind().can_have_children());
    }

    #[test]
    fn test_image_requires_source() {
        let result = Image::builder("no_source")
            .alt_text("Missing source")
            .build();
        assert!(result.is_err());
    }
}

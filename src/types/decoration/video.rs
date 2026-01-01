//! Video decoration for displaying embedded videos.
//!
//! Video displays video content from various sources
//! (YouTube/Vimeo/direct URLs) as a display-only element in the UI.

use std::any::Any;

use crate::core::{Flags, Key, Metadata, SmartStr};
use crate::types::kind::NodeKind;
use crate::types::traits::{Decoration, Node};

/// Source type for video content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VideoSource {
    /// Direct URL to video file (mp4, webm, etc.).
    Url(SmartStr),
    /// YouTube video ID or URL.
    #[allow(clippy::doc_markdown)]
    YouTube(SmartStr),
    /// Vimeo video ID or URL.
    Vimeo(SmartStr),
    /// Local file path.
    File(SmartStr),
    /// Embedded HTML (iframe content).
    Embed(SmartStr),
}

impl VideoSource {
    /// Creates a direct URL source.
    #[must_use]
    pub fn url(url: impl Into<SmartStr>) -> Self {
        Self::Url(url.into())
    }

    /// Creates a YouTube source from video ID or URL.
    #[must_use]
    #[allow(clippy::doc_markdown)]
    pub fn youtube(id_or_url: impl Into<SmartStr>) -> Self {
        Self::YouTube(id_or_url.into())
    }

    /// Creates a Vimeo source from video ID or URL.
    #[must_use]
    pub fn vimeo(id_or_url: impl Into<SmartStr>) -> Self {
        Self::Vimeo(id_or_url.into())
    }

    /// Creates a local file source.
    #[must_use]
    pub fn file(path: impl Into<SmartStr>) -> Self {
        Self::File(path.into())
    }

    /// Creates an embed source (raw iframe HTML).
    #[must_use]
    pub fn embed(html: impl Into<SmartStr>) -> Self {
        Self::Embed(html.into())
    }

    /// Returns the source type name.
    #[must_use]
    pub fn source_type(&self) -> &'static str {
        match self {
            Self::Url(_) => "url",
            Self::YouTube(_) => "youtube",
            Self::Vimeo(_) => "vimeo",
            Self::File(_) => "file",
            Self::Embed(_) => "embed",
        }
    }

    /// Returns the source value.
    #[must_use]
    pub fn value(&self) -> &str {
        match self {
            Self::Url(v) | Self::YouTube(v) | Self::Vimeo(v) | Self::File(v) | Self::Embed(v) => {
                v.as_str()
            }
        }
    }
}

/// Video size specification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VideoSize {
    /// Width in pixels or percentage.
    pub width: u32,
    /// Height in pixels or percentage.
    pub height: u32,
}

impl VideoSize {
    /// Creates a new size with width and height.
    #[must_use]
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// Creates a 16:9 aspect ratio size from width.
    #[must_use]
    pub fn widescreen(width: u32) -> Self {
        Self {
            width,
            height: width * 9 / 16,
        }
    }

    /// Creates a 4:3 aspect ratio size from width.
    #[must_use]
    pub fn standard(width: u32) -> Self {
        Self {
            width,
            height: width * 3 / 4,
        }
    }

    /// Default video size (640x360, 16:9).
    #[must_use]
    pub fn default_size() -> Self {
        Self::widescreen(640)
    }
}

impl Default for VideoSize {
    fn default() -> Self {
        Self::default_size()
    }
}

/// Video playback options packed into a single struct.
#[derive(Debug, Clone, Copy, Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct VideoOptions {
    /// Video starts automatically.
    pub autoplay: bool,
    /// Video is muted.
    pub muted: bool,
    /// Video loops continuously.
    pub looping: bool,
    /// Show playback controls.
    pub controls: bool,
}

impl VideoOptions {
    /// Creates options with controls enabled (default).
    #[must_use]
    pub fn with_controls() -> Self {
        Self {
            controls: true,
            ..Self::default()
        }
    }
}

/// A display-only video decoration.
///
/// Video displays embedded video content from various sources. It has no value
/// and cannot contain children.
///
/// # Example
///
/// ```ignore
/// use paramdef::decoration::{Video, VideoSource};
///
/// // YouTube video
/// let tutorial = Video::youtube("intro", "dQw4w9WgXcQ")
///     .label("Introduction Tutorial");
///
/// // Direct video URL
/// let demo = Video::builder("demo")
///     .source(VideoSource::url("https://example.com/video.mp4"))
///     .autoplay(true)
///     .muted(true)
///     .loop_video(true)
///     .build();
///
/// // Vimeo video with poster
/// let presentation = Video::vimeo("presentation", "123456789")
///     .poster("https://example.com/poster.jpg");
/// ```
#[derive(Debug, Clone)]
pub struct Video {
    metadata: Metadata,
    flags: Flags,
    source: VideoSource,
    poster: Option<SmartStr>,
    size: Option<VideoSize>,
    options: VideoOptions,
}

impl Video {
    /// Creates a new builder for a Video.
    #[must_use]
    pub fn builder(key: impl Into<Key>) -> VideoBuilder {
        VideoBuilder::new(key)
    }

    /// Creates a YouTube video decoration.
    #[must_use]
    #[allow(clippy::doc_markdown)]
    pub fn youtube(key: impl Into<Key>, video_id: impl Into<SmartStr>) -> VideoBuilder {
        VideoBuilder::new(key).source(VideoSource::youtube(video_id))
    }

    /// Creates a Vimeo video decoration.
    #[must_use]
    pub fn vimeo(key: impl Into<Key>, video_id: impl Into<SmartStr>) -> VideoBuilder {
        VideoBuilder::new(key).source(VideoSource::vimeo(video_id))
    }

    /// Creates a video decoration from a direct URL.
    #[must_use]
    pub fn url(key: impl Into<Key>, url: impl Into<SmartStr>) -> VideoBuilder {
        VideoBuilder::new(key).source(VideoSource::url(url))
    }

    /// Returns the flags for this video.
    #[must_use]
    pub fn flags(&self) -> Flags {
        self.flags
    }

    /// Returns the video source.
    #[must_use]
    pub fn source(&self) -> &VideoSource {
        &self.source
    }

    /// Returns the poster image URL, if set.
    #[must_use]
    pub fn poster(&self) -> Option<&str> {
        self.poster.as_deref()
    }

    /// Returns the video size, if set.
    #[must_use]
    pub fn size(&self) -> Option<VideoSize> {
        self.size
    }

    /// Returns true if autoplay is enabled.
    #[must_use]
    pub fn autoplay(&self) -> bool {
        self.options.autoplay
    }

    /// Returns true if the video is muted.
    #[must_use]
    pub fn muted(&self) -> bool {
        self.options.muted
    }

    /// Returns true if the video loops.
    #[must_use]
    pub fn looping(&self) -> bool {
        self.options.looping
    }

    /// Returns true if controls are shown.
    #[must_use]
    pub fn controls(&self) -> bool {
        self.options.controls
    }
}

impl Node for Video {
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

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Decoration for Video {}

// =============================================================================
// Builder
// =============================================================================

/// Builder for [`Video`].
#[derive(Debug)]
pub struct VideoBuilder {
    key: Key,
    label: Option<SmartStr>,
    description: Option<SmartStr>,
    flags: Flags,
    source: Option<VideoSource>,
    poster: Option<SmartStr>,
    size: Option<VideoSize>,
    options: VideoOptions,
}

impl VideoBuilder {
    /// Creates a new builder with the given key.
    #[must_use]
    pub fn new(key: impl Into<Key>) -> Self {
        Self {
            key: key.into(),
            label: None,
            description: None,
            flags: Flags::empty(),
            source: None,
            poster: None,
            size: None,
            options: VideoOptions::with_controls(),
        }
    }

    /// Sets the label.
    #[must_use]
    pub fn label(mut self, label: impl Into<SmartStr>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the description.
    #[must_use]
    pub fn description(mut self, description: impl Into<SmartStr>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the flags.
    #[must_use]
    pub fn flags(mut self, flags: Flags) -> Self {
        self.flags = flags;
        self
    }

    /// Sets the video source.
    #[must_use]
    pub fn source(mut self, source: VideoSource) -> Self {
        self.source = Some(source);
        self
    }

    /// Sets the poster image URL.
    #[must_use]
    pub fn poster(mut self, url: impl Into<SmartStr>) -> Self {
        self.poster = Some(url.into());
        self
    }

    /// Sets the video size.
    #[must_use]
    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.size = Some(VideoSize::new(width, height));
        self
    }

    /// Sets the video size with 16:9 aspect ratio.
    #[must_use]
    pub fn widescreen(mut self, width: u32) -> Self {
        self.size = Some(VideoSize::widescreen(width));
        self
    }

    /// Enables autoplay (video starts automatically).
    #[must_use]
    pub fn autoplay(mut self, autoplay: bool) -> Self {
        self.options.autoplay = autoplay;
        self
    }

    /// Mutes the video.
    #[must_use]
    pub fn muted(mut self, muted: bool) -> Self {
        self.options.muted = muted;
        self
    }

    /// Enables video looping.
    #[must_use]
    pub fn looping(mut self, looping: bool) -> Self {
        self.options.looping = looping;
        self
    }

    /// Shows or hides video controls.
    #[must_use]
    pub fn controls(mut self, controls: bool) -> Self {
        self.options.controls = controls;
        self
    }

    /// Builds the Video decoration.
    ///
    /// # Panics
    ///
    /// Panics if no source was set.
    #[must_use]
    pub fn build(self) -> Video {
        let source = self
            .source
            .expect("Video requires a source. Use .source() to set one.");

        let mut metadata = Metadata::new(self.key);
        if let Some(label) = self.label {
            metadata = metadata.with_label(label);
        }
        if let Some(description) = self.description {
            metadata = metadata.with_description(description);
        }

        Video {
            metadata,
            flags: self.flags,
            source,
            poster: self.poster,
            size: self.size,
            options: self.options,
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_youtube() {
        let video = Video::youtube("intro", "dQw4w9WgXcQ")
            .label("Introduction")
            .build();

        assert_eq!(video.key().as_str(), "intro");
        assert_eq!(video.metadata().label(), Some("Introduction"));
        assert_eq!(video.source().source_type(), "youtube");
        assert_eq!(video.source().value(), "dQw4w9WgXcQ");
        assert!(video.controls());
        assert!(!video.autoplay());
    }

    #[test]
    fn test_video_vimeo() {
        let video = Video::vimeo("presentation", "123456789").build();

        assert_eq!(video.source().source_type(), "vimeo");
        assert_eq!(video.source().value(), "123456789");
    }

    #[test]
    fn test_video_url() {
        let video = Video::url("demo", "https://example.com/video.mp4").build();

        assert_eq!(video.source().source_type(), "url");
        assert_eq!(video.source().value(), "https://example.com/video.mp4");
    }

    #[test]
    fn test_video_builder() {
        let video = Video::builder("custom")
            .source(VideoSource::file("/path/to/video.mp4"))
            .poster("https://example.com/poster.jpg")
            .size(800, 600)
            .autoplay(true)
            .muted(true)
            .looping(true)
            .controls(false)
            .build();

        assert_eq!(video.source().source_type(), "file");
        assert_eq!(video.poster(), Some("https://example.com/poster.jpg"));
        assert_eq!(video.size(), Some(VideoSize::new(800, 600)));
        assert!(video.autoplay());
        assert!(video.muted());
        assert!(video.looping());
        assert!(!video.controls());
    }

    #[test]
    fn test_video_widescreen() {
        let video = Video::youtube("vid", "abc")
            .widescreen(1280)
            .build();

        let size = video.size().unwrap();
        assert_eq!(size.width, 1280);
        assert_eq!(size.height, 720); // 16:9 aspect ratio
    }

    #[test]
    fn test_video_kind() {
        let video = Video::youtube("test", "abc").build();

        assert_eq!(video.kind(), NodeKind::Decoration);
    }

    #[test]
    fn test_video_invariants() {
        let video = Video::youtube("test", "abc").build();

        // Decoration has NO own value
        assert!(!video.kind().has_own_value());

        // Decoration has NO ValueAccess
        assert!(!video.kind().has_value_access());

        // Decoration CANNOT have children
        assert!(!video.kind().can_have_children());
    }

    #[test]
    fn test_video_source_constructors() {
        let url = VideoSource::url("https://example.com");
        assert_eq!(url.source_type(), "url");

        let yt = VideoSource::youtube("abc123");
        assert_eq!(yt.source_type(), "youtube");

        let vimeo = VideoSource::vimeo("456");
        assert_eq!(vimeo.source_type(), "vimeo");

        let file = VideoSource::file("/path/to/file");
        assert_eq!(file.source_type(), "file");

        let embed = VideoSource::embed("<iframe></iframe>");
        assert_eq!(embed.source_type(), "embed");
    }

    #[test]
    fn test_video_size_default() {
        let size = VideoSize::default();
        assert_eq!(size.width, 640);
        assert_eq!(size.height, 360);
    }

    #[test]
    fn test_video_size_standard() {
        let size = VideoSize::standard(800);
        assert_eq!(size.width, 800);
        assert_eq!(size.height, 600); // 4:3 aspect ratio
    }

    #[test]
    #[should_panic(expected = "Video requires a source")]
    fn test_video_requires_source() {
        let _ = Video::builder("no_source").build();
    }
}

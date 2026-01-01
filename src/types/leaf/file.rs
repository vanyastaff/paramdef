//! File parameter type for file uploads and references.

use crate::core::{Flags, Key, Metadata, SmartStr, Value};
use crate::subtype::FileSubtype;
use crate::types::kind::NodeKind;
use crate::types::traits::{Leaf, Node};

/// A file parameter schema for file uploads and references.
///
/// File parameters support various file types through [`FileSubtype`],
/// with MIME type filtering and size constraints.
/// This is the **schema** definition - it does not hold runtime values.
///
/// # Value Representation
///
/// File produces `Value::Object` with standard fields:
/// ```json
/// {
///     "id": "file-abc123",
///     "name": "document.pdf",
///     "size": 102400,
///     "mime": "application/pdf",
///     "url": "https://..."
/// }
/// ```
///
/// # Example
///
/// ```
/// use paramdef::types::leaf::File;
/// use paramdef::subtype::{Image, Pdf};
///
/// // Generic file upload
/// let attachment = File::builder("attachment")
///     .label("Attachment")
///     .build();
///
/// // Image upload with size limit
/// let avatar = File::image("avatar")
///     .max_size_mb(5)
///     .required()
///     .build();
///
/// // PDF document
/// let document = File::pdf("contract")
///     .label("Contract PDF")
///     .required()
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct File<S: FileSubtype = crate::subtype::GenericFile> {
    metadata: Metadata,
    flags: Flags,
    subtype: S,
    /// Additional accepted MIME types (merged with subtype's accept).
    accept: Vec<SmartStr>,
    /// Maximum file size in bytes (overrides subtype's `max_size`).
    max_size: Option<u64>,
    /// Allow multiple files.
    multiple: bool,
}

impl<S: FileSubtype> File<S> {
    /// Returns the file subtype.
    #[must_use]
    pub fn subtype(&self) -> &S {
        &self.subtype
    }

    /// Returns the accepted MIME types.
    ///
    /// Combines subtype's accept list with any additional types.
    #[must_use]
    pub fn accept(&self) -> Vec<&str> {
        let mut result: Vec<&str> = S::accept().to_vec();
        for mime in &self.accept {
            result.push(mime.as_str());
        }
        result
    }

    /// Returns the maximum file size in bytes.
    ///
    /// Returns the explicit `max_size` if set, otherwise the subtype's default.
    #[must_use]
    pub fn max_size(&self) -> Option<u64> {
        self.max_size.or_else(S::max_size)
    }

    /// Returns whether multiple files are allowed.
    #[must_use]
    pub fn is_multiple(&self) -> bool {
        self.multiple
    }

    /// Returns the flags.
    #[must_use]
    pub fn flags(&self) -> Flags {
        self.flags
    }
}

impl File<crate::subtype::GenericFile> {
    /// Creates a new builder for a generic file parameter.
    pub fn builder(key: impl Into<Key>) -> FileBuilder<crate::subtype::GenericFile> {
        FileBuilder::new(key)
    }
}

// Convenience constructors for common subtypes
impl File<crate::subtype::Image> {
    /// Creates an image file parameter builder.
    pub fn image(key: impl Into<Key>) -> FileBuilder<crate::subtype::Image> {
        FileBuilder::new(key).subtype(crate::subtype::Image)
    }
}

impl File<crate::subtype::Photo> {
    /// Creates a photo file parameter builder.
    pub fn photo(key: impl Into<Key>) -> FileBuilder<crate::subtype::Photo> {
        FileBuilder::new(key).subtype(crate::subtype::Photo)
    }
}

impl File<crate::subtype::Avatar> {
    /// Creates an avatar file parameter builder.
    pub fn avatar(key: impl Into<Key>) -> FileBuilder<crate::subtype::Avatar> {
        FileBuilder::new(key).subtype(crate::subtype::Avatar)
    }
}

impl File<crate::subtype::Pdf> {
    /// Creates a PDF file parameter builder.
    pub fn pdf(key: impl Into<Key>) -> FileBuilder<crate::subtype::Pdf> {
        FileBuilder::new(key).subtype(crate::subtype::Pdf)
    }
}

impl File<crate::subtype::Document> {
    /// Creates a document file parameter builder.
    pub fn document(key: impl Into<Key>) -> FileBuilder<crate::subtype::Document> {
        FileBuilder::new(key).subtype(crate::subtype::Document)
    }
}

impl File<crate::subtype::Video> {
    /// Creates a video file parameter builder.
    pub fn video(key: impl Into<Key>) -> FileBuilder<crate::subtype::Video> {
        FileBuilder::new(key).subtype(crate::subtype::Video)
    }
}

impl File<crate::subtype::Audio> {
    /// Creates an audio file parameter builder.
    pub fn audio(key: impl Into<Key>) -> FileBuilder<crate::subtype::Audio> {
        FileBuilder::new(key).subtype(crate::subtype::Audio)
    }
}

impl File<crate::subtype::Archive> {
    /// Creates an archive file parameter builder.
    pub fn archive(key: impl Into<Key>) -> FileBuilder<crate::subtype::Archive> {
        FileBuilder::new(key).subtype(crate::subtype::Archive)
    }
}

impl File<crate::subtype::Signature> {
    /// Creates a signature file parameter builder.
    pub fn signature(key: impl Into<Key>) -> FileBuilder<crate::subtype::Signature> {
        FileBuilder::new(key).subtype(crate::subtype::Signature)
    }
}

impl<S: FileSubtype + 'static> Node for File<S> {
    fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    fn key(&self) -> &Key {
        self.metadata.key()
    }

    fn kind(&self) -> NodeKind {
        NodeKind::Leaf
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl<S: FileSubtype> Leaf for File<S> {
    fn default_value(&self) -> Option<Value> {
        // Files have no default value - they must be uploaded
        None
    }
}

/// Builder for [`File`] parameters.
#[derive(Debug, Clone)]
pub struct FileBuilder<S: FileSubtype = crate::subtype::GenericFile> {
    key: Key,
    label: Option<SmartStr>,
    description: Option<SmartStr>,
    group: Option<Key>,
    flags: Flags,
    subtype: S,
    accept: Vec<SmartStr>,
    max_size: Option<u64>,
    multiple: bool,
}

impl FileBuilder<crate::subtype::GenericFile> {
    /// Creates a new file builder.
    pub fn new(key: impl Into<Key>) -> Self {
        Self {
            key: key.into(),
            label: None,
            description: None,
            group: None,
            flags: Flags::empty(),
            subtype: crate::subtype::GenericFile,
            accept: Vec::new(),
            max_size: None,
            multiple: false,
        }
    }
}

impl<S: FileSubtype> FileBuilder<S> {
    /// Sets the subtype, returning a builder with the new type.
    pub fn subtype<T: FileSubtype>(self, subtype: T) -> FileBuilder<T> {
        FileBuilder {
            key: self.key,
            label: self.label,
            description: self.description,
            group: self.group,
            flags: self.flags,
            subtype,
            accept: self.accept,
            max_size: self.max_size,
            multiple: self.multiple,
        }
    }

    /// Sets the display label.
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

    /// Sets the group.
    #[must_use]
    pub fn group(mut self, group: impl Into<Key>) -> Self {
        self.group = Some(group.into());
        self
    }

    /// Adds additional accepted MIME types.
    #[must_use]
    pub fn accept(mut self, mime_types: impl IntoIterator<Item = impl Into<SmartStr>>) -> Self {
        self.accept.extend(mime_types.into_iter().map(Into::into));
        self
    }

    /// Sets the maximum file size in bytes.
    #[must_use]
    pub fn max_size(mut self, bytes: u64) -> Self {
        self.max_size = Some(bytes);
        self
    }

    /// Sets the maximum file size in kilobytes.
    #[must_use]
    pub fn max_size_kb(mut self, kb: u64) -> Self {
        self.max_size = Some(kb * 1024);
        self
    }

    /// Sets the maximum file size in megabytes.
    #[must_use]
    pub fn max_size_mb(mut self, mb: u64) -> Self {
        self.max_size = Some(mb * 1024 * 1024);
        self
    }

    /// Allows multiple file uploads.
    #[must_use]
    pub fn multiple(mut self) -> Self {
        self.multiple = true;
        self
    }

    /// Marks the parameter as required.
    #[must_use]
    pub fn required(mut self) -> Self {
        self.flags |= Flags::REQUIRED;
        self
    }

    /// Marks the parameter as readonly.
    #[must_use]
    pub fn readonly(mut self) -> Self {
        self.flags |= Flags::READONLY;
        self
    }

    /// Marks the parameter as hidden.
    #[must_use]
    pub fn hidden(mut self) -> Self {
        self.flags |= Flags::HIDDEN;
        self
    }

    /// Builds the file parameter.
    #[must_use]
    pub fn build(self) -> File<S> {
        let mut metadata_builder = Metadata::builder(self.key);

        if let Some(label) = self.label {
            metadata_builder = metadata_builder.label(label);
        }
        if let Some(description) = self.description {
            metadata_builder = metadata_builder.description(description);
        }
        if let Some(group) = self.group {
            metadata_builder = metadata_builder.group(group);
        }

        File {
            metadata: metadata_builder.build(),
            flags: self.flags,
            subtype: self.subtype,
            accept: self.accept,
            max_size: self.max_size,
            multiple: self.multiple,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::subtype::{Avatar, FileSubtype, Image, Pdf};

    #[test]
    fn test_file_minimal() {
        let file = File::builder("attachment").build();

        assert_eq!(file.key(), "attachment");
        assert_eq!(file.kind(), NodeKind::Leaf);
        assert!(file.default_value().is_none());
        assert!(!file.is_multiple());
    }

    #[test]
    fn test_file_builder() {
        let file = File::builder("upload")
            .label("Upload File")
            .description("Select a file to upload")
            .max_size_mb(10)
            .required()
            .build();

        assert_eq!(file.key(), "upload");
        assert_eq!(file.metadata().label(), Some("Upload File"));
        assert_eq!(file.max_size(), Some(10 * 1024 * 1024));
        assert!(file.flags().contains(Flags::REQUIRED));
    }

    #[test]
    fn test_file_image_convenience() {
        let image: File<Image> = File::image("photo").build();

        assert_eq!(image.key(), "photo");
        assert_eq!(Image::accept(), &["image/*"]);
    }

    #[test]
    fn test_file_avatar_convenience() {
        let avatar: File<Avatar> = File::avatar("profile_pic").required().build();

        assert_eq!(avatar.key(), "profile_pic");
        assert!(avatar.flags().contains(Flags::REQUIRED));
        // Avatar has default max_size from subtype
        assert_eq!(Avatar::max_size(), Some(5_242_880));
        assert_eq!(avatar.max_size(), Some(5_242_880));
    }

    #[test]
    fn test_file_pdf_convenience() {
        let pdf: File<Pdf> = File::pdf("document").build();

        assert_eq!(pdf.key(), "document");
        assert_eq!(Pdf::accept(), &["application/pdf"]);
    }

    #[test]
    fn test_file_multiple() {
        let files = File::builder("attachments").multiple().build();

        assert!(files.is_multiple());
    }

    #[test]
    fn test_file_custom_accept() {
        let file = File::builder("data")
            .accept(["application/json", "text/csv"])
            .build();

        let accept = file.accept();
        assert!(accept.contains(&"application/json"));
        assert!(accept.contains(&"text/csv"));
    }

    #[test]
    fn test_file_max_size_override() {
        // Avatar subtype has default 5MB
        let avatar: File<Avatar> = File::avatar("small_avatar").max_size_kb(100).build();

        // Explicit max_size overrides subtype default
        assert_eq!(avatar.max_size(), Some(100 * 1024));
    }

    #[test]
    fn test_file_subtype_change() {
        let builder = File::builder("file").label("File");
        let pdf_file = builder.subtype(Pdf).build();

        assert_eq!(pdf_file.key(), "file");
        assert_eq!(pdf_file.accept(), vec!["application/pdf"]);
    }
}

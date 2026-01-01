//! Standard file subtypes.
//!
//! File subtypes provide semantic meaning and constraints for file uploads:
//!
//! ## Generic
//! - [`GenericFile`] - Any file type
//! - [`Attachment`] - Email-style attachment
//!
//! ## Images
//! - [`Image`] - Any image
//! - [`Photo`] - JPEG/PNG/WebP photos
//! - [`Icon`] - Small PNG/SVG icons
//! - [`Avatar`] - Profile pictures
//! - [`Thumbnail`] - Small preview images
//!
//! ## Documents
//! - [`Document`] - Office documents
//! - [`Pdf`] - PDF files only
//! - [`Spreadsheet`] - Excel/CSV files
//! - [`Presentation`] - Presentation files
//!
//! ## Media
//! - [`Video`] - Video files
//! - [`Audio`] - Audio files
//!
//! ## Data
//! - [`JsonFile`] - JSON data files
//! - [`CsvFile`] - CSV data files
//! - [`XmlFile`] - XML data files
//!
//! ## Archives
//! - [`Archive`] - Compressed archives
//!
//! ## Special
//! - [`Signature`] - Canvas signature (PNG)

use crate::define_file_subtype;

// === Generic ===

define_file_subtype!(GenericFile, "file");
define_file_subtype!(Attachment, "attachment");

// === Images ===

define_file_subtype!(Image, "image", accept: ["image/*"]);
define_file_subtype!(Photo, "photo", accept: ["image/jpeg", "image/png", "image/webp"]);
define_file_subtype!(Icon, "icon", accept: ["image/png", "image/svg+xml"], max_size: 102_400);
define_file_subtype!(Avatar, "avatar", accept: ["image/jpeg", "image/png", "image/webp"], max_size: 5_242_880);
define_file_subtype!(Thumbnail, "thumbnail", accept: ["image/jpeg", "image/png", "image/webp"], max_size: 524_288);

// === Documents ===

define_file_subtype!(Document, "document", accept: [
    "application/pdf",
    "application/msword",
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
    "application/vnd.oasis.opendocument.text"
]);
define_file_subtype!(Pdf, "pdf", accept: ["application/pdf"]);
define_file_subtype!(Spreadsheet, "spreadsheet", accept: [
    "application/vnd.ms-excel",
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
    "text/csv",
    "application/vnd.oasis.opendocument.spreadsheet"
]);
define_file_subtype!(Presentation, "presentation", accept: [
    "application/vnd.ms-powerpoint",
    "application/vnd.openxmlformats-officedocument.presentationml.presentation",
    "application/vnd.oasis.opendocument.presentation"
]);

// === Media ===

define_file_subtype!(Video, "video", accept: ["video/*"]);
define_file_subtype!(Audio, "audio", accept: ["audio/*"]);

// === Data ===

define_file_subtype!(JsonFile, "json_file", accept: ["application/json"]);
define_file_subtype!(CsvFile, "csv_file", accept: ["text/csv"]);
define_file_subtype!(XmlFile, "xml_file", accept: ["application/xml", "text/xml"]);

// === Archives ===

define_file_subtype!(Archive, "archive", accept: [
    "application/zip",
    "application/gzip",
    "application/x-tar",
    "application/x-7z-compressed",
    "application/x-rar-compressed"
]);

// === Special ===

define_file_subtype!(Signature, "signature", accept: ["image/png"], max_size: 524_288);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::subtype::FileSubtype;

    // === Generic Tests ===

    #[test]
    fn test_generic_file() {
        assert_eq!(GenericFile::name(), "file");
        assert!(GenericFile::accept().is_empty());
        assert_eq!(GenericFile::max_size(), None);
    }

    #[test]
    fn test_attachment() {
        assert_eq!(Attachment::name(), "attachment");
    }

    // === Image Tests ===

    #[test]
    fn test_image() {
        assert_eq!(Image::name(), "image");
        assert_eq!(Image::accept(), &["image/*"]);
    }

    #[test]
    fn test_photo() {
        assert_eq!(Photo::name(), "photo");
        assert!(Photo::accept().contains(&"image/jpeg"));
        assert!(Photo::accept().contains(&"image/png"));
        assert!(Photo::accept().contains(&"image/webp"));
    }

    #[test]
    fn test_icon() {
        assert_eq!(Icon::name(), "icon");
        assert_eq!(Icon::max_size(), Some(102_400));
    }

    #[test]
    fn test_avatar() {
        assert_eq!(Avatar::name(), "avatar");
        assert_eq!(Avatar::max_size(), Some(5_242_880));
    }

    // === Document Tests ===

    #[test]
    fn test_document() {
        assert_eq!(Document::name(), "document");
        assert!(Document::accept().contains(&"application/pdf"));
    }

    #[test]
    fn test_pdf() {
        assert_eq!(Pdf::name(), "pdf");
        assert_eq!(Pdf::accept(), &["application/pdf"]);
    }

    #[test]
    fn test_spreadsheet() {
        assert_eq!(Spreadsheet::name(), "spreadsheet");
        assert!(Spreadsheet::accept().contains(&"text/csv"));
    }

    // === Media Tests ===

    #[test]
    fn test_video() {
        assert_eq!(Video::name(), "video");
        assert_eq!(Video::accept(), &["video/*"]);
    }

    #[test]
    fn test_audio() {
        assert_eq!(Audio::name(), "audio");
        assert_eq!(Audio::accept(), &["audio/*"]);
    }

    // === Data Tests ===

    #[test]
    fn test_json_file() {
        assert_eq!(JsonFile::name(), "json_file");
        assert_eq!(JsonFile::accept(), &["application/json"]);
    }

    #[test]
    fn test_csv_file() {
        assert_eq!(CsvFile::name(), "csv_file");
        assert_eq!(CsvFile::accept(), &["text/csv"]);
    }

    // === Archive Tests ===

    #[test]
    fn test_archive() {
        assert_eq!(Archive::name(), "archive");
        assert!(Archive::accept().contains(&"application/zip"));
    }

    // === Special Tests ===

    #[test]
    fn test_signature() {
        assert_eq!(Signature::name(), "signature");
        assert_eq!(Signature::accept(), &["image/png"]);
        assert_eq!(Signature::max_size(), Some(524_288));
    }
}

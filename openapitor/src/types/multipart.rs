//! Multipart form data types.

use std::path::PathBuf;

/// An attachement to a multipart form.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Attachment {
    /// The name of the field.
    pub name: String,
    /// The file path of the attachment.
    pub filepath: Option<PathBuf>,
    /// The content type of the attachment.
    pub content_type: Option<String>,
    /// The data of the attachment.
    pub data: Vec<u8>,
}

impl std::convert::TryFrom<Attachment> for reqwest::multipart::Part {
    type Error = reqwest::Error;

    fn try_from(attachment: Attachment) -> Result<Self, Self::Error> {
        let mut part = reqwest::multipart::Part::bytes(attachment.data);
        if let Some(filepath) = attachment.filepath {
            part = part.file_name(filepath.to_string_lossy().to_string());
        }
        if let Some(content_type) = attachment.content_type {
            part = part.mime_str(&content_type)?;
        }
        Ok(part)
    }
}

impl std::convert::TryFrom<std::path::PathBuf> for Attachment {
    type Error = std::io::Error;

    fn try_from(path: std::path::PathBuf) -> Result<Self, Self::Error> {
        // Get the mime type of the file.
        let content_type = mime_guess::from_path(&path).first_raw();
        let data = std::fs::read(&path)?;
        Ok(Attachment {
            name: "file".to_string(),
            filepath: Some(path),
            content_type: content_type.map(|s| s.to_string()),
            data,
        })
    }
}

//! Multipart form data types.

/// An attachement to a multipart form.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Attachment {
    /// The name of the field.
    pub name: String,
    /// The filename of the attachment.
    pub filename: Option<String>,
    /// The content type of the attachment.
    pub content_type: Option<String>,
    /// The data of the attachment.
    pub data: Vec<u8>,
}

impl std::convert::TryFrom<Attachment> for reqwest::multipart::Part {
    type Error = reqwest::Error;

    fn try_from(attachment: Attachment) -> Result<Self, Self::Error> {
        let mut part = reqwest::multipart::Part::bytes(attachment.data);
        if let Some(filename) = attachment.filename {
            part = part.file_name(filename);
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
        let filename = path
            .file_name()
            .ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid filename")
            })?
            .to_str()
            .ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "invalid filename")
            })?
            .to_string();
        // Get the mime type of the file.
        let content_type = mime_guess::from_path(&path).first_raw();
        let data = std::fs::read(path)?;
        Ok(Attachment {
            name: "file".to_string(),
            filename: Some(filename),
            content_type: content_type.map(|s| s.to_string()),
            data,
        })
    }
}

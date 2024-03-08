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

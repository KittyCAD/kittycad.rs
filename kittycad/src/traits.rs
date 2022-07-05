use anyhow::Result;

#[async_trait::async_trait]
pub trait Base64Ops {
    /// Create a file conversion and decode the output.
    async fn create_conversion_with_decode<B: Into<reqwest::Body> + std::marker::Send>(
        &self,
        output_format: crate::types::FileOutputFormat,
        src_format: crate::types::FileSourceFormat,
        body: B,
    ) -> Result<(crate::types::FileConversion, Vec<u8>)>;
}

#[async_trait::async_trait]
impl Base64Ops for crate::file::File {
    async fn create_conversion_with_decode<B: Into<reqwest::Body> + std::marker::Send>(
        &self,
        output_format: crate::types::FileOutputFormat,
        src_format: crate::types::FileSourceFormat,
        body: B,
    ) -> Result<(crate::types::FileConversion, Vec<u8>)> {
        let conversion = self
            .create_conversion(output_format, src_format, body)
            .await?;

        // Decode the output.
        let output = if let Some(o) = &conversion.output {
            data_encoding::BASE64.decode(o)?
        } else {
            vec![]
        };

        Ok((conversion, output))
    }
}

use crate::Client;
use anyhow::Result;
pub struct File {
    pub client: Client,
}

impl File {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Convert CAD file.\n\nConvert a CAD file from one format to another. If the file being converted is larger than 25MB, it will be performed asynchronously.\nIf the conversion is performed synchronously, the contents of the converted file (`output`) will be returned as a base64 encoded string.\nIf the operation is performed asynchronously, the `id` of the operation will be returned. You can use the `id` returned from the request to get status information about the async operation from the `/async/operations/{id}` endpoint."]
    pub async fn create_conversion(
        &self,
        output_format: crate::types::FileOutputFormat,
        src_format: crate::types::FileSourceFormat,
        body: &bytes::Bytes,
    ) -> Result<crate::types::FileConversion> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "file/conversion/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.body(body.clone());
        let resp = req.send().await?;
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        if status.is_success() {
            serde_json::from_str(&text)
                .map_err(|err| format_serde_error::SerdeError::new(text.to_string(), err).into())
        } else {
            Err(anyhow::anyhow!(
                "response was not successful `{}` -> `{}`",
                status,
                text
            ))
        }
    }

    #[doc = "Get a file conversion.\n\nGet the status and output of an async file conversion.\nThis endpoint requires authentication by any KittyCAD user. It returns details of the requested file conversion for the user.\nIf the user is not authenticated to view the specified file conversion, then it is not returned.\nOnly KittyCAD employees with the proper access can view file conversions for other users."]
    pub async fn get_conversion(&self, id: &str) -> Result<crate::types::AsyncApiCallOutput> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "file/conversions/{id}".replace("{id}", &id)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        if status.is_success() {
            serde_json::from_str(&text)
                .map_err(|err| format_serde_error::SerdeError::new(text.to_string(), err).into())
        } else {
            Err(anyhow::anyhow!(
                "response was not successful `{}` -> `{}`",
                status,
                text
            ))
        }
    }

    #[doc = "Get CAD file density.\n\nGet the density of an object in a CAD file. If the file is larger than 25MB, it will be performed asynchronously.\nIf the operation is performed asynchronously, the `id` of the operation will be returned. You can use the `id` returned from the request to get status information about the async operation from the `/async/operations/{id}` endpoint."]
    pub async fn create_density(
        &self,
        material_mass: f64,
        src_format: crate::types::FileSourceFormat,
        body: &bytes::Bytes,
    ) -> Result<crate::types::FileDensity> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "file/density"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        query_params.push(("material_mass", format!("{}", material_mass)));
        query_params.push(("src_format", format!("{}", src_format)));
        req = req.query(&query_params);
        req = req.body(body.clone());
        let resp = req.send().await?;
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        if status.is_success() {
            serde_json::from_str(&text)
                .map_err(|err| format_serde_error::SerdeError::new(text.to_string(), err).into())
        } else {
            Err(anyhow::anyhow!(
                "response was not successful `{}` -> `{}`",
                status,
                text
            ))
        }
    }

    #[doc = "Execute a KittyCAD program in a specific language."]
    pub async fn create_execution(
        &self,
        lang: crate::types::CodeLanguage,
        output: Option<String>,
        body: &bytes::Bytes,
    ) -> Result<crate::types::CodeOutput> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!(
                "{}/{}",
                self.client.base_url,
                "file/execute/{lang}".replace("{lang}", &format!("{}", lang))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        if let Some(p) = output {
            query_params.push(("output", p));
        }

        req = req.query(&query_params);
        req = req.body(body.clone());
        let resp = req.send().await?;
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        if status.is_success() {
            serde_json::from_str(&text)
                .map_err(|err| format_serde_error::SerdeError::new(text.to_string(), err).into())
        } else {
            Err(anyhow::anyhow!(
                "response was not successful `{}` -> `{}`",
                status,
                text
            ))
        }
    }

    #[doc = "Get CAD file mass.\n\nGet the mass of an object in a CAD file. If the file is larger than 25MB, it will be performed asynchronously.\nIf the operation is performed asynchronously, the `id` of the operation will be returned. You can use the `id` returned from the request to get status information about the async operation from the `/async/operations/{id}` endpoint."]
    pub async fn create_mass(
        &self,
        material_density: f64,
        src_format: crate::types::FileSourceFormat,
        body: &bytes::Bytes,
    ) -> Result<crate::types::FileMass> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "file/mass"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        query_params.push(("material_density", format!("{}", material_density)));
        query_params.push(("src_format", format!("{}", src_format)));
        req = req.query(&query_params);
        req = req.body(body.clone());
        let resp = req.send().await?;
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        if status.is_success() {
            serde_json::from_str(&text)
                .map_err(|err| format_serde_error::SerdeError::new(text.to_string(), err).into())
        } else {
            Err(anyhow::anyhow!(
                "response was not successful `{}` -> `{}`",
                status,
                text
            ))
        }
    }

    #[doc = "Get CAD file volume.\n\nGet the volume of an object in a CAD file. If the file is larger than 25MB, it will be performed asynchronously.\nIf the operation is performed asynchronously, the `id` of the operation will be returned. You can use the `id` returned from the request to get status information about the async operation from the `/async/operations/{id}` endpoint."]
    pub async fn create_volume(
        &self,
        src_format: crate::types::FileSourceFormat,
        body: &bytes::Bytes,
    ) -> Result<crate::types::FileVolume> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "file/volume"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        query_params.push(("src_format", format!("{}", src_format)));
        req = req.query(&query_params);
        req = req.body(body.clone());
        let resp = req.send().await?;
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        if status.is_success() {
            serde_json::from_str(&text)
                .map_err(|err| format_serde_error::SerdeError::new(text.to_string(), err).into())
        } else {
            Err(anyhow::anyhow!(
                "response was not successful `{}` -> `{}`",
                status,
                text
            ))
        }
    }

    #[doc = "Get a file conversion for your user.\n\nGet the status and output of an async file conversion. If completed, the contents of the converted file (`output`) will be returned as a base64 encoded string.\nThis endpoint requires authentication by any KittyCAD user. It returns details of the requested file conversion for the user."]
    pub async fn get_conversion_for_user(
        &self,
        id: &str,
    ) -> Result<crate::types::AsyncApiCallOutput> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "user/file/conversions/{id}".replace("{id}", &id)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        if status.is_success() {
            serde_json::from_str(&text)
                .map_err(|err| format_serde_error::SerdeError::new(text.to_string(), err).into())
        } else {
            Err(anyhow::anyhow!(
                "response was not successful `{}` -> `{}`",
                status,
                text
            ))
        }
    }
}

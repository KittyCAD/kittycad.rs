use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct File {
    pub client: Client,
}

impl File {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get CAD file center of mass.\n\nGet the center of mass of an object in a CAD file. If \
             the file is larger than 25MB, it will be performed asynchronously.\nIf the operation \
             is performed asynchronously, the `id` of the operation will be returned. You can use \
             the `id` returned from the request to get status information about the async \
             operation from the `/async/operations/{id}` endpoint.\n\n**Parameters:**\n\n- \
             `material_density: f64`: The material density. (required)\n- `src_format: \
             crate::types::FileSourceFormat`: The format of the file. \
             (required)\n\n```rust,no_run\nasync fn example_file_create_center_of_mass() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::FileCenterOfMass = client\n        .file()\n        \
             .create_center_of_mass(\n            3.14 as f64,\n            \
             kittycad::types::FileSourceFormat::Obj,\n            \
             &bytes::Bytes::from(\"some-string\"),\n        )\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_center_of_mass<'a>(
        &'a self,
        material_density: f64,
        src_format: crate::types::FileSourceFormat,
        body: &bytes::Bytes,
    ) -> Result<crate::types::FileCenterOfMass, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "file/center-of-mass"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        query_params.push(("material_density", format!("{}", material_density)));
        query_params.push(("src_format", format!("{}", src_format)));
        req = req.query(&query_params);
        req = req.body(body.clone());
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    format_serde_error::SerdeError::new(text.to_string(), err),
                    status,
                )
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Convert CAD file.\n\nConvert a CAD file from one format to another. If the file being \
             converted is larger than 25MB, it will be performed asynchronously.\nIf the \
             conversion is performed synchronously, the contents of the converted file (`output`) \
             will be returned as a base64 encoded string.\nIf the operation is performed \
             asynchronously, the `id` of the operation will be returned. You can use the `id` \
             returned from the request to get status information about the async operation from \
             the `/async/operations/{id}` endpoint.\n\n**Parameters:**\n\n- `output_format: \
             crate::types::FileOutputFormat`: The format the file should be converted to. \
             (required)\n- `src_format: crate::types::FileSourceFormat`: The format of the file to \
             convert. (required)\n\n```rust,no_run\nasync fn example_file_create_conversion() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::FileConversion = client\n        .file()\n        \
             .create_conversion(\n            kittycad::types::FileOutputFormat::Fbx,\n            \
             kittycad::types::FileSourceFormat::Dae,\n            \
             &bytes::Bytes::from(\"some-string\"),\n        )\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_conversion<'a>(
        &'a self,
        output_format: crate::types::FileOutputFormat,
        src_format: crate::types::FileSourceFormat,
        body: &bytes::Bytes,
    ) -> Result<crate::types::FileConversion, crate::types::error::Error> {
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
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    format_serde_error::SerdeError::new(text.to_string(), err),
                    status,
                )
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Get a file conversion.\n\nGet the status and output of an async file \
             conversion.\nThis endpoint requires authentication by any KittyCAD user. It returns \
             details of the requested file conversion for the user.\nIf the user is not \
             authenticated to view the specified file conversion, then it is not returned.\nOnly \
             KittyCAD employees with the proper access can view file conversions for other \
             users.\n\n**Parameters:**\n\n- `id: &'astr`: The ID of the async operation. \
             (required)\n\n```rust,no_run\nasync fn example_file_get_conversion() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::AsyncApiCallOutput =\n        \
             client.file().get_conversion(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_conversion<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<crate::types::AsyncApiCallOutput, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "file/conversions/{id}".replace("{id}", id)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    format_serde_error::SerdeError::new(text.to_string(), err),
                    status,
                )
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Get CAD file density.\n\nGet the density of an object in a CAD file. If the file is larger than 25MB, it will be performed asynchronously.\nIf the operation is performed asynchronously, the `id` of the operation will be returned. You can use the `id` returned from the request to get status information about the async operation from the `/async/operations/{id}` endpoint.\n\n**Parameters:**\n\n- `material_mass: f64`: The material mass. (required)\n- `src_format: crate::types::FileSourceFormat`: The format of the file. (required)\n\n```rust,no_run\nasync fn example_file_create_density() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::FileDensity = client\n        .file()\n        .create_density(\n            3.14 as f64,\n            kittycad::types::FileSourceFormat::Dae,\n            &bytes::Bytes::from(\"some-string\"),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_density<'a>(
        &'a self,
        material_mass: f64,
        src_format: crate::types::FileSourceFormat,
        body: &bytes::Bytes,
    ) -> Result<crate::types::FileDensity, crate::types::error::Error> {
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
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    format_serde_error::SerdeError::new(text.to_string(), err),
                    status,
                )
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Execute a KittyCAD program in a specific language.\n\n**Parameters:**\n\n- `lang: \
             crate::types::CodeLanguage`: The language of the code. (required)\n- `output: \
             Option<String>`: The output file we want to get the contents for (the paths are \
             relative to where in litterbox it is being run). You can denote more than one file \
             with a comma separated list of string paths.\n\n```rust,no_run\nasync fn \
             example_file_create_execution() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: kittycad::types::CodeOutput = \
             client\n        .file()\n        .create_execution(\n            \
             kittycad::types::CodeLanguage::Node,\n            \
             Some(\"some-string\".to_string()),\n            \
             &bytes::Bytes::from(\"some-string\"),\n        )\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_execution<'a>(
        &'a self,
        lang: crate::types::CodeLanguage,
        output: Option<String>,
        body: &bytes::Bytes,
    ) -> Result<crate::types::CodeOutput, crate::types::error::Error> {
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
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    format_serde_error::SerdeError::new(text.to_string(), err),
                    status,
                )
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Get CAD file mass.\n\nGet the mass of an object in a CAD file. If the file is larger than 25MB, it will be performed asynchronously.\nIf the operation is performed asynchronously, the `id` of the operation will be returned. You can use the `id` returned from the request to get status information about the async operation from the `/async/operations/{id}` endpoint.\n\n**Parameters:**\n\n- `material_density: f64`: The material density. (required)\n- `src_format: crate::types::FileSourceFormat`: The format of the file. (required)\n\n```rust,no_run\nasync fn example_file_create_mass() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::FileMass = client\n        .file()\n        .create_mass(\n            3.14 as f64,\n            kittycad::types::FileSourceFormat::Step,\n            &bytes::Bytes::from(\"some-string\"),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_mass<'a>(
        &'a self,
        material_density: f64,
        src_format: crate::types::FileSourceFormat,
        body: &bytes::Bytes,
    ) -> Result<crate::types::FileMass, crate::types::error::Error> {
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
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    format_serde_error::SerdeError::new(text.to_string(), err),
                    status,
                )
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Get CAD file surface area.\n\nGet the surface area of an object in a CAD file. If the file is larger than 25MB, it will be performed asynchronously.\nIf the operation is performed asynchronously, the `id` of the operation will be returned. You can use the `id` returned from the request to get status information about the async operation from the `/async/operations/{id}` endpoint.\n\n**Parameters:**\n\n- `src_format: crate::types::FileSourceFormat`: The format of the file. (required)\n\n```rust,no_run\nasync fn example_file_create_surface_area() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::FileSurfaceArea = client\n        .file()\n        .create_surface_area(\n            kittycad::types::FileSourceFormat::Dae,\n            &bytes::Bytes::from(\"some-string\"),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_surface_area<'a>(
        &'a self,
        src_format: crate::types::FileSourceFormat,
        body: &bytes::Bytes,
    ) -> Result<crate::types::FileSurfaceArea, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "file/surface-area"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        query_params.push(("src_format", format!("{}", src_format)));
        req = req.query(&query_params);
        req = req.body(body.clone());
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    format_serde_error::SerdeError::new(text.to_string(), err),
                    status,
                )
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Get CAD file volume.\n\nGet the volume of an object in a CAD file. If the file is \
             larger than 25MB, it will be performed asynchronously.\nIf the operation is performed \
             asynchronously, the `id` of the operation will be returned. You can use the `id` \
             returned from the request to get status information about the async operation from \
             the `/async/operations/{id}` endpoint.\n\n**Parameters:**\n\n- `src_format: \
             crate::types::FileSourceFormat`: The format of the file. \
             (required)\n\n```rust,no_run\nasync fn example_file_create_volume() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::FileVolume = client\n        .file()\n        \
             .create_volume(\n            kittycad::types::FileSourceFormat::Step,\n            \
             &bytes::Bytes::from(\"some-string\"),\n        )\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_volume<'a>(
        &'a self,
        src_format: crate::types::FileSourceFormat,
        body: &bytes::Bytes,
    ) -> Result<crate::types::FileVolume, crate::types::error::Error> {
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
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    format_serde_error::SerdeError::new(text.to_string(), err),
                    status,
                )
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Get a file conversion for your user.\n\nGet the status and output of an async file \
             conversion. If completed, the contents of the converted file (`output`) will be \
             returned as a base64 encoded string.\nThis endpoint requires authentication by any \
             KittyCAD user. It returns details of the requested file conversion for the \
             user.\n\n**Parameters:**\n\n- `id: &'astr`: The ID of the async operation. \
             (required)\n\n```rust,no_run\nasync fn example_file_get_conversion_for_user() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::AsyncApiCallOutput =\n        \
             client.file().get_conversion_for_user(\"some-string\").await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_conversion_for_user<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<crate::types::AsyncApiCallOutput, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "user/file/conversions/{id}".replace("{id}", id)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    format_serde_error::SerdeError::new(text.to_string(), err),
                    status,
                )
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }
}

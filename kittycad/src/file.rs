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

    #[doc = "Get CAD file center of mass.\n\nWe assume any file given to us has one consistent unit throughout. We also assume the file is at the proper scale.\nThis endpoint returns the cartesian coordinate in world space measure units.\nIn the future, we will use the units inside the file if they are given and do any conversions if necessary for the calculation. But currently, that is not supported.\nGet the center of mass of an object in a CAD file. If the file is larger than 25MB, it will be performed asynchronously.\nIf the operation is performed asynchronously, the `id` of the operation will be returned. You can use the `id` returned from the request to get status information about the async operation from the `/async/operations/{id}` endpoint.\n\n**Parameters:**\n\n- `output_unit: Option<crate::types::UnitLength>`: The output unit for the center of mass.\n- `src_format: crate::types::FileImportFormat`: The format of the file. (required)\n\n```rust,no_run\nasync fn example_file_create_center_of_mass() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::FileCenterOfMass = client\n        .file()\n        .create_center_of_mass(\n            Some(kittycad::types::UnitLength::Yd),\n            kittycad::types::FileImportFormat::Stl,\n            &bytes::Bytes::from(\"some-string\"),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_center_of_mass<'a>(
        &'a self,
        output_unit: Option<crate::types::UnitLength>,
        src_format: crate::types::FileImportFormat,
        body: &bytes::Bytes,
    ) -> Result<crate::types::FileCenterOfMass, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "file/center-of-mass"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![("src_format", format!("{}", src_format))];
        if let Some(p) = output_unit {
            query_params.push(("output_unit", format!("{}", p)));
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
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Convert CAD file with defaults.\n\nIf you wish to specify the conversion options, use \
             the `/file/conversion` endpoint instead.\nConvert a CAD file from one format to \
             another. If the file being converted is larger than 25MB, it will be performed \
             asynchronously.\nIf the conversion is performed synchronously, the contents of the \
             converted file (`output`) will be returned as a base64 encoded string.\nIf the \
             operation is performed asynchronously, the `id` of the operation will be returned. \
             You can use the `id` returned from the request to get status information about the \
             async operation from the `/async/operations/{id}` endpoint.\n\n**Parameters:**\n\n- \
             `output_format: crate::types::FileExportFormat`: The format the file should be \
             converted to. (required)\n- `src_format: crate::types::FileImportFormat`: The format \
             of the file to convert. (required)\n\n```rust,no_run\nasync fn \
             example_file_create_conversion() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: kittycad::types::FileConversion = \
             client\n        .file()\n        .create_conversion(\n            \
             kittycad::types::FileExportFormat::Stl,\n            \
             kittycad::types::FileImportFormat::Stl,\n            \
             &bytes::Bytes::from(\"some-string\"),\n        )\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_conversion<'a>(
        &'a self,
        output_format: crate::types::FileExportFormat,
        src_format: crate::types::FileImportFormat,
        body: &bytes::Bytes,
    ) -> Result<crate::types::FileConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
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
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Get CAD file density.\n\nWe assume any file given to us has one consistent unit \
             throughout. We also assume the file is at the proper scale.\nThis endpoint assumes if \
             you are giving a material mass in a specific mass units, we return a density in mass \
             unit per cubic measure unit.\nIn the future, we will use the units inside the file if \
             they are given and do any conversions if necessary for the calculation. But \
             currently, that is not supported.\nGet the density of an object in a CAD file. If the \
             file is larger than 25MB, it will be performed asynchronously.\nIf the operation is \
             performed asynchronously, the `id` of the operation will be returned. You can use the \
             `id` returned from the request to get status information about the async operation \
             from the `/async/operations/{id}` endpoint.\n\n**Parameters:**\n\n- `material_mass: \
             f64`: The material mass. (required)\n- `material_mass_unit: \
             Option<crate::types::UnitMass>`: The unit of the material mass.\n- `output_unit: \
             Option<crate::types::UnitDensity>`: The output unit for the density.\n- `src_format: \
             crate::types::FileImportFormat`: The format of the file. \
             (required)\n\n```rust,no_run\nasync fn example_file_create_density() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::FileDensity = client\n        .file()\n        \
             .create_density(\n            3.14 as f64,\n            \
             Some(kittycad::types::UnitMass::Lb),\n            \
             Some(kittycad::types::UnitDensity::KgM3),\n            \
             kittycad::types::FileImportFormat::Stl,\n            \
             &bytes::Bytes::from(\"some-string\"),\n        )\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_density<'a>(
        &'a self,
        material_mass: f64,
        material_mass_unit: Option<crate::types::UnitMass>,
        output_unit: Option<crate::types::UnitDensity>,
        src_format: crate::types::FileImportFormat,
        body: &bytes::Bytes,
    ) -> Result<crate::types::FileDensity, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "file/density"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![
            ("material_mass", format!("{}", material_mass)),
            ("src_format", format!("{}", src_format)),
        ];
        if let Some(p) = material_mass_unit {
            query_params.push(("material_mass_unit", format!("{}", p)));
        }

        if let Some(p) = output_unit {
            query_params.push(("output_unit", format!("{}", p)));
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
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Get CAD file mass.\n\nWe assume any file given to us has one consistent unit throughout. We also assume the file is at the proper scale.\nThis endpoint assumes if you are giving a material density in a specific mass unit per cubic measure unit, we return a mass in mass units. The same mass units as passed in the material density.\nIn the future, we will use the units inside the file if they are given and do any conversions if necessary for the calculation. But currently, that is not supported.\nGet the mass of an object in a CAD file. If the file is larger than 25MB, it will be performed asynchronously.\nIf the operation is performed asynchronously, the `id` of the operation will be returned. You can use the `id` returned from the request to get status information about the async operation from the `/async/operations/{id}` endpoint.\n\n**Parameters:**\n\n- `material_density: f64`: The material density. (required)\n- `material_density_unit: Option<crate::types::UnitDensity>`: The unit of the material density.\n- `output_unit: Option<crate::types::UnitMass>`: The output unit for the mass.\n- `src_format: crate::types::FileImportFormat`: The format of the file. (required)\n\n```rust,no_run\nasync fn example_file_create_mass() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::FileMass = client\n        .file()\n        .create_mass(\n            3.14 as f64,\n            Some(kittycad::types::UnitDensity::KgM3),\n            Some(kittycad::types::UnitMass::Lb),\n            kittycad::types::FileImportFormat::Stl,\n            &bytes::Bytes::from(\"some-string\"),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_mass<'a>(
        &'a self,
        material_density: f64,
        material_density_unit: Option<crate::types::UnitDensity>,
        output_unit: Option<crate::types::UnitMass>,
        src_format: crate::types::FileImportFormat,
        body: &bytes::Bytes,
    ) -> Result<crate::types::FileMass, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "file/mass"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![
            ("material_density", format!("{}", material_density)),
            ("src_format", format!("{}", src_format)),
        ];
        if let Some(p) = material_density_unit {
            query_params.push(("material_density_unit", format!("{}", p)));
        }

        if let Some(p) = output_unit {
            query_params.push(("output_unit", format!("{}", p)));
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
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Get CAD file surface area.\n\nWe assume any file given to us has one consistent unit \
             throughout. We also assume the file is at the proper scale.\nThis endpoint returns \
             the square measure units.\nIn the future, we will use the units inside the file if \
             they are given and do any conversions if necessary for the calculation. But \
             currently, that is not supported.\nGet the surface area of an object in a CAD file. \
             If the file is larger than 25MB, it will be performed asynchronously.\nIf the \
             operation is performed asynchronously, the `id` of the operation will be returned. \
             You can use the `id` returned from the request to get status information about the \
             async operation from the `/async/operations/{id}` endpoint.\n\n**Parameters:**\n\n- \
             `output_unit: Option<crate::types::UnitArea>`: The output unit for the surface \
             area.\n- `src_format: crate::types::FileImportFormat`: The format of the file. \
             (required)\n\n```rust,no_run\nasync fn example_file_create_surface_area() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::FileSurfaceArea = client\n        .file()\n        \
             .create_surface_area(\n            Some(kittycad::types::UnitArea::Yd2),\n            \
             kittycad::types::FileImportFormat::Stl,\n            \
             &bytes::Bytes::from(\"some-string\"),\n        )\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_surface_area<'a>(
        &'a self,
        output_unit: Option<crate::types::UnitArea>,
        src_format: crate::types::FileImportFormat,
        body: &bytes::Bytes,
    ) -> Result<crate::types::FileSurfaceArea, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "file/surface-area"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![("src_format", format!("{}", src_format))];
        if let Some(p) = output_unit {
            query_params.push(("output_unit", format!("{}", p)));
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
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Get CAD file volume.\n\nWe assume any file given to us has one consistent unit \
             throughout. We also assume the file is at the proper scale.\nThis endpoint returns \
             the cubic measure units.\nIn the future, we will use the units inside the file if \
             they are given and do any conversions if necessary for the calculation. But \
             currently, that is not supported.\nGet the volume of an object in a CAD file. If the \
             file is larger than 25MB, it will be performed asynchronously.\nIf the operation is \
             performed asynchronously, the `id` of the operation will be returned. You can use the \
             `id` returned from the request to get status information about the async operation \
             from the `/async/operations/{id}` endpoint.\n\n**Parameters:**\n\n- `output_unit: \
             Option<crate::types::UnitVolume>`: The output unit for the volume.\n- `src_format: \
             crate::types::FileImportFormat`: The format of the file. \
             (required)\n\n```rust,no_run\nasync fn example_file_create_volume() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::FileVolume = client\n        .file()\n        \
             .create_volume(\n            Some(kittycad::types::UnitVolume::Ml),\n            \
             kittycad::types::FileImportFormat::Stl,\n            \
             &bytes::Bytes::from(\"some-string\"),\n        )\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_volume<'a>(
        &'a self,
        output_unit: Option<crate::types::UnitVolume>,
        src_format: crate::types::FileImportFormat,
        body: &bytes::Bytes,
    ) -> Result<crate::types::FileVolume, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "file/volume"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![("src_format", format!("{}", src_format))];
        if let Some(p) = output_unit {
            query_params.push(("output_unit", format!("{}", p)));
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
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }
}

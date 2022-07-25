use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Unit {
    pub client: Client,
}

impl Unit {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Convert units.\n\nConvert a unit value to another metric unit value. This is a nice \
             endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: \
             crate::types::UnitAccelerationFormat`: The output format of the unit. (required)\n- \
             `src_format: crate::types::UnitAccelerationFormat`: The source format of the unit. \
             (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn \
             example_unit_get_acceleration_conversion() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::UnitAccelerationConversion = client\n        .unit()\n        \
             .get_acceleration_conversion(\n            \
             kittycad::types::UnitAccelerationFormat::StandardGravity,\n            \
             kittycad::types::UnitAccelerationFormat::FeetPerSecondSquared,\n            3.14 as \
             f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_acceleration_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitAccelerationFormat,
        src_format: crate::types::UnitAccelerationFormat,
        value: f64,
    ) -> Result<crate::types::UnitAccelerationConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/acceleration/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        query_params.push(("value", format!("{}", value)));
        req = req.query(&query_params);
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

    #[doc = "Convert units.\n\nConvert a unit value to another metric unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitAngleFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitAngleFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_angle_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitAngleConversion = client\n        .unit()\n        .get_angle_conversion(\n            kittycad::types::UnitAngleFormat::Turn,\n            kittycad::types::UnitAngleFormat::Turn,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_angle_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitAngleFormat,
        src_format: crate::types::UnitAngleFormat,
        value: f64,
    ) -> Result<crate::types::UnitAngleConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/angle/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        query_params.push(("value", format!("{}", value)));
        req = req.query(&query_params);
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

    #[doc = "Convert units.\n\nConvert a unit value to another metric unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitAngularVelocityFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitAngularVelocityFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_angular_velocity_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitAngularVelocityConversion = client\n        .unit()\n        .get_angular_velocity_conversion(\n            kittycad::types::UnitAngularVelocityFormat::RadiansPerSecond,\n            kittycad::types::UnitAngularVelocityFormat::RevolutionsPerMinute,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_angular_velocity_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitAngularVelocityFormat,
        src_format: crate::types::UnitAngularVelocityFormat,
        value: f64,
    ) -> Result<crate::types::UnitAngularVelocityConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/angular-velocity/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        query_params.push(("value", format!("{}", value)));
        req = req.query(&query_params);
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

    #[doc = "Convert units.\n\nConvert a unit value to another metric unit value. This is a nice \
             endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: \
             crate::types::UnitAreaFormat`: The output format of the unit. (required)\n- \
             `src_format: crate::types::UnitAreaFormat`: The source format of the unit. \
             (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn \
             example_unit_get_area_conversion() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::UnitAreaConversion = client\n        .unit()\n        \
             .get_area_conversion(\n            \
             kittycad::types::UnitAreaFormat::SquareKilometer,\n            \
             kittycad::types::UnitAreaFormat::Acre,\n            3.14 as f64,\n        )\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_area_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitAreaFormat,
        src_format: crate::types::UnitAreaFormat,
        value: f64,
    ) -> Result<crate::types::UnitAreaConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/area/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        query_params.push(("value", format!("{}", value)));
        req = req.query(&query_params);
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

    #[doc = "Convert units.\n\nConvert a unit value to another metric unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitChargeFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitChargeFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_charge_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitChargeConversion = client\n        .unit()\n        .get_charge_conversion(\n            kittycad::types::UnitChargeFormat::Coulomb,\n            kittycad::types::UnitChargeFormat::Coulomb,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_charge_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitChargeFormat,
        src_format: crate::types::UnitChargeFormat,
        value: f64,
    ) -> Result<crate::types::UnitChargeConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/charge/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        query_params.push(("value", format!("{}", value)));
        req = req.query(&query_params);
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

    #[doc = "Convert units.\n\nConvert a unit value to another metric unit value. This is a nice \
             endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: \
             crate::types::UnitConcentrationFormat`: The output format of the unit. (required)\n- \
             `src_format: crate::types::UnitConcentrationFormat`: The source format of the unit. \
             (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn \
             example_unit_get_concentration_conversion() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::UnitConcentrationConversion = client\n        .unit()\n        \
             .get_concentration_conversion(\n            \
             kittycad::types::UnitConcentrationFormat::PartsPerTrillion,\n            \
             kittycad::types::UnitConcentrationFormat::Percent,\n            3.14 as f64,\n        \
             )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_concentration_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitConcentrationFormat,
        src_format: crate::types::UnitConcentrationFormat,
        value: f64,
    ) -> Result<crate::types::UnitConcentrationConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/concentration/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        query_params.push(("value", format!("{}", value)));
        req = req.query(&query_params);
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

    #[doc = "Convert units.\n\nConvert a unit value to another metric unit value. This is a nice \
             endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: \
             crate::types::UnitDataTransferRateFormat`: The output format of the unit. \
             (required)\n- `src_format: crate::types::UnitDataTransferRateFormat`: The source \
             format of the unit. (required)\n- `value: f64`: The initial value. \
             (required)\n\n```rust,no_run\nasync fn \
             example_unit_get_data_transfer_rate_conversion() -> anyhow::Result<()> {\n    let \
             client = kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::UnitDataTransferRateConversion = client\n        .unit()\n        \
             .get_data_transfer_rate_conversion(\n            \
             kittycad::types::UnitDataTransferRateFormat::ExabitsPerSecond,\n            \
             kittycad::types::UnitDataTransferRateFormat::ExabytesPerSecond,\n            3.14 as \
             f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_data_transfer_rate_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitDataTransferRateFormat,
        src_format: crate::types::UnitDataTransferRateFormat,
        value: f64,
    ) -> Result<crate::types::UnitDataTransferRateConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/data-transfer-rate/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        query_params.push(("value", format!("{}", value)));
        req = req.query(&query_params);
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

    #[doc = "Convert units.\n\nConvert a unit value to another metric unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitDataFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitDataFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_data_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitDataConversion = client\n        .unit()\n        .get_data_conversion(\n            kittycad::types::UnitDataFormat::Exabit,\n            kittycad::types::UnitDataFormat::Byte,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_data_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitDataFormat,
        src_format: crate::types::UnitDataFormat,
        value: f64,
    ) -> Result<crate::types::UnitDataConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/data/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        query_params.push(("value", format!("{}", value)));
        req = req.query(&query_params);
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

    #[doc = "Convert units.\n\nConvert a unit value to another metric unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitDensityFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitDensityFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_density_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitDensityConversion = client\n        .unit()\n        .get_density_conversion(\n            kittycad::types::UnitDensityFormat::GramsPerMilliliter,\n            kittycad::types::UnitDensityFormat::OuncesPerGallon,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_density_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitDensityFormat,
        src_format: crate::types::UnitDensityFormat,
        value: f64,
    ) -> Result<crate::types::UnitDensityConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/density/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        query_params.push(("value", format!("{}", value)));
        req = req.query(&query_params);
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

    #[doc = "Convert units.\n\nConvert a unit value to another metric unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitEnergyFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitEnergyFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_energy_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitEnergyConversion = client\n        .unit()\n        .get_energy_conversion(\n            kittycad::types::UnitEnergyFormat::BritishThermalUnit,\n            kittycad::types::UnitEnergyFormat::BritishThermalUnitIso,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_energy_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitEnergyFormat,
        src_format: crate::types::UnitEnergyFormat,
        value: f64,
    ) -> Result<crate::types::UnitEnergyConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/energy/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        query_params.push(("value", format!("{}", value)));
        req = req.query(&query_params);
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

    #[doc = "Convert units.\n\nConvert a unit value to another metric unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitForceFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitForceFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_force_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitForceConversion = client\n        .unit()\n        .get_force_conversion(\n            kittycad::types::UnitForceFormat::Dyne,\n            kittycad::types::UnitForceFormat::Newton,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_force_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitForceFormat,
        src_format: crate::types::UnitForceFormat,
        value: f64,
    ) -> Result<crate::types::UnitForceConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/force/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        query_params.push(("value", format!("{}", value)));
        req = req.query(&query_params);
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

    #[doc = "Convert units.\n\nConvert a unit value to another metric unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitIlluminanceFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitIlluminanceFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_illuminance_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitIlluminanceConversion = client\n        .unit()\n        .get_illuminance_conversion(\n            kittycad::types::UnitIlluminanceFormat::Phot,\n            kittycad::types::UnitIlluminanceFormat::Phot,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_illuminance_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitIlluminanceFormat,
        src_format: crate::types::UnitIlluminanceFormat,
        value: f64,
    ) -> Result<crate::types::UnitIlluminanceConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/illuminance/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        query_params.push(("value", format!("{}", value)));
        req = req.query(&query_params);
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

    #[doc = "Convert units.\n\nConvert a unit value to another metric unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitLengthFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitLengthFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_length_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitLengthConversion = client\n        .unit()\n        .get_length_conversion(\n            kittycad::types::UnitLengthFormat::Chain,\n            kittycad::types::UnitLengthFormat::Yard,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_length_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitLengthFormat,
        src_format: crate::types::UnitLengthFormat,
        value: f64,
    ) -> Result<crate::types::UnitLengthConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/length/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        query_params.push(("value", format!("{}", value)));
        req = req.query(&query_params);
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

    #[doc = "Convert units.\n\nConvert a unit value to another metric unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitMagneticFieldStrengthFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitMagneticFieldStrengthFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_magnetic_field_strength_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitMagneticFieldStrengthConversion = client\n        .unit()\n        .get_magnetic_field_strength_conversion(\n            kittycad::types::UnitMagneticFieldStrengthFormat::Gauss,\n            kittycad::types::UnitMagneticFieldStrengthFormat::Gauss,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_magnetic_field_strength_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitMagneticFieldStrengthFormat,
        src_format: crate::types::UnitMagneticFieldStrengthFormat,
        value: f64,
    ) -> Result<crate::types::UnitMagneticFieldStrengthConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/magnetic-field-strength/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        query_params.push(("value", format!("{}", value)));
        req = req.query(&query_params);
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

    #[doc = "Convert units.\n\nConvert a unit value to another metric unit value. This is a nice \
             endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: \
             crate::types::UnitMagneticFluxFormat`: The output format of the unit. (required)\n- \
             `src_format: crate::types::UnitMagneticFluxFormat`: The source format of the unit. \
             (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn \
             example_unit_get_magnetic_flux_conversion() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::UnitMagneticFluxConversion = client\n        .unit()\n        \
             .get_magnetic_flux_conversion(\n            \
             kittycad::types::UnitMagneticFluxFormat::Maxwell,\n            \
             kittycad::types::UnitMagneticFluxFormat::Weber,\n            3.14 as f64,\n        \
             )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_magnetic_flux_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitMagneticFluxFormat,
        src_format: crate::types::UnitMagneticFluxFormat,
        value: f64,
    ) -> Result<crate::types::UnitMagneticFluxConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/magnetic-flux/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        query_params.push(("value", format!("{}", value)));
        req = req.query(&query_params);
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

    #[doc = "Convert units.\n\nConvert a unit value to another metric unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitMassFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitMassFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_mass_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitMassConversion = client\n        .unit()\n        .get_mass_conversion(\n            kittycad::types::UnitMassFormat::MetricTon,\n            kittycad::types::UnitMassFormat::Pound,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_mass_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitMassFormat,
        src_format: crate::types::UnitMassFormat,
        value: f64,
    ) -> Result<crate::types::UnitMassConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/mass/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        query_params.push(("value", format!("{}", value)));
        req = req.query(&query_params);
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

    #[doc = "Convert units.\n\nConvert a unit value to another metric unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitMetricFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitMetricFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_metric_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitMetricConversion = client\n        .unit()\n        .get_metric_conversion(\n            kittycad::types::UnitMetricFormat::Deca,\n            kittycad::types::UnitMetricFormat::Femto,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_metric_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitMetricFormat,
        src_format: crate::types::UnitMetricFormat,
        value: f64,
    ) -> Result<crate::types::UnitMetricConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/metric/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        query_params.push(("value", format!("{}", value)));
        req = req.query(&query_params);
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

    #[doc = "Convert units.\n\nConvert a unit value to another metric unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitPowerFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitPowerFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_power_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitPowerConversion = client\n        .unit()\n        .get_power_conversion(\n            kittycad::types::UnitPowerFormat::Horsepower,\n            kittycad::types::UnitPowerFormat::Horsepower,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_power_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitPowerFormat,
        src_format: crate::types::UnitPowerFormat,
        value: f64,
    ) -> Result<crate::types::UnitPowerConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/power/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        query_params.push(("value", format!("{}", value)));
        req = req.query(&query_params);
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

    #[doc = "Convert units.\n\nConvert a unit value to another metric unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitPressureFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitPressureFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_pressure_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitPressureConversion = client\n        .unit()\n        .get_pressure_conversion(\n            kittycad::types::UnitPressureFormat::Atmosphere,\n            kittycad::types::UnitPressureFormat::Pascal,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_pressure_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitPressureFormat,
        src_format: crate::types::UnitPressureFormat,
        value: f64,
    ) -> Result<crate::types::UnitPressureConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/pressure/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        query_params.push(("value", format!("{}", value)));
        req = req.query(&query_params);
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

    #[doc = "Convert units.\n\nConvert a unit value to another metric unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitRadiationFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitRadiationFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_radiation_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitRadiationConversion = client\n        .unit()\n        .get_radiation_conversion(\n            kittycad::types::UnitRadiationFormat::Rad,\n            kittycad::types::UnitRadiationFormat::Gray,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_radiation_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitRadiationFormat,
        src_format: crate::types::UnitRadiationFormat,
        value: f64,
    ) -> Result<crate::types::UnitRadiationConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/radiation/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        query_params.push(("value", format!("{}", value)));
        req = req.query(&query_params);
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

    #[doc = "Convert units.\n\nConvert a unit value to another metric unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitSolidAngleFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitSolidAngleFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_solid_angle_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitSolidAngleConversion = client\n        .unit()\n        .get_solid_angle_conversion(\n            kittycad::types::UnitSolidAngleFormat::Spat,\n            kittycad::types::UnitSolidAngleFormat::DegreeSquared,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_solid_angle_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitSolidAngleFormat,
        src_format: crate::types::UnitSolidAngleFormat,
        value: f64,
    ) -> Result<crate::types::UnitSolidAngleConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/solid-angle/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        query_params.push(("value", format!("{}", value)));
        req = req.query(&query_params);
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

    #[doc = "Convert units.\n\nConvert a unit value to another metric unit value. This is a nice \
             endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: \
             crate::types::UnitTemperatureFormat`: The output format of the unit. (required)\n- \
             `src_format: crate::types::UnitTemperatureFormat`: The source format of the unit. \
             (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn \
             example_unit_get_temperature_conversion() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::UnitTemperatureConversion = client\n        .unit()\n        \
             .get_temperature_conversion(\n            \
             kittycad::types::UnitTemperatureFormat::Celsius,\n            \
             kittycad::types::UnitTemperatureFormat::Rankine,\n            3.14 as f64,\n        \
             )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_temperature_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitTemperatureFormat,
        src_format: crate::types::UnitTemperatureFormat,
        value: f64,
    ) -> Result<crate::types::UnitTemperatureConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/temperature/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        query_params.push(("value", format!("{}", value)));
        req = req.query(&query_params);
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

    #[doc = "Convert units.\n\nConvert a unit value to another metric unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitTimeFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitTimeFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_time_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitTimeConversion = client\n        .unit()\n        .get_time_conversion(\n            kittycad::types::UnitTimeFormat::Minute,\n            kittycad::types::UnitTimeFormat::Second,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_time_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitTimeFormat,
        src_format: crate::types::UnitTimeFormat,
        value: f64,
    ) -> Result<crate::types::UnitTimeConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/time/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        query_params.push(("value", format!("{}", value)));
        req = req.query(&query_params);
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

    #[doc = "Convert units.\n\nConvert a unit value to another metric unit value. This is a nice \
             endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: \
             crate::types::UnitVelocityFormat`: The output format of the unit. (required)\n- \
             `src_format: crate::types::UnitVelocityFormat`: The source format of the unit. \
             (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn \
             example_unit_get_velocity_conversion() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::UnitVelocityConversion = client\n        .unit()\n        \
             .get_velocity_conversion(\n            \
             kittycad::types::UnitVelocityFormat::MilesPerHour,\n            \
             kittycad::types::UnitVelocityFormat::MilesPerHour,\n            3.14 as f64,\n        \
             )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_velocity_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitVelocityFormat,
        src_format: crate::types::UnitVelocityFormat,
        value: f64,
    ) -> Result<crate::types::UnitVelocityConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/velocity/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        query_params.push(("value", format!("{}", value)));
        req = req.query(&query_params);
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

    #[doc = "Convert units.\n\nConvert a unit value to another metric unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitVoltageFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitVoltageFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_voltage_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitVoltageConversion = client\n        .unit()\n        .get_voltage_conversion(\n            kittycad::types::UnitVoltageFormat::Statvolt,\n            kittycad::types::UnitVoltageFormat::Statvolt,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_voltage_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitVoltageFormat,
        src_format: crate::types::UnitVoltageFormat,
        value: f64,
    ) -> Result<crate::types::UnitVoltageConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/voltage/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        query_params.push(("value", format!("{}", value)));
        req = req.query(&query_params);
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

    #[doc = "Convert units.\n\nConvert a unit value to another metric unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitVolumeFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitVolumeFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_volume_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitVolumeConversion = client\n        .unit()\n        .get_volume_conversion(\n            kittycad::types::UnitVolumeFormat::CubicMile,\n            kittycad::types::UnitVolumeFormat::Liter,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_volume_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitVolumeFormat,
        src_format: crate::types::UnitVolumeFormat,
        value: f64,
    ) -> Result<crate::types::UnitVolumeConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/volume/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        query_params.push(("value", format!("{}", value)));
        req = req.query(&query_params);
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

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

    #[doc = "Convert acceleration units.\n\nConvert an acceleration unit value to another acceleration unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitAccelerationFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitAccelerationFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_acceleration_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitAccelerationConversion = client\n        .unit()\n        .get_acceleration_conversion(\n            kittycad::types::UnitAccelerationFormat::StandardGravity,\n            kittycad::types::UnitAccelerationFormat::StandardGravity,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_acceleration_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitAccelerationFormat,
        src_format: crate::types::UnitAccelerationFormat,
        value: f64,
    ) -> Result<crate::types::UnitAccelerationConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/acceleration/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("value", format!("{}", value))];
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

    #[doc = "Convert angle units.\n\nConvert an angle unit value to another angle unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitAngleFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitAngleFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_angle_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitAngleConversion = client\n        .unit()\n        .get_angle_conversion(\n            kittycad::types::UnitAngleFormat::Gradian,\n            kittycad::types::UnitAngleFormat::Gradian,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_angle_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitAngleFormat,
        src_format: crate::types::UnitAngleFormat,
        value: f64,
    ) -> Result<crate::types::UnitAngleConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/angle/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("value", format!("{}", value))];
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

    #[doc = "Convert angular velocity units.\n\nConvert an angular velocity unit value to another angular velocity unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitAngularVelocityFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitAngularVelocityFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_angular_velocity_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitAngularVelocityConversion = client\n        .unit()\n        .get_angular_velocity_conversion(\n            kittycad::types::UnitAngularVelocityFormat::MilliarcsecondsPerYear,\n            kittycad::types::UnitAngularVelocityFormat::MilliarcsecondsPerYear,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_angular_velocity_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitAngularVelocityFormat,
        src_format: crate::types::UnitAngularVelocityFormat,
        value: f64,
    ) -> Result<crate::types::UnitAngularVelocityConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/angular-velocity/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("value", format!("{}", value))];
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

    #[doc = "Convert area units.\n\nConvert an area unit value to another area unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitAreaFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitAreaFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_area_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitAreaConversion = client\n        .unit()\n        .get_area_conversion(\n            kittycad::types::UnitAreaFormat::Acre,\n            kittycad::types::UnitAreaFormat::Acre,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_area_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitAreaFormat,
        src_format: crate::types::UnitAreaFormat,
        value: f64,
    ) -> Result<crate::types::UnitAreaConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/area/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("value", format!("{}", value))];
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

    #[doc = "Convert charge units.\n\nConvert a charge unit value to another charge unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitChargeFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitChargeFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_charge_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitChargeConversion = client\n        .unit()\n        .get_charge_conversion(\n            kittycad::types::UnitChargeFormat::AmpereHour,\n            kittycad::types::UnitChargeFormat::AmpereHour,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_charge_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitChargeFormat,
        src_format: crate::types::UnitChargeFormat,
        value: f64,
    ) -> Result<crate::types::UnitChargeConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/charge/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("value", format!("{}", value))];
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

    #[doc = "Convert concentration units.\n\nConvert a concentration unit value to another \
             concentration unit value. This is a nice endpoint to use for helper \
             functions.\n\n**Parameters:**\n\n- `output_format: \
             crate::types::UnitConcentrationFormat`: The output format of the unit. (required)\n- \
             `src_format: crate::types::UnitConcentrationFormat`: The source format of the unit. \
             (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn \
             example_unit_get_concentration_conversion() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::UnitConcentrationConversion = client\n        .unit()\n        \
             .get_concentration_conversion(\n            \
             kittycad::types::UnitConcentrationFormat::Percent,\n            \
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
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/concentration/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("value", format!("{}", value))];
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

    #[doc = "Convert data transfer rate units.\n\nConvert a data transfer rate unit value to \
             another data transfer rate unit value. This is a nice endpoint to use for helper \
             functions.\n\n**Parameters:**\n\n- `output_format: \
             crate::types::UnitDataTransferRateFormat`: The output format of the unit. \
             (required)\n- `src_format: crate::types::UnitDataTransferRateFormat`: The source \
             format of the unit. (required)\n- `value: f64`: The initial value. \
             (required)\n\n```rust,no_run\nasync fn \
             example_unit_get_data_transfer_rate_conversion() -> anyhow::Result<()> {\n    let \
             client = kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::UnitDataTransferRateConversion = client\n        .unit()\n        \
             .get_data_transfer_rate_conversion(\n            \
             kittycad::types::UnitDataTransferRateFormat::ExabitsPerSecond,\n            \
             kittycad::types::UnitDataTransferRateFormat::ExabitsPerSecond,\n            3.14 as \
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
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/data-transfer-rate/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("value", format!("{}", value))];
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

    #[doc = "Convert data units.\n\nConvert a data unit value to another data unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitDataFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitDataFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_data_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitDataConversion = client\n        .unit()\n        .get_data_conversion(\n            kittycad::types::UnitDataFormat::Exabit,\n            kittycad::types::UnitDataFormat::Exabit,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_data_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitDataFormat,
        src_format: crate::types::UnitDataFormat,
        value: f64,
    ) -> Result<crate::types::UnitDataConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/data/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("value", format!("{}", value))];
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

    #[doc = "Convert density units.\n\nConvert a density unit value to another density unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitDensityFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitDensityFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_density_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitDensityConversion = client\n        .unit()\n        .get_density_conversion(\n            kittycad::types::UnitDensityFormat::PoundsPerGallon,\n            kittycad::types::UnitDensityFormat::PoundsPerGallon,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_density_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitDensityFormat,
        src_format: crate::types::UnitDensityFormat,
        value: f64,
    ) -> Result<crate::types::UnitDensityConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/density/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("value", format!("{}", value))];
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

    #[doc = "Convert energy units.\n\nConvert a energy unit value to another energy unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitEnergyFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitEnergyFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_energy_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitEnergyConversion = client\n        .unit()\n        .get_energy_conversion(\n            kittycad::types::UnitEnergyFormat::FootPound,\n            kittycad::types::UnitEnergyFormat::FootPound,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_energy_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitEnergyFormat,
        src_format: crate::types::UnitEnergyFormat,
        value: f64,
    ) -> Result<crate::types::UnitEnergyConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/energy/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("value", format!("{}", value))];
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

    #[doc = "Convert force units.\n\nConvert a force unit value to another force unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitForceFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitForceFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_force_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitForceConversion = client\n        .unit()\n        .get_force_conversion(\n            kittycad::types::UnitForceFormat::Poundal,\n            kittycad::types::UnitForceFormat::Poundal,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_force_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitForceFormat,
        src_format: crate::types::UnitForceFormat,
        value: f64,
    ) -> Result<crate::types::UnitForceConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/force/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("value", format!("{}", value))];
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

    #[doc = "Convert illuminance units.\n\nConvert a illuminance unit value to another illuminance unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitIlluminanceFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitIlluminanceFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_illuminance_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitIlluminanceConversion = client\n        .unit()\n        .get_illuminance_conversion(\n            kittycad::types::UnitIlluminanceFormat::Phot,\n            kittycad::types::UnitIlluminanceFormat::Phot,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_illuminance_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitIlluminanceFormat,
        src_format: crate::types::UnitIlluminanceFormat,
        value: f64,
    ) -> Result<crate::types::UnitIlluminanceConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/illuminance/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("value", format!("{}", value))];
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

    #[doc = "Convert length units.\n\nConvert a length unit value to another length unit value. \
             This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- \
             `output_format: crate::types::UnitLengthFormat`: The output format of the unit. \
             (required)\n- `src_format: crate::types::UnitLengthFormat`: The source format of the \
             unit. (required)\n- `value: f64`: The initial value. \
             (required)\n\n```rust,no_run\nasync fn example_unit_get_length_conversion() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::UnitLengthConversion = client\n        .unit()\n        \
             .get_length_conversion(\n            \
             kittycad::types::UnitLengthFormat::NauticalLeague,\n            \
             kittycad::types::UnitLengthFormat::NauticalLeague,\n            3.14 as f64,\n        \
             )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_length_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitLengthFormat,
        src_format: crate::types::UnitLengthFormat,
        value: f64,
    ) -> Result<crate::types::UnitLengthConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/length/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("value", format!("{}", value))];
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

    #[doc = "Convert magnetic field strength units.\n\nConvert a magnetic field strength unit value to another magnetic field strength unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitMagneticFieldStrengthFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitMagneticFieldStrengthFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_magnetic_field_strength_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitMagneticFieldStrengthConversion = client\n        .unit()\n        .get_magnetic_field_strength_conversion(\n            kittycad::types::UnitMagneticFieldStrengthFormat::Gauss,\n            kittycad::types::UnitMagneticFieldStrengthFormat::Gauss,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_magnetic_field_strength_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitMagneticFieldStrengthFormat,
        src_format: crate::types::UnitMagneticFieldStrengthFormat,
        value: f64,
    ) -> Result<crate::types::UnitMagneticFieldStrengthConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/magnetic-field-strength/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("value", format!("{}", value))];
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

    #[doc = "Convert magnetic flux units.\n\nConvert a magnetic flux unit value to another \
             magnetic flux unit value. This is a nice endpoint to use for helper \
             functions.\n\n**Parameters:**\n\n- `output_format: \
             crate::types::UnitMagneticFluxFormat`: The output format of the unit. (required)\n- \
             `src_format: crate::types::UnitMagneticFluxFormat`: The source format of the unit. \
             (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn \
             example_unit_get_magnetic_flux_conversion() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::UnitMagneticFluxConversion = client\n        .unit()\n        \
             .get_magnetic_flux_conversion(\n            \
             kittycad::types::UnitMagneticFluxFormat::Maxwell,\n            \
             kittycad::types::UnitMagneticFluxFormat::Maxwell,\n            3.14 as f64,\n        \
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
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/magnetic-flux/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("value", format!("{}", value))];
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

    #[doc = "Convert mass units.\n\nConvert a mass unit value to another mass unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitMassFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitMassFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_mass_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitMassConversion = client\n        .unit()\n        .get_mass_conversion(\n            kittycad::types::UnitMassFormat::Carat,\n            kittycad::types::UnitMassFormat::Carat,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_mass_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitMassFormat,
        src_format: crate::types::UnitMassFormat,
        value: f64,
    ) -> Result<crate::types::UnitMassConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/mass/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("value", format!("{}", value))];
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

    #[doc = "Convert metric cubed units.\n\nConvert a metric cubed unit value to another metric \
             cubed unit value. This is a nice endpoint to use for helper \
             functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitMetricPower`: \
             The output format of the unit. (required)\n- `src_format: \
             crate::types::UnitMetricPower`: The source format of the unit. (required)\n- `value: \
             f64`: The initial value. (required)\n\n```rust,no_run\nasync fn \
             example_unit_get_metric_power_cubed_conversion() -> anyhow::Result<()> {\n    let \
             client = kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::UnitMetricPowerCubedConversion = client\n        .unit()\n        \
             .get_metric_power_cubed_conversion(\n            \
             kittycad::types::UnitMetricPower::Tera,\n            \
             kittycad::types::UnitMetricPower::Tera,\n            3.14 as f64,\n        )\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_metric_power_cubed_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitMetricPower,
        src_format: crate::types::UnitMetricPower,
        value: f64,
    ) -> Result<crate::types::UnitMetricPowerCubedConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/metric/cubed/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("value", format!("{}", value))];
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

    #[doc = "Convert metric units.\n\nConvert a metric unit value to another metric unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitMetricPower`: The output format of the unit. (required)\n- `src_format: crate::types::UnitMetricPower`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_metric_power_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitMetricPowerConversion = client\n        .unit()\n        .get_metric_power_conversion(\n            kittycad::types::UnitMetricPower::Tera,\n            kittycad::types::UnitMetricPower::Tera,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_metric_power_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitMetricPower,
        src_format: crate::types::UnitMetricPower,
        value: f64,
    ) -> Result<crate::types::UnitMetricPowerConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/metric/power/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("value", format!("{}", value))];
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

    #[doc = "Convert metric squared units.\n\nConvert a metric squared unit value to another \
             metric squared unit value. This is a nice endpoint to use for helper \
             functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitMetricPower`: \
             The output format of the unit. (required)\n- `src_format: \
             crate::types::UnitMetricPower`: The source format of the unit. (required)\n- `value: \
             f64`: The initial value. (required)\n\n```rust,no_run\nasync fn \
             example_unit_get_metric_power_squared_conversion() -> anyhow::Result<()> {\n    let \
             client = kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::UnitMetricPowerSquaredConversion = client\n        .unit()\n        \
             .get_metric_power_squared_conversion(\n            \
             kittycad::types::UnitMetricPower::Tera,\n            \
             kittycad::types::UnitMetricPower::Tera,\n            3.14 as f64,\n        )\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_metric_power_squared_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitMetricPower,
        src_format: crate::types::UnitMetricPower,
        value: f64,
    ) -> Result<crate::types::UnitMetricPowerSquaredConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/metric/squared/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("value", format!("{}", value))];
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

    #[doc = "Convert power units.\n\nConvert a power unit value to another power unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitPowerFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitPowerFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_power_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitPowerConversion = client\n        .unit()\n        .get_power_conversion(\n            kittycad::types::UnitPowerFormat::Milliwatt,\n            kittycad::types::UnitPowerFormat::Milliwatt,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_power_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitPowerFormat,
        src_format: crate::types::UnitPowerFormat,
        value: f64,
    ) -> Result<crate::types::UnitPowerConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/power/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("value", format!("{}", value))];
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

    #[doc = "Convert pressure units.\n\nConvert a pressure unit value to another pressure unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitPressureFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitPressureFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_pressure_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitPressureConversion = client\n        .unit()\n        .get_pressure_conversion(\n            kittycad::types::UnitPressureFormat::PoundsPerSquareInch,\n            kittycad::types::UnitPressureFormat::PoundsPerSquareInch,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_pressure_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitPressureFormat,
        src_format: crate::types::UnitPressureFormat,
        value: f64,
    ) -> Result<crate::types::UnitPressureConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/pressure/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("value", format!("{}", value))];
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

    #[doc = "Convert radiation units.\n\nConvert a radiation unit value to another radiation unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitRadiationFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitRadiationFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_radiation_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitRadiationConversion = client\n        .unit()\n        .get_radiation_conversion(\n            kittycad::types::UnitRadiationFormat::Rad,\n            kittycad::types::UnitRadiationFormat::Rad,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_radiation_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitRadiationFormat,
        src_format: crate::types::UnitRadiationFormat,
        value: f64,
    ) -> Result<crate::types::UnitRadiationConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/radiation/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("value", format!("{}", value))];
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

    #[doc = "Convert radioactivity units.\n\nConvert a radioactivity unit value to another radioactivity unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitRadioactivityFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitRadioactivityFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_radioactivity_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitRadioactivityConversion = client\n        .unit()\n        .get_radioactivity_conversion(\n            kittycad::types::UnitRadioactivityFormat::Rutherford,\n            kittycad::types::UnitRadioactivityFormat::Rutherford,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_radioactivity_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitRadioactivityFormat,
        src_format: crate::types::UnitRadioactivityFormat,
        value: f64,
    ) -> Result<crate::types::UnitRadioactivityConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/radioactivity/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("value", format!("{}", value))];
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

    #[doc = "Convert solid angle units.\n\nConvert a solid angle unit value to another solid angle unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitSolidAngleFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitSolidAngleFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_solid_angle_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitSolidAngleConversion = client\n        .unit()\n        .get_solid_angle_conversion(\n            kittycad::types::UnitSolidAngleFormat::Spat,\n            kittycad::types::UnitSolidAngleFormat::Spat,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_solid_angle_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitSolidAngleFormat,
        src_format: crate::types::UnitSolidAngleFormat,
        value: f64,
    ) -> Result<crate::types::UnitSolidAngleConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/solid-angle/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("value", format!("{}", value))];
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

    #[doc = "Convert temperature units.\n\nConvert a temperature unit value to another temperature \
             unit value. This is a nice endpoint to use for helper \
             functions.\n\n**Parameters:**\n\n- `output_format: \
             crate::types::UnitTemperatureFormat`: The output format of the unit. (required)\n- \
             `src_format: crate::types::UnitTemperatureFormat`: The source format of the unit. \
             (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn \
             example_unit_get_temperature_conversion() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::UnitTemperatureConversion = client\n        .unit()\n        \
             .get_temperature_conversion(\n            \
             kittycad::types::UnitTemperatureFormat::Rankine,\n            \
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
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/temperature/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("value", format!("{}", value))];
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

    #[doc = "Convert time units.\n\nConvert a time unit value to another time unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitTimeFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitTimeFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_time_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitTimeConversion = client\n        .unit()\n        .get_time_conversion(\n            kittycad::types::UnitTimeFormat::GregorianYear,\n            kittycad::types::UnitTimeFormat::GregorianYear,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_time_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitTimeFormat,
        src_format: crate::types::UnitTimeFormat,
        value: f64,
    ) -> Result<crate::types::UnitTimeConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/time/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("value", format!("{}", value))];
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

    #[doc = "Convert velocity units.\n\nConvert a velocity unit value to another velocity unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitVelocityFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitVelocityFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_velocity_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitVelocityConversion = client\n        .unit()\n        .get_velocity_conversion(\n            kittycad::types::UnitVelocityFormat::Knot,\n            kittycad::types::UnitVelocityFormat::Knot,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_velocity_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitVelocityFormat,
        src_format: crate::types::UnitVelocityFormat,
        value: f64,
    ) -> Result<crate::types::UnitVelocityConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/velocity/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("value", format!("{}", value))];
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

    #[doc = "Convert voltage units.\n\nConvert a voltage unit value to another voltage unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitVoltageFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitVoltageFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_voltage_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitVoltageConversion = client\n        .unit()\n        .get_voltage_conversion(\n            kittycad::types::UnitVoltageFormat::Abvolt,\n            kittycad::types::UnitVoltageFormat::Abvolt,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_voltage_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitVoltageFormat,
        src_format: crate::types::UnitVoltageFormat,
        value: f64,
    ) -> Result<crate::types::UnitVoltageConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/voltage/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("value", format!("{}", value))];
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

    #[doc = "Convert volume units.\n\nConvert a volume unit value to another volume unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `output_format: crate::types::UnitVolumeFormat`: The output format of the unit. (required)\n- `src_format: crate::types::UnitVolumeFormat`: The source format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_volume_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitVolumeConversion = client\n        .unit()\n        .get_volume_conversion(\n            kittycad::types::UnitVolumeFormat::Gill,\n            kittycad::types::UnitVolumeFormat::Gill,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_volume_conversion<'a>(
        &'a self,
        output_format: crate::types::UnitVolumeFormat,
        src_format: crate::types::UnitVolumeFormat,
        value: f64,
    ) -> Result<crate::types::UnitVolumeConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/volume/{src_format}/{output_format}"
                    .replace("{output_format}", &format!("{}", output_format))
                    .replace("{src_format}", &format!("{}", src_format))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("value", format!("{}", value))];
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

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

    #[doc = "Convert angle units.\n\nConvert an angle unit value to another angle unit value. This \
             is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `input_unit: \
             crate::types::UnitAngle`: The source format of the unit. (required)\n- `output_unit: \
             crate::types::UnitAngle`: The output format of the unit. (required)\n- `value: f64`: \
             The initial value. (required)\n\n```rust,no_run\nasync fn \
             example_unit_get_angle_conversion() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::UnitAngleConversion = client\n        .unit()\n        \
             .get_angle_conversion(\n            kittycad::types::UnitAngle::Radians,\n            \
             kittycad::types::UnitAngle::Radians,\n            3.14 as f64,\n        )\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_angle_conversion<'a>(
        &'a self,
        input_unit: crate::types::UnitAngle,
        output_unit: crate::types::UnitAngle,
        value: f64,
    ) -> Result<crate::types::UnitAngleConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/angle/{input_unit}/{output_unit}"
                    .replace("{input_unit}", &format!("{}", input_unit))
                    .replace("{output_unit}", &format!("{}", output_unit))
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Convert area units.\n\nConvert an area unit value to another area unit value. This is \
             a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `input_unit: \
             crate::types::UnitArea`: The source format of the unit. (required)\n- `output_unit: \
             crate::types::UnitArea`: The output format of the unit. (required)\n- `value: f64`: \
             The initial value. (required)\n\n```rust,no_run\nasync fn \
             example_unit_get_area_conversion() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::UnitAreaConversion = client\n        .unit()\n        \
             .get_area_conversion(\n            kittycad::types::UnitArea::Yd2,\n            \
             kittycad::types::UnitArea::Yd2,\n            3.14 as f64,\n        )\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_area_conversion<'a>(
        &'a self,
        input_unit: crate::types::UnitArea,
        output_unit: crate::types::UnitArea,
        value: f64,
    ) -> Result<crate::types::UnitAreaConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/area/{input_unit}/{output_unit}"
                    .replace("{input_unit}", &format!("{}", input_unit))
                    .replace("{output_unit}", &format!("{}", output_unit))
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Convert current units.\n\nConvert a current unit value to another current unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `input_unit: crate::types::UnitCurrent`: The source format of the unit. (required)\n- `output_unit: crate::types::UnitCurrent`: The output format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_current_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitCurrentConversion = client\n        .unit()\n        .get_current_conversion(\n            kittycad::types::UnitCurrent::Nanoamperes,\n            kittycad::types::UnitCurrent::Nanoamperes,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_current_conversion<'a>(
        &'a self,
        input_unit: crate::types::UnitCurrent,
        output_unit: crate::types::UnitCurrent,
        value: f64,
    ) -> Result<crate::types::UnitCurrentConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/current/{input_unit}/{output_unit}"
                    .replace("{input_unit}", &format!("{}", input_unit))
                    .replace("{output_unit}", &format!("{}", output_unit))
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Convert energy units.\n\nConvert a energy unit value to another energy unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `input_unit: crate::types::UnitEnergy`: The source format of the unit. (required)\n- `output_unit: crate::types::UnitEnergy`: The output format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_energy_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitEnergyConversion = client\n        .unit()\n        .get_energy_conversion(\n            kittycad::types::UnitEnergy::WattHours,\n            kittycad::types::UnitEnergy::WattHours,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_energy_conversion<'a>(
        &'a self,
        input_unit: crate::types::UnitEnergy,
        output_unit: crate::types::UnitEnergy,
        value: f64,
    ) -> Result<crate::types::UnitEnergyConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/energy/{input_unit}/{output_unit}"
                    .replace("{input_unit}", &format!("{}", input_unit))
                    .replace("{output_unit}", &format!("{}", output_unit))
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Convert force units.\n\nConvert a force unit value to another force unit value. This \
             is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `input_unit: \
             crate::types::UnitForce`: The source format of the unit. (required)\n- `output_unit: \
             crate::types::UnitForce`: The output format of the unit. (required)\n- `value: f64`: \
             The initial value. (required)\n\n```rust,no_run\nasync fn \
             example_unit_get_force_conversion() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::UnitForceConversion = client\n        .unit()\n        \
             .get_force_conversion(\n            kittycad::types::UnitForce::Pounds,\n            \
             kittycad::types::UnitForce::Pounds,\n            3.14 as f64,\n        )\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_force_conversion<'a>(
        &'a self,
        input_unit: crate::types::UnitForce,
        output_unit: crate::types::UnitForce,
        value: f64,
    ) -> Result<crate::types::UnitForceConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/force/{input_unit}/{output_unit}"
                    .replace("{input_unit}", &format!("{}", input_unit))
                    .replace("{output_unit}", &format!("{}", output_unit))
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Convert frequency units.\n\nConvert a frequency unit value to another frequency unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `input_unit: crate::types::UnitFrequency`: The source format of the unit. (required)\n- `output_unit: crate::types::UnitFrequency`: The output format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_frequency_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitFrequencyConversion = client\n        .unit()\n        .get_frequency_conversion(\n            kittycad::types::UnitFrequency::Terahertz,\n            kittycad::types::UnitFrequency::Terahertz,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_frequency_conversion<'a>(
        &'a self,
        input_unit: crate::types::UnitFrequency,
        output_unit: crate::types::UnitFrequency,
        value: f64,
    ) -> Result<crate::types::UnitFrequencyConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/frequency/{input_unit}/{output_unit}"
                    .replace("{input_unit}", &format!("{}", input_unit))
                    .replace("{output_unit}", &format!("{}", output_unit))
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Convert length units.\n\nConvert a length unit value to another length unit value. \
             This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- \
             `input_unit: crate::types::UnitLength`: The source format of the unit. (required)\n- \
             `output_unit: crate::types::UnitLength`: The output format of the unit. (required)\n- \
             `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn \
             example_unit_get_length_conversion() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::UnitLengthConversion = client\n        .unit()\n        \
             .get_length_conversion(\n            kittycad::types::UnitLength::Yd,\n            \
             kittycad::types::UnitLength::Yd,\n            3.14 as f64,\n        )\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_length_conversion<'a>(
        &'a self,
        input_unit: crate::types::UnitLength,
        output_unit: crate::types::UnitLength,
        value: f64,
    ) -> Result<crate::types::UnitLengthConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/length/{input_unit}/{output_unit}"
                    .replace("{input_unit}", &format!("{}", input_unit))
                    .replace("{output_unit}", &format!("{}", output_unit))
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Convert mass units.\n\nConvert a mass unit value to another mass unit value. This is \
             a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `input_unit: \
             crate::types::UnitMass`: The source format of the unit. (required)\n- `output_unit: \
             crate::types::UnitMass`: The output format of the unit. (required)\n- `value: f64`: \
             The initial value. (required)\n\n```rust,no_run\nasync fn \
             example_unit_get_mass_conversion() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::UnitMassConversion = client\n        .unit()\n        \
             .get_mass_conversion(\n            kittycad::types::UnitMass::Lb,\n            \
             kittycad::types::UnitMass::Lb,\n            3.14 as f64,\n        )\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_mass_conversion<'a>(
        &'a self,
        input_unit: crate::types::UnitMass,
        output_unit: crate::types::UnitMass,
        value: f64,
    ) -> Result<crate::types::UnitMassConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/mass/{input_unit}/{output_unit}"
                    .replace("{input_unit}", &format!("{}", input_unit))
                    .replace("{output_unit}", &format!("{}", output_unit))
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Convert power units.\n\nConvert a power unit value to another power unit value. This \
             is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `input_unit: \
             crate::types::UnitPower`: The source format of the unit. (required)\n- `output_unit: \
             crate::types::UnitPower`: The output format of the unit. (required)\n- `value: f64`: \
             The initial value. (required)\n\n```rust,no_run\nasync fn \
             example_unit_get_power_conversion() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::UnitPowerConversion = client\n        .unit()\n        \
             .get_power_conversion(\n            kittycad::types::UnitPower::Watts,\n            \
             kittycad::types::UnitPower::Watts,\n            3.14 as f64,\n        )\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_power_conversion<'a>(
        &'a self,
        input_unit: crate::types::UnitPower,
        output_unit: crate::types::UnitPower,
        value: f64,
    ) -> Result<crate::types::UnitPowerConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/power/{input_unit}/{output_unit}"
                    .replace("{input_unit}", &format!("{}", input_unit))
                    .replace("{output_unit}", &format!("{}", output_unit))
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Convert pressure units.\n\nConvert a pressure unit value to another pressure unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `input_unit: crate::types::UnitPressure`: The source format of the unit. (required)\n- `output_unit: crate::types::UnitPressure`: The output format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_pressure_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitPressureConversion = client\n        .unit()\n        .get_pressure_conversion(\n            kittycad::types::UnitPressure::Psi,\n            kittycad::types::UnitPressure::Psi,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_pressure_conversion<'a>(
        &'a self,
        input_unit: crate::types::UnitPressure,
        output_unit: crate::types::UnitPressure,
        value: f64,
    ) -> Result<crate::types::UnitPressureConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/pressure/{input_unit}/{output_unit}"
                    .replace("{input_unit}", &format!("{}", input_unit))
                    .replace("{output_unit}", &format!("{}", output_unit))
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Convert temperature units.\n\nConvert a temperature unit value to another temperature unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `input_unit: crate::types::UnitTemperature`: The source format of the unit. (required)\n- `output_unit: crate::types::UnitTemperature`: The output format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_temperature_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitTemperatureConversion = client\n        .unit()\n        .get_temperature_conversion(\n            kittycad::types::UnitTemperature::Rankine,\n            kittycad::types::UnitTemperature::Rankine,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_temperature_conversion<'a>(
        &'a self,
        input_unit: crate::types::UnitTemperature,
        output_unit: crate::types::UnitTemperature,
        value: f64,
    ) -> Result<crate::types::UnitTemperatureConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/temperature/{input_unit}/{output_unit}"
                    .replace("{input_unit}", &format!("{}", input_unit))
                    .replace("{output_unit}", &format!("{}", output_unit))
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Convert torque units.\n\nConvert a torque unit value to another torque unit value. This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- `input_unit: crate::types::UnitTorque`: The source format of the unit. (required)\n- `output_unit: crate::types::UnitTorque`: The output format of the unit. (required)\n- `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn example_unit_get_torque_conversion() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UnitTorqueConversion = client\n        .unit()\n        .get_torque_conversion(\n            kittycad::types::UnitTorque::PoundFoot,\n            kittycad::types::UnitTorque::PoundFoot,\n            3.14 as f64,\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_torque_conversion<'a>(
        &'a self,
        input_unit: crate::types::UnitTorque,
        output_unit: crate::types::UnitTorque,
        value: f64,
    ) -> Result<crate::types::UnitTorqueConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/torque/{input_unit}/{output_unit}"
                    .replace("{input_unit}", &format!("{}", input_unit))
                    .replace("{output_unit}", &format!("{}", output_unit))
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Convert volume units.\n\nConvert a volume unit value to another volume unit value. \
             This is a nice endpoint to use for helper functions.\n\n**Parameters:**\n\n- \
             `input_unit: crate::types::UnitVolume`: The source format of the unit. (required)\n- \
             `output_unit: crate::types::UnitVolume`: The output format of the unit. (required)\n- \
             `value: f64`: The initial value. (required)\n\n```rust,no_run\nasync fn \
             example_unit_get_volume_conversion() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::UnitVolumeConversion = client\n        .unit()\n        \
             .get_volume_conversion(\n            kittycad::types::UnitVolume::Ml,\n            \
             kittycad::types::UnitVolume::Ml,\n            3.14 as f64,\n        )\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_volume_conversion<'a>(
        &'a self,
        input_unit: crate::types::UnitVolume,
        output_unit: crate::types::UnitVolume,
        value: f64,
    ) -> Result<crate::types::UnitVolumeConversion, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "unit/conversion/volume/{input_unit}/{output_unit}"
                    .replace("{input_unit}", &format!("{}", input_unit))
                    .replace("{output_unit}", &format!("{}", output_unit))
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }
}

use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Factory {
    pub client: Client,
}

impl Factory {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List finishes currently available for customer Factory submissions.\n\nInternal-only \
             entries are omitted. Clients should refetch this endpoint after a catalog validation \
             error before asking the customer to choose again.\n\n```rust,no_run\nasync fn \
             example_factory_get_user_finishes() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: \
             Vec<kittycad::types::FactoryCustomerCatalogOption> =\n        \
             client.factory().get_user_finishes().await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_user_finishes<'a>(
        &'a self,
    ) -> Result<Vec<crate::types::FactoryCustomerCatalogOption>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "user/factory/finishes"),
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Submit a part for manufacturing. Requires a signed-in Zoo account.\n\nThe request is `multipart/form-data`: - one JSON part named `body` (`FactoryIntakeForm`) whose `fields` object holds   intake data (material, finish, quantity, notes, …). Material and finish   are required customer-visible catalog names; all other fields are stored   verbatim so they can be added or renamed without an API change. - one or more file parts (any part name). At least one file is required.\n\nThe submitter's identity (email, name, user id) comes from the authenticated account, not the form.\n\nFetch `GET /user/factory/materials` and `GET /user/factory/finishes`, then send the returned exact `material` and `finish` names. The server rejects missing, non-string, unknown, deleted, and internal-only choices with these stable field-specific `error_code` values: - `factory_material_input_missing` - `factory_material_input_invalid_type` - `factory_material_not_found` - `factory_material_not_customer_visible` - `factory_finish_input_missing` - `factory_finish_input_invalid_type` - `factory_finish_not_found` - `factory_finish_not_customer_visible` - `quantity`: a positive integer.\n\nExample `body` part: ```ignorejson { \"fields\": { \"material\": \"6061 Aluminum\", \"finish\": \"Anodized\", \"quantity\": 10, \"notes\": \"deburr all edges\" } } ```ignore\n\nExample request (curl): ```ignore curl -X POST https://api.zoo.dev/user/factory/jobs \\   -H \"Authorization: Bearer $ZOO_API_TOKEN\" \\   -F 'body={\"fields\":{\"material\":\"6061 Aluminum\",\"finish\":\"Anodized\",\"quantity\":10}};type=application/json' \\   -F 'file=@bracket.step' ```ignore\n\nReturns `201` with the created job (`FactoryJobResponse`).\n\n```rust,no_run\nasync fn example_factory_create_user_job() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::FactoryJobResponse = client\n        .factory()\n        .create_user_job(vec![kittycad::types::multipart::Attachment {\n            name: \"thing\".to_string(),\n            filepath: Some(\"myfile.json\".into()),\n            content_type: Some(\"application/json\".to_string()),\n            data: std::fs::read(\"myfile.json\").unwrap(),\n        }])\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_user_job<'a>(
        &'a self,
        attachments: Vec<crate::types::multipart::Attachment>,
    ) -> Result<crate::types::FactoryJobResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "user/factory/jobs"),
        );
        req = req.bearer_auth(&self.client.token);
        use std::convert::TryInto;
        let mut form = reqwest::multipart::Form::new();
        for attachment in attachments {
            form = form.part(attachment.name.clone(), attachment.try_into()?);
        }

        req = req.multipart(form);
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

    #[doc = "List materials currently available for customer Factory submissions.\n\nInternal-only \
             entries are omitted. Clients should refetch this endpoint after a catalog validation \
             error before asking the customer to choose again.\n\n```rust,no_run\nasync fn \
             example_factory_get_user_materials() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: \
             Vec<kittycad::types::FactoryCustomerCatalogOption> =\n        \
             client.factory().get_user_materials().await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_user_materials<'a>(
        &'a self,
    ) -> Result<Vec<crate::types::FactoryCustomerCatalogOption>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "user/factory/materials"),
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
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }
}

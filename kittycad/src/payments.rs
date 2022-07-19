use crate::Client;
use anyhow::Result;
pub struct Payments {
    pub client: Client,
}

impl Payments {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get payment info about your user.\n\nThis includes billing address, phone, and name.\nThis endpoint requires authentication by any KittyCAD user. It gets the payment information for the authenticated user.\n\n```rust,no_run\nasync fn example_payments_get_payment_information_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::Customer =\n        client.payments().get_payment_information_for_user().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n```"]
    pub async fn get_payment_information_for_user<'a>(
        &'a self,
    ) -> Result<crate::types::Customer, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "user/payment"),
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

    #[doc = "Update payment info for your user.\n\nThis includes billing address, phone, and name.\nThis endpoint requires authentication by any KittyCAD user. It updates the payment information for the authenticated user.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_payments_update_payment_information_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::Customer = client\n        .payments()\n        .update_payment_information_for_user(&kittycad::types::BillingInfo {\n            address: Some(kittycad::types::Address {\n                city: Some(\"\".to_string()),\n                country: Some(\"cjsc\".to_string()),\n                created_at: chrono::DateTime::<chrono::Utc>::parse_from_rfc3339(\n                    \"2054-02-25T18:46:46.611+00:00\",\n                )?,\n                id: uuid::Uuid::from_str(\"afd18e0a-468d-4ab5-a8c2-8509ae02a890\")?,\n                state: Some(\"d\".to_string()),\n                street_1: Some(\"\".to_string()),\n                street_2: Some(\"q\".to_string()),\n                updated_at: chrono::DateTime::<chrono::Utc>::parse_from_rfc3339(\n                    \"2026-12-18T17:35:06.275+00:00\",\n                )?,\n                user_id: Some(\"\".to_string()),\n                zip: Some(\"\".to_string()),\n            }),\n            name: Some(\"oank\".to_string()),\n            phone: kittycad::types::phone_number::PhoneNumber::from_str(\"+1 617-885-3382\")?,\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n```"]
    pub async fn update_payment_information_for_user<'a>(
        &'a self,
        body: &crate::types::BillingInfo,
    ) -> Result<crate::types::Customer, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            &format!("{}/{}", self.client.base_url, "user/payment"),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.json(body);
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

    #[doc = "Create payment info for your user.\n\nThis includes billing address, phone, and name.\nThis endpoint requires authentication by any KittyCAD user. It creates the payment information for the authenticated user.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_payments_create_payment_information_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::Customer = client\n        .payments()\n        .create_payment_information_for_user(&kittycad::types::BillingInfo {\n            address: Some(kittycad::types::Address {\n                city: Some(\"pht\".to_string()),\n                country: Some(\"xv\".to_string()),\n                created_at: chrono::DateTime::<chrono::Utc>::parse_from_rfc3339(\n                    \"2071-11-26T00:27:04.016+00:00\",\n                )?,\n                id: uuid::Uuid::from_str(\"869469aa-66cb-4d60-92d6-5678cad27202\")?,\n                state: Some(\"tu\".to_string()),\n                street_1: Some(\"qwsfuuo\".to_string()),\n                street_2: Some(\"ikvheoxw\".to_string()),\n                updated_at: chrono::DateTime::<chrono::Utc>::parse_from_rfc3339(\n                    \"2000-12-26T03:26:10.885+00:00\",\n                )?,\n                user_id: Some(\"twhwnex\".to_string()),\n                zip: Some(\"imvd\".to_string()),\n            }),\n            name: Some(\"ypr\".to_string()),\n            phone: kittycad::types::phone_number::PhoneNumber::from_str(\"+1 470-484-6302\")?,\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n```"]
    pub async fn create_payment_information_for_user<'a>(
        &'a self,
        body: &crate::types::BillingInfo,
    ) -> Result<crate::types::Customer, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "user/payment"),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.json(body);
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

    #[doc = "Delete payment info for your user.\n\nThis includes billing address, phone, and name.\nThis endpoint requires authentication by any KittyCAD user. It deletes the payment information for the authenticated user.\n\n```rust,no_run\nasync fn example_payments_delete_payment_information_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client\n        .payments()\n        .delete_payment_information_for_user()\n        .await?;\n    Ok(())\n}\n\n```"]
    pub async fn delete_payment_information_for_user<'a>(
        &'a self,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            &format!("{}/{}", self.client.base_url, "user/payment"),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Get balance for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It gets the balance information for the authenticated user.\n\n```rust,no_run\nasync fn example_payments_get_payment_balance_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::CustomerBalance =\n        client.payments().get_payment_balance_for_user().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n```"]
    pub async fn get_payment_balance_for_user<'a>(
        &'a self,
    ) -> Result<crate::types::CustomerBalance, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "user/payment/balance"),
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

    #[doc = "Create a payment intent for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It creates a new payment intent for the authenticated user.\n\n```rust,no_run\nasync fn example_payments_create_payment_intent_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::PaymentIntent =\n        client.payments().create_payment_intent_for_user().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n```"]
    pub async fn create_payment_intent_for_user<'a>(
        &'a self,
    ) -> Result<crate::types::PaymentIntent, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "user/payment/intent"),
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

    #[doc = "List invoices for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It lists invoices for the authenticated user.\n\n```rust,no_run\nasync fn example_payments_list_invoices_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: Vec<kittycad::types::Invoice> = client.payments().list_invoices_for_user().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n```"]
    pub async fn list_invoices_for_user<'a>(
        &'a self,
    ) -> Result<Vec<crate::types::Invoice>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "user/payment/invoices"),
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

    #[doc = "List payment methods for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It lists payment methods for the authenticated user.\n\n```rust,no_run\nasync fn example_payments_list_payment_methods_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: Vec<kittycad::types::PaymentMethod> =\n        client.payments().list_payment_methods_for_user().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n```"]
    pub async fn list_payment_methods_for_user<'a>(
        &'a self,
    ) -> Result<Vec<crate::types::PaymentMethod>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "user/payment/methods"),
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

    #[doc = "Delete a payment method for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It deletes the specified payment method for the authenticated user.\n\n```rust,no_run\nasync fn example_payments_delete_payment_method_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client\n        .payments()\n        .delete_payment_method_for_user(\"qtgydofe\")\n        .await?;\n    Ok(())\n}\n\n```"]
    pub async fn delete_payment_method_for_user<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            &format!(
                "{}/{}",
                self.client.base_url,
                "user/payment/methods/{id}".replace("{id}", id)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }
}

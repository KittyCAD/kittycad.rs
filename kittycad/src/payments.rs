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

    #[doc = "Get payment info about your user.\n\nThis includes billing address, phone, and name.\nThis endpoint requires authentication by any KittyCAD user. It gets the payment information for the authenticated user.\n\n```rust,no_run\nasync fn example_payments_get_payment_information_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::Customer =\n        client.payments().get_payment_information_for_user().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
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

    #[doc = "Update payment info for your user.\n\nThis includes billing address, phone, and name.\nThis endpoint requires authentication by any KittyCAD user. It updates the payment information for the authenticated user.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_payments_update_payment_information_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::Customer = client\n        .payments()\n        .update_payment_information_for_user(&kittycad::types::BillingInfo {\n            address: Some(kittycad::types::Address {\n                city: Some(\"lxomaycwh\".to_string()),\n                country: Some(\"hmcxvd\".to_string()),\n                created_at: chrono::Utc::now(),\n                id: uuid::Uuid::from_str(\"fca56f22-4b66-4c3e-8687-133caa757887\")?,\n                state: Some(\"hyxui\".to_string()),\n                street_1: Some(\"t\".to_string()),\n                street_2: Some(\"iu\".to_string()),\n                updated_at: chrono::Utc::now(),\n                user_id: Some(\"enne\".to_string()),\n                zip: Some(\"oojdhkp\".to_string()),\n            }),\n            name: Some(\"ylidmb\".to_string()),\n            phone: Some(kittycad::types::phone_number::PhoneNumber::from_str(\n                \"+1 884-145-2152\",\n            )?),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
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

    #[doc = "Create payment info for your user.\n\nThis includes billing address, phone, and name.\nThis endpoint requires authentication by any KittyCAD user. It creates the payment information for the authenticated user.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_payments_create_payment_information_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::Customer = client\n        .payments()\n        .create_payment_information_for_user(&kittycad::types::BillingInfo {\n            address: Some(kittycad::types::Address {\n                city: Some(\"ywk\".to_string()),\n                country: Some(\"\".to_string()),\n                created_at: chrono::Utc::now(),\n                id: uuid::Uuid::from_str(\"5099d3bc-5e14-43ac-8e8f-46d8b463a547\")?,\n                state: Some(\"x\".to_string()),\n                street_1: Some(\"mtqxmqpx\".to_string()),\n                street_2: Some(\"reekqb\".to_string()),\n                updated_at: chrono::Utc::now(),\n                user_id: Some(\"jisuep\".to_string()),\n                zip: Some(\"bn\".to_string()),\n            }),\n            name: Some(\"quormvbbi\".to_string()),\n            phone: Some(kittycad::types::phone_number::PhoneNumber::from_str(\n                \"+1 0450675165\",\n            )?),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
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

    #[doc = "Delete payment info for your user.\n\nThis includes billing address, phone, and name.\nThis endpoint requires authentication by any KittyCAD user. It deletes the payment information for the authenticated user.\n\n```rust,no_run\nasync fn example_payments_delete_payment_information_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client\n        .payments()\n        .delete_payment_information_for_user()\n        .await?;\n    Ok(())\n}\n```"]
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

    #[doc = "Get balance for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It gets the balance information for the authenticated user.\n\n```rust,no_run\nasync fn example_payments_get_payment_balance_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::CustomerBalance =\n        client.payments().get_payment_balance_for_user().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
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

    #[doc = "Create a payment intent for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It creates a new payment intent for the authenticated user.\n\n```rust,no_run\nasync fn example_payments_create_payment_intent_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::PaymentIntent =\n        client.payments().create_payment_intent_for_user().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
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

    #[doc = "List invoices for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It lists invoices for the authenticated user.\n\n```rust,no_run\nasync fn example_payments_list_invoices_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: Vec<kittycad::types::Invoice> = client.payments().list_invoices_for_user().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
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

    #[doc = "List payment methods for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It lists payment methods for the authenticated user.\n\n```rust,no_run\nasync fn example_payments_list_payment_methods_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: Vec<kittycad::types::PaymentMethod> =\n        client.payments().list_payment_methods_for_user().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
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

    #[doc = "Delete a payment method for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It deletes the specified payment method for the authenticated user.\n\n```rust,no_run\nasync fn example_payments_delete_payment_method_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client\n        .payments()\n        .delete_payment_method_for_user(\"buau\")\n        .await?;\n    Ok(())\n}\n```"]
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

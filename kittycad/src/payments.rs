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

    #[doc = "Get payment info about your user.\n\nThis includes billing address, phone, and name.\nThis endpoint requires authentication by any KittyCAD user. It gets the payment information for the authenticated user.\n\n```\n/// Get payment info about your user.\n/// \n/// This includes billing address, phone, and name.\n/// This endpoint requires authentication by any KittyCAD user. It gets the payment information for the authenticated user.\nasync fn example_get_payment_information_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::Customer =\n        client.payments().get_payment_information_for_user().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n```"]
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

    #[doc = "Update payment info for your user.\n\nThis includes billing address, phone, and name.\nThis endpoint requires authentication by any KittyCAD user. It updates the payment information for the authenticated user.\n\n```\n/// Update payment info for your user.\n/// \n/// This includes billing address, phone, and name.\n/// This endpoint requires authentication by any KittyCAD user. It updates the payment information for the authenticated user.\nuse std::str::FromStr;\nasync fn example_update_payment_information_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::Customer = client\n        .payments()\n        .update_payment_information_for_user(&kittycad::types::BillingInfo {\n            address: Some(kittycad::types::Address {\n                city: Some(\"cgm\".to_string()),\n                country: Some(\"i\".to_string()),\n                created_at: chrono::DateTime::<chrono::Utc>::parse_from_rfc3339(\n                    \"2021-10-02T13:07:34.607+00:00\",\n                )?,\n                id: uuid::Uuid::from_str(\"2825ff5d-5e55-429d-98e9-b53b4be9376c\")?,\n                state: Some(\"litbf\".to_string()),\n                street1: Some(\"\".to_string()),\n                street2: Some(\"kkwj\".to_string()),\n                updated_at: chrono::DateTime::<chrono::Utc>::parse_from_rfc3339(\n                    \"1916-08-18T00:57:45.686+00:00\",\n                )?,\n                user_id: Some(\"elbifvuj\".to_string()),\n                zip: Some(\"oxshlcha\".to_string()),\n            }),\n            name: Some(\"yyyeins\".to_string()),\n            phone: kittycad::types::phone_number::PhoneNumber::from_str(\"+1 322-752-8333\")?,\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n```"]
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

    #[doc = "Create payment info for your user.\n\nThis includes billing address, phone, and name.\nThis endpoint requires authentication by any KittyCAD user. It creates the payment information for the authenticated user.\n\n```\n/// Create payment info for your user.\n/// \n/// This includes billing address, phone, and name.\n/// This endpoint requires authentication by any KittyCAD user. It creates the payment information for the authenticated user.\nuse std::str::FromStr;\nasync fn example_create_payment_information_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::Customer = client\n        .payments()\n        .create_payment_information_for_user(&kittycad::types::BillingInfo {\n            address: Some(kittycad::types::Address {\n                city: Some(\"mnv\".to_string()),\n                country: Some(\"fnotps\".to_string()),\n                created_at: chrono::DateTime::<chrono::Utc>::parse_from_rfc3339(\n                    \"1903-09-26T05:21:31.315+00:00\",\n                )?,\n                id: uuid::Uuid::from_str(\"50105945-4415-4906-bc41-9969135bb0d3\")?,\n                state: Some(\"w\".to_string()),\n                street1: Some(\"uihoqkep\".to_string()),\n                street2: Some(\"pohcx\".to_string()),\n                updated_at: chrono::DateTime::<chrono::Utc>::parse_from_rfc3339(\n                    \"2041-04-12T17:49:08.194+00:00\",\n                )?,\n                user_id: Some(\"xojfopti\".to_string()),\n                zip: Some(\"bia\".to_string()),\n            }),\n            name: Some(\"tw\".to_string()),\n            phone: kittycad::types::phone_number::PhoneNumber::from_str(\"+1 686-888-6443\")?,\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n```"]
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

    #[doc = "Delete payment info for your user.\n\nThis includes billing address, phone, and name.\nThis endpoint requires authentication by any KittyCAD user. It deletes the payment information for the authenticated user.\n\n```\n/// Delete payment info for your user.\n/// \n/// This includes billing address, phone, and name.\n/// This endpoint requires authentication by any KittyCAD user. It deletes the payment information for the authenticated user.\nasync fn example_delete_payment_information_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client\n        .payments()\n        .delete_payment_information_for_user()\n        .await?;\n    Ok(())\n}\n\n```"]
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

    #[doc = "Get balance for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It gets the balance information for the authenticated user.\n\n```\n/// Get balance for your user.\n/// \n/// This endpoint requires authentication by any KittyCAD user. It gets the balance information for the authenticated user.\nasync fn example_get_payment_balance_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::CustomerBalance =\n        client.payments().get_payment_balance_for_user().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n```"]
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

    #[doc = "Create a payment intent for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It creates a new payment intent for the authenticated user.\n\n```\n/// Create a payment intent for your user.\n/// \n/// This endpoint requires authentication by any KittyCAD user. It creates a new payment intent for the authenticated user.\nasync fn example_create_payment_intent_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::PaymentIntent =\n        client.payments().create_payment_intent_for_user().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n```"]
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

    #[doc = "List invoices for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It lists invoices for the authenticated user.\n\n```\n/// List invoices for your user.\n/// \n/// This endpoint requires authentication by any KittyCAD user. It lists invoices for the authenticated user.\nasync fn example_list_invoices_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: Vec<kittycad::types::Invoice> = client.payments().list_invoices_for_user().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n```"]
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

    #[doc = "List payment methods for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It lists payment methods for the authenticated user.\n\n```\n/// List payment methods for your user.\n/// \n/// This endpoint requires authentication by any KittyCAD user. It lists payment methods for the authenticated user.\nasync fn example_list_payment_methods_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: Vec<kittycad::types::PaymentMethod> =\n        client.payments().list_payment_methods_for_user().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n```"]
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

    #[doc = "Delete a payment method for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It deletes the specified payment method for the authenticated user.\n\n```\n/// Delete a payment method for your user.\n/// \n/// This endpoint requires authentication by any KittyCAD user. It deletes the specified payment method for the authenticated user.\nasync fn example_delete_payment_method_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client\n        .payments()\n        .delete_payment_method_for_user(\"qir\".to_string())\n        .await?;\n    Ok(())\n}\n\n```"]
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

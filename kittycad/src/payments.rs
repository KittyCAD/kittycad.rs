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

    #[doc = "Get payment info about your user.\n\nThis includes billing address, phone, and name.\nThis endpoint requires authentication by any KittyCAD user. It gets the payment information for the authenticated user.\n\n```\n/// Get payment info about your user.\n/// \n/// This includes billing address, phone, and name.\n/// This endpoint requires authentication by any KittyCAD user. It gets the payment information for the authenticated user.\nasync fn example_get_payment_information_for_user() -> anyhow::Result<()> {\n    let result: crate::types::Customer =\n        client.payments().get_payment_information_for_user().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n```"]
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
                .into()
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Update payment info for your user.\n\nThis includes billing address, phone, and name.\nThis endpoint requires authentication by any KittyCAD user. It updates the payment information for the authenticated user.\n\n```\n/// Update payment info for your user.\n/// \n/// This includes billing address, phone, and name.\n/// This endpoint requires authentication by any KittyCAD user. It updates the payment information for the authenticated user.\nasync fn example_update_payment_information_for_user() -> anyhow::Result<()> {\n    let result: crate::types::Customer = client\n        .payments()\n        .update_payment_information_for_user(&crate::types::BillingInfo {\n            address: crate::types::Address {\n                city: \"\".to_string(),\n                country: \"rnvutdlc\".to_string(),\n                created_at: chrono::DateTime::<chrono::Utc>::parse_from_rfc3339(\n                    \"2007-10-20T00:19:35.234+00:00\",\n                )?,\n                id: uuid::Uuid::from_str(\"6e1431c1-a186-4b67-a756-b921ad1e02d3\")?,\n                state: \"nl\".to_string(),\n                street1: \"kle\".to_string(),\n                street2: \"bvysojk\".to_string(),\n                updated_at: chrono::DateTime::<chrono::Utc>::parse_from_rfc3339(\n                    \"2030-04-11T05:49:07.831+00:00\",\n                )?,\n                user_id: \"gubgp\".to_string(),\n                zip: \"meqnob\".to_string(),\n            },\n            name: \"sggq\".to_string(),\n            phone: crate::types::phone_number::PhoneNumber::from_str(\"+1 450-431-0164\")?,\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n```"]
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
                .into()
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Create payment info for your user.\n\nThis includes billing address, phone, and name.\nThis endpoint requires authentication by any KittyCAD user. It creates the payment information for the authenticated user.\n\n```\n/// Create payment info for your user.\n/// \n/// This includes billing address, phone, and name.\n/// This endpoint requires authentication by any KittyCAD user. It creates the payment information for the authenticated user.\nasync fn example_create_payment_information_for_user() -> anyhow::Result<()> {\n    let result: crate::types::Customer = client\n        .payments()\n        .create_payment_information_for_user(&crate::types::BillingInfo {\n            address: crate::types::Address {\n                city: \"wo\".to_string(),\n                country: \"\".to_string(),\n                created_at: chrono::DateTime::<chrono::Utc>::parse_from_rfc3339(\n                    \"1911-06-03T04:18:46.704+00:00\",\n                )?,\n                id: uuid::Uuid::from_str(\"11101075-d2a7-4347-b985-689a0f058313\")?,\n                state: \"r\".to_string(),\n                street1: \"vdyver\".to_string(),\n                street2: \"gmdttqkh\".to_string(),\n                updated_at: chrono::DateTime::<chrono::Utc>::parse_from_rfc3339(\n                    \"2098-12-13T10:05:26.498+00:00\",\n                )?,\n                user_id: \"gmgqtkno\".to_string(),\n                zip: \"lt\".to_string(),\n            },\n            name: \"r\".to_string(),\n            phone: crate::types::phone_number::PhoneNumber::from_str(\"+1 778-325-5131\")?,\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n```"]
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
                .into()
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Delete payment info for your user.\n\nThis includes billing address, phone, and name.\nThis endpoint requires authentication by any KittyCAD user. It deletes the payment information for the authenticated user.\n\n```\n/// Delete payment info for your user.\n/// \n/// This includes billing address, phone, and name.\n/// This endpoint requires authentication by any KittyCAD user. It deletes the payment information for the authenticated user.\nasync fn example_delete_payment_information_for_user() -> anyhow::Result<()> {\n    client\n        .payments()\n        .delete_payment_information_for_user()\n        .await?;\n    Ok(())\n}\n\n```"]
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

    #[doc = "Get balance for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It gets the balance information for the authenticated user.\n\n```\n/// Get balance for your user.\n/// \n/// This endpoint requires authentication by any KittyCAD user. It gets the balance information for the authenticated user.\nasync fn example_get_payment_balance_for_user() -> anyhow::Result<()> {\n    let result: crate::types::CustomerBalance =\n        client.payments().get_payment_balance_for_user().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n```"]
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
                .into()
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Create a payment intent for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It creates a new payment intent for the authenticated user.\n\n```\n/// Create a payment intent for your user.\n/// \n/// This endpoint requires authentication by any KittyCAD user. It creates a new payment intent for the authenticated user.\nasync fn example_create_payment_intent_for_user() -> anyhow::Result<()> {\n    let result: crate::types::PaymentIntent =\n        client.payments().create_payment_intent_for_user().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n```"]
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
                .into()
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "List invoices for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It lists invoices for the authenticated user.\n\n```\n/// List invoices for your user.\n/// \n/// This endpoint requires authentication by any KittyCAD user. It lists invoices for the authenticated user.\nasync fn example_list_invoices_for_user() -> anyhow::Result<()> {\n    let result: Vec<crate::types::Invoice> = client.payments().list_invoices_for_user().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n```"]
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
                .into()
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "List payment methods for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It lists payment methods for the authenticated user.\n\n```\n/// List payment methods for your user.\n/// \n/// This endpoint requires authentication by any KittyCAD user. It lists payment methods for the authenticated user.\nasync fn example_list_payment_methods_for_user() -> anyhow::Result<()> {\n    let result: Vec<crate::types::PaymentMethod> =\n        client.payments().list_payment_methods_for_user().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n```"]
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
                .into()
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Delete a payment method for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It deletes the specified payment method for the authenticated user.\n\n```\n/// Delete a payment method for your user.\n/// \n/// This endpoint requires authentication by any KittyCAD user. It deletes the specified payment method for the authenticated user.\nasync fn example_delete_payment_method_for_user() -> anyhow::Result<()> {\n    client\n        .payments()\n        .delete_payment_method_for_user(\"uyilweb\".to_string())\n        .await?;\n    Ok(())\n}\n\n```"]
    pub async fn delete_payment_method_for_user<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            &format!(
                "{}/{}",
                self.client.base_url,
                "user/payment/methods/{id}".replace("{id}", &id)
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

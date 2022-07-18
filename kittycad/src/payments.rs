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

    #[doc = "Get payment info about your user.\n\nThis includes billing address, phone, and name.\nThis endpoint requires authentication by any KittyCAD user. It gets the payment information for the authenticated user."]
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

    #[doc = "Update payment info for your user.\n\nThis includes billing address, phone, and name.\nThis endpoint requires authentication by any KittyCAD user. It updates the payment information for the authenticated user."]
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

    #[doc = "Create payment info for your user.\n\nThis includes billing address, phone, and name.\nThis endpoint requires authentication by any KittyCAD user. It creates the payment information for the authenticated user."]
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

    #[doc = "Delete payment info for your user.\n\nThis includes billing address, phone, and name.\nThis endpoint requires authentication by any KittyCAD user. It deletes the payment information for the authenticated user."]
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
            let _text = resp.text().await.unwrap_or_default();
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Get balance for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It gets the balance information for the authenticated user."]
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

    #[doc = "Create a payment intent for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It creates a new payment intent for the authenticated user."]
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

    #[doc = "List invoices for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It lists invoices for the authenticated user."]
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

    #[doc = "List payment methods for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It lists payment methods for the authenticated user."]
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

    #[doc = "Delete a payment method for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It deletes the specified payment method for the authenticated user."]
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
            let _text = resp.text().await.unwrap_or_default();
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }
}

use anyhow::Result;

use crate::Client;

pub struct Payments {
    pub client: Client,
}

impl Payments {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Payments { client }
    }

    #[doc = "Get payment info about your user.\n\nThis includes billing address, phone, and name.\nThis endpoint requires authentication by any KittyCAD user. It gets the payment information for the authenticated user."]
    pub async fn get_payment_information_for_user(&self) -> Result<crate::types::Customer> {
        let mut rb = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "user/payment"),
        );
        rb = rb.bearer_auth(self.client.token);
        let req = rb.build()?;
        let resp = self.client.client.execute(req).await?;
        resp.json()?
    }

    #[doc = "Update payment info for your user.\n\nThis includes billing address, phone, and name.\nThis endpoint requires authentication by any KittyCAD user. It updates the payment information for the authenticated user."]
    pub async fn update_payment_information_for_user(
        &self,
        body: &crate::types::BillingInfo,
    ) -> Result<crate::types::Customer> {
        let mut rb = self.client.client.request(
            http::Method::PUT,
            &format!("{}/{}", self.client.base_url, "user/payment"),
        );
        rb = rb.bearer_auth(self.client.token);
        rb = rb.json(body);
        let req = rb.build()?;
        let resp = self.client.client.execute(req).await?;
        resp.json()?
    }

    #[doc = "Create payment info for your user.\n\nThis includes billing address, phone, and name.\nThis endpoint requires authentication by any KittyCAD user. It creates the payment information for the authenticated user."]
    pub async fn create_payment_information_for_user(
        &self,
        body: &crate::types::BillingInfo,
    ) -> Result<crate::types::Customer> {
        let mut rb = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "user/payment"),
        );
        rb = rb.bearer_auth(self.client.token);
        rb = rb.json(body);
        let req = rb.build()?;
        let resp = self.client.client.execute(req).await?;
        resp.json()?
    }

    #[doc = "Delete payment info for your user.\n\nThis includes billing address, phone, and name.\nThis endpoint requires authentication by any KittyCAD user. It deletes the payment information for the authenticated user."]
    pub async fn delete_payment_information_for_user(&self) -> Result<()> {
        let mut rb = self.client.client.request(
            http::Method::DELETE,
            &format!("{}/{}", self.client.base_url, "user/payment"),
        );
        rb = rb.bearer_auth(self.client.token);
        let req = rb.build()?;
        let resp = self.client.client.execute(req).await?;
        resp.json()?
    }

    #[doc = "Create a payment intent for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It creates a new payment intent for the authenticated user."]
    pub async fn create_payment_intent_for_user(&self) -> Result<crate::types::PaymentIntent> {
        let mut rb = self.client.client.request(
            http::Method::POST,
            &format!("{}/{}", self.client.base_url, "user/payment/intent"),
        );
        rb = rb.bearer_auth(self.client.token);
        let req = rb.build()?;
        let resp = self.client.client.execute(req).await?;
        resp.json()?
    }

    #[doc = "List invoices for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It lists invoices for the authenticated user."]
    pub async fn list_invoices_for_user(&self) -> Result<Vec<crate::types::Invoice>> {
        let mut rb = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "user/payment/invoices"),
        );
        rb = rb.bearer_auth(self.client.token);
        let req = rb.build()?;
        let resp = self.client.client.execute(req).await?;
        resp.json()?
    }

    #[doc = "List payment methods for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It lists payment methods for the authenticated user."]
    pub async fn list_payment_methods_for_user(&self) -> Result<Vec<crate::types::PaymentMethod>> {
        let mut rb = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "user/payment/methods"),
        );
        rb = rb.bearer_auth(self.client.token);
        let req = rb.build()?;
        let resp = self.client.client.execute(req).await?;
        resp.json()?
    }

    #[doc = "Delete a payment method for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It deletes the specified payment method for the authenticated user."]
    pub async fn delete_payment_method_for_user(&self, id: String) -> Result<()> {
        let mut rb = self.client.client.request(
            http::Method::DELETE,
            &format!("{}/{}", self.client.base_url, "user/payment/methods/{id}"),
        );
        rb = rb.bearer_auth(self.client.token);
        let req = rb.build()?;
        let resp = self.client.client.execute(req).await?;
        resp.json()?
    }
}

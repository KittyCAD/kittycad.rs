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

    /**
     * Get payment info about your user.
     *
     * This function performs a `GET` to the `/user/payment` endpoint.
     *
     * This includes billing address, phone, and name.
     * This endpoint requires authentication by any KittyCAD user. It gets the payment information for the authenticated user.
     */
    pub async fn get_information_for_user(&self) -> Result<crate::types::Customer> {
        let url = "/user/payment".to_string();
        self.client.get(&url, None).await
    }

    /**
     * Update payment info for your user.
     *
     * This function performs a `PUT` to the `/user/payment` endpoint.
     *
     * This includes billing address, phone, and name.
     * This endpoint requires authentication by any KittyCAD user. It updates the payment information for the authenticated user.
     */
    pub async fn update_information_for_user(
        &self,
        body: &crate::types::BillingInfo,
    ) -> Result<crate::types::Customer> {
        let url = "/user/payment".to_string();
        self.client
            .put(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Create payment info for your user.
     *
     * This function performs a `POST` to the `/user/payment` endpoint.
     *
     * This includes billing address, phone, and name.
     * This endpoint requires authentication by any KittyCAD user. It creates the payment information for the authenticated user.
     */
    pub async fn create_information_for_user(
        &self,
        body: &crate::types::BillingInfo,
    ) -> Result<crate::types::Customer> {
        let url = "/user/payment".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }

    /**
     * Delete payment info for your user.
     *
     * This function performs a `DELETE` to the `/user/payment` endpoint.
     *
     * This includes billing address, phone, and name.
     * This endpoint requires authentication by any KittyCAD user. It deletes the payment information for the authenticated user.
     */
    pub async fn delete_information_for_user(&self) -> Result<()> {
        let url = "/user/payment".to_string();
        self.client.delete(&url, None).await
    }

    /**
     * Create a payment intent for your user.
     *
     * This function performs a `POST` to the `/user/payment/intent` endpoint.
     *
     * This endpoint requires authentication by any KittyCAD user. It creates a new payment intent for the authenticated user.
     */
    pub async fn create_intent_for_user(&self) -> Result<crate::types::PaymentIntent> {
        let url = "/user/payment/intent".to_string();
        self.client.post(&url, None).await
    }

    /**
     * List invoices for your user.
     *
     * This function performs a `GET` to the `/user/payment/invoices` endpoint.
     *
     * This endpoint requires authentication by any KittyCAD user. It lists invoices for the authenticated user.
     */
    pub async fn list_invoices_for_user(&self) -> Result<Vec<crate::types::Invoice>> {
        let url = "/user/payment/invoices".to_string();
        self.client.get(&url, None).await
    }

    /**
     * List payment methods for your user.
     *
     * This function performs a `GET` to the `/user/payment/methods` endpoint.
     *
     * This endpoint requires authentication by any KittyCAD user. It lists payment methods for the authenticated user.
     */
    pub async fn list_methods_for_user(&self) -> Result<Vec<crate::types::PaymentMethod>> {
        let url = "/user/payment/methods".to_string();
        self.client.get(&url, None).await
    }

    /**
     * Delete a payment method for your user.
     *
     * This function performs a `DELETE` to the `/user/payment/methods/{id}` endpoint.
     *
     * This endpoint requires authentication by any KittyCAD user. It deletes the specified payment method for the authenticated user.
     *
     * **Parameters:**
     *
     * * `id: &str` -- The ID of the payment method.
     */
    pub async fn delete_method_for_user(&self, id: &str) -> Result<()> {
        let url = format!(
            "/user/payment/methods/{}",
            crate::progenitor_support::encode_path(&id.to_string()),
        );

        self.client.delete(&url, None).await
    }
}

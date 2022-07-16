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
        todo!()
    }

    #[doc = "Update payment info for your user.\n\nThis includes billing address, phone, and name.\nThis endpoint requires authentication by any KittyCAD user. It updates the payment information for the authenticated user."]
    pub async fn update_payment_information_for_user(
        &self,
        _body: &crate::types::BillingInfo,
    ) -> Result<crate::types::Customer> {
        todo!()
    }

    #[doc = "Create payment info for your user.\n\nThis includes billing address, phone, and name.\nThis endpoint requires authentication by any KittyCAD user. It creates the payment information for the authenticated user."]
    pub async fn create_payment_information_for_user(
        &self,
        _body: &crate::types::BillingInfo,
    ) -> Result<crate::types::Customer> {
        todo!()
    }

    #[doc = "Delete payment info for your user.\n\nThis includes billing address, phone, and name.\nThis endpoint requires authentication by any KittyCAD user. It deletes the payment information for the authenticated user."]
    pub async fn delete_payment_information_for_user(&self) -> Result<()> {
        todo!()
    }

    #[doc = "Create a payment intent for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It creates a new payment intent for the authenticated user."]
    pub async fn create_payment_intent_for_user(&self) -> Result<crate::types::PaymentIntent> {
        todo!()
    }

    #[doc = "List invoices for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It lists invoices for the authenticated user."]
    pub async fn list_invoices_for_user(&self) -> Result<Vec<crate::types::Invoice>> {
        todo!()
    }

    #[doc = "List payment methods for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It lists payment methods for the authenticated user."]
    pub async fn list_payment_methods_for_user(&self) -> Result<Vec<crate::types::PaymentMethod>> {
        todo!()
    }

    #[doc = "Delete a payment method for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It deletes the specified payment method for the authenticated user."]
    pub async fn delete_payment_method_for_user(&self, _id: String) -> Result<()> {
        todo!()
    }
}

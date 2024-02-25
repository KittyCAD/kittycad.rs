use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Payments {
    pub client: Client,
}

impl Payments {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get payment info about your org.\n\nThis includes billing address, phone, and \
             name.\nThis endpoint requires authentication by an org admin. It gets the payment \
             information for the authenticated user's org.\n\n```rust,no_run\nasync fn \
             example_payments_get_information_for_org() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: kittycad::types::Customer = \
             client.payments().get_information_for_org().await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_information_for_org<'a>(
        &'a self,
    ) -> Result<crate::types::Customer, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "org/payment"),
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
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Update payment info for your org.\n\nThis includes billing address, phone, and \
             name.\nThis endpoint requires authentication by an org admin. It updates the payment \
             information for the authenticated user's org.\n\n```rust,no_run\nuse \
             std::str::FromStr;\nasync fn example_payments_update_information_for_org() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::Customer = client\n        .payments()\n        \
             .update_information_for_org(&kittycad::types::BillingInfo {\n            address: \
             Some(kittycad::types::AddressDetails {\n                city: \
             Some(\"some-string\".to_string()),\n                country: \
             \"some-string\".to_string(),\n                state: \
             Some(\"some-string\".to_string()),\n                street_1: \
             Some(\"some-string\".to_string()),\n                street_2: \
             Some(\"some-string\".to_string()),\n                zip: \
             Some(\"some-string\".to_string()),\n            }),\n            name: \
             Some(\"some-string\".to_string()),\n            phone: \
             kittycad::types::phone_number::PhoneNumber::from_str(\"+1555-555-5555\")?,\n        \
             })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_information_for_org<'a>(
        &'a self,
        body: &crate::types::BillingInfo,
    ) -> Result<crate::types::Customer, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!("{}/{}", self.client.base_url, "org/payment"),
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
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Create payment info for your org.\n\nThis includes billing address, phone, and \
             name.\nThis endpoint requires authentication by the org admin. It creates the payment \
             information for the authenticated user's org.\n\n```rust,no_run\nuse \
             std::str::FromStr;\nasync fn example_payments_create_information_for_org() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::Customer = client\n        .payments()\n        \
             .create_information_for_org(&kittycad::types::BillingInfo {\n            address: \
             Some(kittycad::types::AddressDetails {\n                city: \
             Some(\"some-string\".to_string()),\n                country: \
             \"some-string\".to_string(),\n                state: \
             Some(\"some-string\".to_string()),\n                street_1: \
             Some(\"some-string\".to_string()),\n                street_2: \
             Some(\"some-string\".to_string()),\n                zip: \
             Some(\"some-string\".to_string()),\n            }),\n            name: \
             Some(\"some-string\".to_string()),\n            phone: \
             kittycad::types::phone_number::PhoneNumber::from_str(\"+1555-555-5555\")?,\n        \
             })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_information_for_org<'a>(
        &'a self,
        body: &crate::types::BillingInfo,
    ) -> Result<crate::types::Customer, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "org/payment"),
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
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Delete payment info for your org.\n\nThis includes billing address, phone, and \
             name.\nThis endpoint requires authentication by an org admin. It deletes the payment \
             information for the authenticated user's org.\n\n```rust,no_run\nasync fn \
             example_payments_delete_information_for_org() -> anyhow::Result<()> {\n    let client \
             = kittycad::Client::new_from_env();\n    \
             client.payments().delete_information_for_org().await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_information_for_org<'a>(
        &'a self,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!("{}/{}", self.client.base_url, "org/payment"),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Get balance for your org.\n\nThis endpoint requires authentication by an org admin. \
             It gets the balance information for the authenticated user's \
             org.\n\n```rust,no_run\nasync fn example_payments_get_balance_for_org() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::CustomerBalance = \
             client.payments().get_balance_for_org().await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_balance_for_org<'a>(
        &'a self,
    ) -> Result<crate::types::CustomerBalance, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "org/payment/balance"),
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
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Create a payment intent for your org.\n\nThis endpoint requires authentication by the org admin. It creates a new payment intent for the authenticated user's org's org.\n\n```rust,no_run\nasync fn example_payments_create_intent_for_org() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::PaymentIntent = client.payments().create_intent_for_org().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_intent_for_org<'a>(
        &'a self,
    ) -> Result<crate::types::PaymentIntent, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "org/payment/intent"),
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
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "List invoices for your org.\n\nThis endpoint requires authentication by an org admin. It lists invoices for the authenticated user's org.\n\n```rust,no_run\nasync fn example_payments_list_invoices_for_org() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: Vec<kittycad::types::Invoice> = client.payments().list_invoices_for_org().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_invoices_for_org<'a>(
        &'a self,
    ) -> Result<Vec<crate::types::Invoice>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "org/payment/invoices"),
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
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "List payment methods for your org.\n\nThis endpoint requires authentication by an org admin. It lists payment methods for the authenticated user's org.\n\n```rust,no_run\nasync fn example_payments_list_methods_for_org() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: Vec<kittycad::types::PaymentMethod> = client.payments().list_methods_for_org().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_methods_for_org<'a>(
        &'a self,
    ) -> Result<Vec<crate::types::PaymentMethod>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "org/payment/methods"),
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
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Delete a payment method for your org.\n\nThis endpoint requires authentication by an \
             org admin. It deletes the specified payment method for the authenticated user's \
             org.\n\n**Parameters:**\n\n- `id: &'astr`: The ID of the payment method. \
             (required)\n\n```rust,no_run\nasync fn example_payments_delete_method_for_org() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    \
             client\n        .payments()\n        .delete_method_for_org(\"some-string\")\n        \
             .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_method_for_org<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "org/payment/methods/{id}".replace("{id}", id)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Get the subscription for an org.\n\nThis endpoint requires authentication by an org admin. It gets the subscription for the authenticated user's org.\n\n```rust,no_run\nasync fn example_payments_get_org_subscription() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ZooProductSubscriptions =\n        client.payments().get_org_subscription().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_org_subscription<'a>(
        &'a self,
    ) -> Result<crate::types::ZooProductSubscriptions, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "org/payment/subscriptions"),
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
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Update the subscription for an org.\n\nThis endpoint requires authentication by an org admin. It updates the subscription for the authenticated user's org.\n\n```rust,no_run\nasync fn example_payments_update_org_subscription() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ZooProductSubscriptions = client\n        .payments()\n        .update_org_subscription(&kittycad::types::ZooProductSubscriptionsOrgRequest {\n            modeling_app: Some(kittycad::types::ModelingAppOrganizationSubscriptionTier::Enterprise),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_org_subscription<'a>(
        &'a self,
        body: &crate::types::ZooProductSubscriptionsOrgRequest,
    ) -> Result<crate::types::ZooProductSubscriptions, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!("{}/{}", self.client.base_url, "org/payment/subscriptions"),
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
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Create the subscription for an org.\n\nThis endpoint requires authentication by an org admin. It creates the subscription for the authenticated user's org.\n\n```rust,no_run\nasync fn example_payments_create_org_subscription() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ZooProductSubscriptions = client\n        .payments()\n        .create_org_subscription(&kittycad::types::ZooProductSubscriptionsOrgRequest {\n            modeling_app: Some(kittycad::types::ModelingAppOrganizationSubscriptionTier::Enterprise),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_org_subscription<'a>(
        &'a self,
        body: &crate::types::ZooProductSubscriptionsOrgRequest,
    ) -> Result<crate::types::ZooProductSubscriptions, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "org/payment/subscriptions"),
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
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Validate an orgs's information is correct and valid for automatic tax.\n\nThis endpoint requires authentication by an org admin. It will return an error if the org's information is not valid for automatic tax. Otherwise, it will return an empty successful response.\n\n```rust,no_run\nasync fn example_payments_validate_customer_tax_information_for_org() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client\n        .payments()\n        .validate_customer_tax_information_for_org()\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn validate_customer_tax_information_for_org<'a>(
        &'a self,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "org/payment/tax"),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Get balance for an org.\n\nThis endpoint requires authentication by a Zoo employee. \
             It gets the balance information for the specified org.\n\n**Parameters:**\n\n- `id: \
             uuid::Uuid`: The organization ID. (required)\n\n```rust,no_run\nuse \
             std::str::FromStr;\nasync fn example_payments_get_balance_for_any_org() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::CustomerBalance = client\n        .payments()\n        \
             .get_balance_for_any_org(uuid::Uuid::from_str(\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_balance_for_any_org<'a>(
        &'a self,
        id: uuid::Uuid,
    ) -> Result<crate::types::CustomerBalance, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "orgs/{id}/payment/balance".replace("{id}", &format!("{}", id))
            ),
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
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Update balance for an org.\n\nThis endpoint requires authentication by a Zoo employee. It updates the balance information for the specified org.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The organization ID. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_payments_update_balance_for_any_org() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::CustomerBalance = client\n        .payments()\n        .update_balance_for_any_org(\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            &kittycad::types::UpdatePaymentBalance {\n                monthly_credits_remaining: Some(3.14 as f64),\n                pre_pay_cash_remaining: Some(3.14 as f64),\n                pre_pay_credits_remaining: Some(3.14 as f64),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_balance_for_any_org<'a>(
        &'a self,
        id: uuid::Uuid,
        body: &crate::types::UpdatePaymentBalance,
    ) -> Result<crate::types::CustomerBalance, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "orgs/{id}/payment/balance".replace("{id}", &format!("{}", id))
            ),
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
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Get payment info about your user.\n\nThis includes billing address, phone, and \
             name.\nThis endpoint requires authentication by any Zoo user. It gets the payment \
             information for the authenticated user.\n\n```rust,no_run\nasync fn \
             example_payments_get_information_for_user() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: kittycad::types::Customer = \
             client.payments().get_information_for_user().await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_information_for_user<'a>(
        &'a self,
    ) -> Result<crate::types::Customer, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "user/payment"),
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
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Update payment info for your user.\n\nThis includes billing address, phone, and \
             name.\nThis endpoint requires authentication by any Zoo user. It updates the payment \
             information for the authenticated user.\n\n```rust,no_run\nuse \
             std::str::FromStr;\nasync fn example_payments_update_information_for_user() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::Customer = client\n        .payments()\n        \
             .update_information_for_user(&kittycad::types::BillingInfo {\n            address: \
             Some(kittycad::types::AddressDetails {\n                city: \
             Some(\"some-string\".to_string()),\n                country: \
             \"some-string\".to_string(),\n                state: \
             Some(\"some-string\".to_string()),\n                street_1: \
             Some(\"some-string\".to_string()),\n                street_2: \
             Some(\"some-string\".to_string()),\n                zip: \
             Some(\"some-string\".to_string()),\n            }),\n            name: \
             Some(\"some-string\".to_string()),\n            phone: \
             kittycad::types::phone_number::PhoneNumber::from_str(\"+1555-555-5555\")?,\n        \
             })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_information_for_user<'a>(
        &'a self,
        body: &crate::types::BillingInfo,
    ) -> Result<crate::types::Customer, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!("{}/{}", self.client.base_url, "user/payment"),
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
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Create payment info for your user.\n\nThis includes billing address, phone, and \
             name.\nThis endpoint requires authentication by any Zoo user. It creates the payment \
             information for the authenticated user.\n\n```rust,no_run\nuse \
             std::str::FromStr;\nasync fn example_payments_create_information_for_user() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::Customer = client\n        .payments()\n        \
             .create_information_for_user(&kittycad::types::BillingInfo {\n            address: \
             Some(kittycad::types::AddressDetails {\n                city: \
             Some(\"some-string\".to_string()),\n                country: \
             \"some-string\".to_string(),\n                state: \
             Some(\"some-string\".to_string()),\n                street_1: \
             Some(\"some-string\".to_string()),\n                street_2: \
             Some(\"some-string\".to_string()),\n                zip: \
             Some(\"some-string\".to_string()),\n            }),\n            name: \
             Some(\"some-string\".to_string()),\n            phone: \
             kittycad::types::phone_number::PhoneNumber::from_str(\"+1555-555-5555\")?,\n        \
             })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_information_for_user<'a>(
        &'a self,
        body: &crate::types::BillingInfo,
    ) -> Result<crate::types::Customer, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "user/payment"),
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
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Delete payment info for your user.\n\nThis includes billing address, phone, and \
             name.\nThis endpoint requires authentication by any Zoo user. It deletes the payment \
             information for the authenticated user.\n\n```rust,no_run\nasync fn \
             example_payments_delete_information_for_user() -> anyhow::Result<()> {\n    let \
             client = kittycad::Client::new_from_env();\n    \
             client.payments().delete_information_for_user().await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_information_for_user<'a>(
        &'a self,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!("{}/{}", self.client.base_url, "user/payment"),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Get balance for your user.\n\nThis endpoint requires authentication by any Zoo user. It gets the balance information for the authenticated user.\n\n```rust,no_run\nasync fn example_payments_get_balance_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::CustomerBalance = client.payments().get_balance_for_user().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_balance_for_user<'a>(
        &'a self,
    ) -> Result<crate::types::CustomerBalance, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "user/payment/balance"),
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
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Create a payment intent for your user.\n\nThis endpoint requires authentication by any Zoo user. It creates a new payment intent for the authenticated user.\n\n```rust,no_run\nasync fn example_payments_create_intent_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::PaymentIntent = client.payments().create_intent_for_user().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_intent_for_user<'a>(
        &'a self,
    ) -> Result<crate::types::PaymentIntent, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "user/payment/intent"),
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
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "List invoices for your user.\n\nThis endpoint requires authentication by any Zoo user. It lists invoices for the authenticated user.\n\n```rust,no_run\nasync fn example_payments_list_invoices_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: Vec<kittycad::types::Invoice> = client.payments().list_invoices_for_user().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_invoices_for_user<'a>(
        &'a self,
    ) -> Result<Vec<crate::types::Invoice>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "user/payment/invoices"),
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
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "List payment methods for your user.\n\nThis endpoint requires authentication by any Zoo user. It lists payment methods for the authenticated user.\n\n```rust,no_run\nasync fn example_payments_list_methods_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: Vec<kittycad::types::PaymentMethod> =\n        client.payments().list_methods_for_user().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_methods_for_user<'a>(
        &'a self,
    ) -> Result<Vec<crate::types::PaymentMethod>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "user/payment/methods"),
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
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Delete a payment method for your user.\n\nThis endpoint requires authentication by any Zoo user. It deletes the specified payment method for the authenticated user.\n\n**Parameters:**\n\n- `id: &'astr`: The ID of the payment method. (required)\n\n```rust,no_run\nasync fn example_payments_delete_method_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client\n        .payments()\n        .delete_method_for_user(\"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_method_for_user<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
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
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Get the subscription for a user.\n\nThis endpoint requires authentication by any Zoo user. It gets the subscription for the user.\n\n```rust,no_run\nasync fn example_payments_get_user_subscription() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ZooProductSubscriptions =\n        client.payments().get_user_subscription().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_user_subscription<'a>(
        &'a self,
    ) -> Result<crate::types::ZooProductSubscriptions, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "user/payment/subscriptions"),
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
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Update the user's subscription.\n\nThis endpoint requires authentication by any Zoo user. It updates the subscription for the user.\n\n```rust,no_run\nasync fn example_payments_update_user_subscription() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ZooProductSubscriptions = client\n        .payments()\n        .update_user_subscription(&kittycad::types::ZooProductSubscriptionsUserRequest {\n            modeling_app: Some(kittycad::types::ModelingAppIndividualSubscriptionTier::Pro),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_user_subscription<'a>(
        &'a self,
        body: &crate::types::ZooProductSubscriptionsUserRequest,
    ) -> Result<crate::types::ZooProductSubscriptions, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!("{}/{}", self.client.base_url, "user/payment/subscriptions"),
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
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Create the subscription for a user.\n\nThis endpoint requires authentication by any Zoo user. It creates the subscription for the user.\n\n```rust,no_run\nasync fn example_payments_create_user_subscription() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ZooProductSubscriptions = client\n        .payments()\n        .create_user_subscription(&kittycad::types::ZooProductSubscriptionsUserRequest {\n            modeling_app: Some(kittycad::types::ModelingAppIndividualSubscriptionTier::Pro),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_user_subscription<'a>(
        &'a self,
        body: &crate::types::ZooProductSubscriptionsUserRequest,
    ) -> Result<crate::types::ZooProductSubscriptions, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "user/payment/subscriptions"),
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
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Validate a user's information is correct and valid for automatic tax.\n\nThis \
             endpoint requires authentication by any Zoo user. It will return an error if the \
             user's information is not valid for automatic tax. Otherwise, it will return an empty \
             successful response.\n\n```rust,no_run\nasync fn \
             example_payments_validate_customer_tax_information_for_user() -> anyhow::Result<()> \
             {\n    let client = kittycad::Client::new_from_env();\n    client\n        \
             .payments()\n        .validate_customer_tax_information_for_user()\n        \
             .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn validate_customer_tax_information_for_user<'a>(
        &'a self,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "user/payment/tax"),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Get balance for an user.\n\nThis endpoint requires authentication by a Zoo employee. \
             It gets the balance information for the specified user.\n\n**Parameters:**\n\n- `id: \
             uuid::Uuid`: The user ID. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync \
             fn example_payments_get_balance_for_any_user() -> anyhow::Result<()> {\n    let \
             client = kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::CustomerBalance = client\n        .payments()\n        \
             .get_balance_for_any_user(uuid::Uuid::from_str(\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_balance_for_any_user<'a>(
        &'a self,
        id: uuid::Uuid,
    ) -> Result<crate::types::CustomerBalance, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "users/{id}/payment/balance".replace("{id}", &format!("{}", id))
            ),
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
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "Update balance for an user.\n\nThis endpoint requires authentication by a Zoo employee. It updates the balance information for the specified user.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The user ID. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_payments_update_balance_for_any_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::CustomerBalance = client\n        .payments()\n        .update_balance_for_any_user(\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            &kittycad::types::UpdatePaymentBalance {\n                monthly_credits_remaining: Some(3.14 as f64),\n                pre_pay_cash_remaining: Some(3.14 as f64),\n                pre_pay_credits_remaining: Some(3.14 as f64),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_balance_for_any_user<'a>(
        &'a self,
        id: uuid::Uuid,
        body: &crate::types::UpdatePaymentBalance,
    ) -> Result<crate::types::CustomerBalance, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "users/{id}/payment/balance".replace("{id}", &format!("{}", id))
            ),
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
            let text = resp.text().await.unwrap_or_default();
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }
}

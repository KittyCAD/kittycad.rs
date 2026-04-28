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
             name.\n\nThis endpoint requires authentication by an org admin. It gets the payment \
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Update payment info for your org.\n\nThis includes billing address, phone, and \
             name.\n\nThis endpoint requires authentication by an org admin. It updates the \
             payment information for the authenticated user's org.\n\n```rust,no_run\nuse \
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Create payment info for your org.\n\nThis includes billing address, phone, and \
             name.\n\nThis endpoint requires authentication by the org admin. It creates the \
             payment information for the authenticated user's org.\n\n```rust,no_run\nuse \
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Delete payment info for your org.\n\nThis includes billing address, phone, and \
             name.\n\nThis endpoint requires authentication by an org admin. It deletes the \
             payment information for the authenticated user's org.\n\n```rust,no_run\nasync fn \
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Get balance for your org.\n\nThis endpoint requires authentication by any member of \
             an org. It gets the balance information for the authenticated user's \
             org.\n\n**Parameters:**\n\n- `include_total_due: Option<bool>`: If you would like to \
             return the total due for a user. This makes the API call take longer so it is off by \
             default.\n\n```rust,no_run\nasync fn example_payments_get_balance_for_org() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::CustomerBalance =\n        \
             client.payments().get_balance_for_org(Some(true)).await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_balance_for_org<'a>(
        &'a self,
        include_total_due: Option<bool>,
    ) -> Result<crate::types::CustomerBalance, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "org/payment/balance"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = include_total_due {
            query_params.push(("include_total_due", format!("{}", p)));
        }

        req = req.query(&query_params);
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "List invoices for your org.\n\nThis endpoint requires authentication by an org admin. It lists invoices for the authenticated user's org.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_payments_list_invoices_for_org_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut payments = client.payments();\n    let mut stream = payments.list_invoices_for_org_stream(Some(4 as u32));\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_invoices_for_org<'a>(
        &'a self,
        limit: Option<u32>,
        page_token: Option<String>,
    ) -> Result<crate::types::InvoiceResultsPage, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "org/payment/invoices"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = limit {
            query_params.push(("limit", format!("{}", p)));
        }

        if let Some(p) = page_token {
            query_params.push(("page_token", p));
        }

        req = req.query(&query_params);
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

    #[doc = "List invoices for your org.\n\nThis endpoint requires authentication by an org admin. It lists invoices for the authenticated user's org.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_payments_list_invoices_for_org_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut payments = client.payments();\n    let mut stream = payments.list_invoices_for_org_stream(Some(4 as u32));\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_invoices_for_org_stream<'a>(
        &'a self,
        limit: Option<u32>,
    ) -> impl futures::Stream<Item = Result<crate::types::Invoice, crate::types::error::Error>>
           + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        self.list_invoices_for_org(limit, None)
            .map_ok(move |result| {
                let items = futures::stream::iter(result.items().into_iter().map(Ok));
                let next_pages = futures::stream::try_unfold(
                    (None, result),
                    move |(prev_page_token, new_result)| async move {
                        if new_result.has_more_pages()
                            && !new_result.items().is_empty()
                            && prev_page_token != new_result.next_page_token()
                        {
                            async {
                                let mut req = self.client.client.request(
                                    http::Method::GET,
                                    format!("{}/{}", self.client.base_url, "org/payment/invoices"),
                                );
                                req = req.bearer_auth(&self.client.token);
                                let mut request = req.build()?;
                                request = new_result.next_page(request)?;
                                let resp = self.client.client.execute(request).await?;
                                let status = resp.status();
                                if status.is_success() {
                                    let text = resp.text().await.unwrap_or_default();
                                    serde_json::from_str(&text).map_err(|err| {
                                        crate::types::error::Error::from_serde_error(
                                            format_serde_error::SerdeError::new(
                                                text.to_string(),
                                                err,
                                            ),
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
                            .map_ok(|result: crate::types::InvoiceResultsPage| {
                                Some((
                                    futures::stream::iter(result.items().into_iter().map(Ok)),
                                    (new_result.next_page_token(), result),
                                ))
                            })
                            .await
                        } else {
                            Ok(None)
                        }
                    },
                )
                .try_flatten();
                items.chain(next_pages)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Redirect to a fresh Stripe-hosted payment-method update link for your org.\n\nIf the request is not authenticated, this redirects to website login with a callback back to this endpoint. If authenticated as an org admin, it creates a fresh hosted Stripe portal session and redirects the browser to it.\n\n**Parameters:**\n\n- `return_url: Option<String>`: The URL Stripe should offer after the hosted flow completes.\n\nIf omitted, this defaults to the account page.\n\n```rust,no_run\nasync fn example_payments_redirect_method_portal_link_for_org() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client\n        .payments()\n        .redirect_method_portal_link_for_org(Some(\"https://example.com/foo/bar\".to_string()))\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn redirect_method_portal_link_for_org<'a>(
        &'a self,
        return_url: Option<String>,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url, "org/payment/method-portal-link"
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = return_url {
            query_params.push(("return_url", p));
        }

        req = req.query(&query_params);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Delete a payment method for your org.\n\nThis endpoint requires authentication by an \
             org admin. It deletes the specified payment method for the authenticated user's \
             org.\n\n**Parameters:**\n\n- `id: &'astr`: Stripe payment method identifier. \
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Get the subscription for an org.\n\nThis endpoint requires authentication by any member of an org. It gets the subscription for the authenticated user's org.\n\n```rust,no_run\nasync fn example_payments_get_org_subscription() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ZooProductSubscriptions =\n        client.payments().get_org_subscription().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Update the subscription for an org.\n\nThis endpoint requires authentication by an org admin. It updates the subscription for the authenticated user's org.\n\n```rust,no_run\nasync fn example_payments_update_org_subscription() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ZooProductSubscriptions = client\n        .payments()\n        .update_org_subscription(&kittycad::types::ZooProductSubscriptionsOrgRequest {\n            modeling_app: \"some-string\".to_string(),\n            pay_annually: Some(true),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Create the subscription for an org.\n\nThis endpoint requires authentication by an org admin. It creates the subscription for the authenticated user's org.\n\n```rust,no_run\nasync fn example_payments_create_org_subscription() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ZooProductSubscriptions = client\n        .payments()\n        .create_org_subscription(&kittycad::types::ZooProductSubscriptionsOrgRequest {\n            modeling_app: \"some-string\".to_string(),\n            pay_annually: Some(true),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Get balance for an org.\n\nThis endpoint requires authentication by a Zoo employee. \
             It gets the balance information for the specified org.\n\n**Parameters:**\n\n- `id: \
             uuid::Uuid`: The organization ID. (required)\n- `include_total_due: Option<bool>`: If \
             you would like to return the total due for a user. This makes the API call take \
             longer so it is off by default.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_payments_get_balance_for_any_org() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: kittycad::types::CustomerBalance = \
             client\n        .payments()\n        .get_balance_for_any_org(\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            \
             Some(true),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_balance_for_any_org<'a>(
        &'a self,
        id: uuid::Uuid,
        include_total_due: Option<bool>,
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
        let mut query_params = vec![];
        if let Some(p) = include_total_due {
            query_params.push(("include_total_due", format!("{}", p)));
        }

        req = req.query(&query_params);
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

    #[doc = "Update balance for an org.\n\nThis endpoint requires authentication by a Zoo employee. It updates the balance information for the specified org.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The organization ID. (required)\n- `include_total_due: Option<bool>`: If you would like to return the total due for a user. This makes the API call take longer so it is off by default.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_payments_update_balance_for_any_org() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::CustomerBalance = client\n        .payments()\n        .update_balance_for_any_org(\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            Some(true),\n            &kittycad::types::UpdatePaymentBalance {\n                monthly_api_credits_remaining_monetary_value: Some(3.14 as f64),\n                stable_api_credits_remaining_monetary_value: Some(3.14 as f64),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_balance_for_any_org<'a>(
        &'a self,
        id: uuid::Uuid,
        include_total_due: Option<bool>,
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
        let mut query_params = vec![];
        if let Some(p) = include_total_due {
            query_params.push(("include_total_due", format!("{}", p)));
        }

        req = req.query(&query_params);
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Update the subscription for any org (admin override).\n\nThis endpoint requires \
             authentication by a Zoo admin. It updates the subscription for the specified \
             org.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The organization ID. \
             (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_payments_update_org_subscription_for_any_org() -> anyhow::Result<()> {\n    \
             let client = kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::ZooProductSubscriptions = client\n        .payments()\n        \
             .update_org_subscription_for_any_org(\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            \
             &kittycad::types::ZooProductSubscriptionsOrgRequest {\n                modeling_app: \
             \"some-string\".to_string(),\n                pay_annually: Some(true),\n            \
             },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_org_subscription_for_any_org<'a>(
        &'a self,
        id: uuid::Uuid,
        body: &crate::types::ZooProductSubscriptionsOrgRequest,
    ) -> Result<crate::types::ZooProductSubscriptions, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "orgs/{id}/payment/subscriptions".replace("{id}", &format!("{}", id))
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Create or update a price for a subscription plan.\n\nYou must be a Zoo admin to perform this request.\n\n**Parameters:**\n\n- `slug: &'astr` (required)\n\n```rust,no_run\nasync fn example_payments_upsert_subscription_plan_price() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::SubscriptionPlanPriceRecord = client\n        .payments()\n        .upsert_subscription_plan_price(\n            \"some-string\",\n            &kittycad::types::PriceUpsertRequest {\n                active: true,\n                billing_model: kittycad::types::SubscriptionPlanBillingModel::PerUser,\n                cadence: kittycad::types::PlanInterval::Year,\n                unit_amount: 3.14 as f64,\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn upsert_subscription_plan_price<'a>(
        &'a self,
        slug: &'a str,
        body: &crate::types::PriceUpsertRequest,
    ) -> Result<crate::types::SubscriptionPlanPriceRecord, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "subscription-plans/{slug}/prices".replace("{slug}", slug)
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Get payment info about your user.\n\nThis includes billing address, phone, and \
             name.\n\nThis endpoint requires authentication by any Zoo user. It gets the payment \
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Update payment info for your user.\n\nThis includes billing address, phone, and \
             name.\n\nThis endpoint requires authentication by any Zoo user. It updates the \
             payment information for the authenticated user.\n\n```rust,no_run\nuse \
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Create payment info for your user.\n\nThis includes billing address, phone, and \
             name.\n\nThis endpoint requires authentication by any Zoo user. It creates the \
             payment information for the authenticated user.\n\n```rust,no_run\nuse \
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Delete payment info for your user.\n\nThis includes billing address, phone, and \
             name.\n\nThis endpoint requires authentication by any Zoo user. It deletes the \
             payment information for the authenticated user.\n\n```rust,no_run\nasync fn \
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Get balance for your user.\n\nThis endpoint requires authentication by any Zoo user. \
             It gets the balance information for the authenticated user.\n\n**Parameters:**\n\n- \
             `include_total_due: Option<bool>`: If you would like to return the total due for a \
             user. This makes the API call take longer so it is off by \
             default.\n\n```rust,no_run\nasync fn example_payments_get_balance_for_user() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::CustomerBalance =\n        \
             client.payments().get_balance_for_user(Some(true)).await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_balance_for_user<'a>(
        &'a self,
        include_total_due: Option<bool>,
    ) -> Result<crate::types::CustomerBalance, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "user/payment/balance"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = include_total_due {
            query_params.push(("include_total_due", format!("{}", p)));
        }

        req = req.query(&query_params);
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "List invoices for your user.\n\nThis endpoint requires authentication by any Zoo \
             user. It lists invoices for the authenticated user.\n\n**Parameters:**\n\n- `limit: \
             Option<u32>`: Maximum number of items returned by a single call\n- `page_token: \
             Option<String>`: Token returned by previous call to retrieve the subsequent \
             page\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn \
             example_payments_list_invoices_for_user_stream() -> anyhow::Result<()> {\n    let \
             client = kittycad::Client::new_from_env();\n    let mut payments = \
             client.payments();\n    let mut stream = \
             payments.list_invoices_for_user_stream(Some(4 as u32));\n    loop {\n        match \
             stream.try_next().await {\n            Ok(Some(item)) => {\n                \
             println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                \
             break;\n            }\n            Err(err) => {\n                return \
             Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_invoices_for_user<'a>(
        &'a self,
        limit: Option<u32>,
        page_token: Option<String>,
    ) -> Result<crate::types::InvoiceResultsPage, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "user/payment/invoices"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = limit {
            query_params.push(("limit", format!("{}", p)));
        }

        if let Some(p) = page_token {
            query_params.push(("page_token", p));
        }

        req = req.query(&query_params);
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

    #[doc = "List invoices for your user.\n\nThis endpoint requires authentication by any Zoo \
             user. It lists invoices for the authenticated user.\n\n**Parameters:**\n\n- `limit: \
             Option<u32>`: Maximum number of items returned by a single call\n- `page_token: \
             Option<String>`: Token returned by previous call to retrieve the subsequent \
             page\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn \
             example_payments_list_invoices_for_user_stream() -> anyhow::Result<()> {\n    let \
             client = kittycad::Client::new_from_env();\n    let mut payments = \
             client.payments();\n    let mut stream = \
             payments.list_invoices_for_user_stream(Some(4 as u32));\n    loop {\n        match \
             stream.try_next().await {\n            Ok(Some(item)) => {\n                \
             println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                \
             break;\n            }\n            Err(err) => {\n                return \
             Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_invoices_for_user_stream<'a>(
        &'a self,
        limit: Option<u32>,
    ) -> impl futures::Stream<Item = Result<crate::types::Invoice, crate::types::error::Error>>
           + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        self.list_invoices_for_user(limit, None)
            .map_ok(move |result| {
                let items = futures::stream::iter(result.items().into_iter().map(Ok));
                let next_pages = futures::stream::try_unfold(
                    (None, result),
                    move |(prev_page_token, new_result)| async move {
                        if new_result.has_more_pages()
                            && !new_result.items().is_empty()
                            && prev_page_token != new_result.next_page_token()
                        {
                            async {
                                let mut req = self.client.client.request(
                                    http::Method::GET,
                                    format!("{}/{}", self.client.base_url, "user/payment/invoices"),
                                );
                                req = req.bearer_auth(&self.client.token);
                                let mut request = req.build()?;
                                request = new_result.next_page(request)?;
                                let resp = self.client.client.execute(request).await?;
                                let status = resp.status();
                                if status.is_success() {
                                    let text = resp.text().await.unwrap_or_default();
                                    serde_json::from_str(&text).map_err(|err| {
                                        crate::types::error::Error::from_serde_error(
                                            format_serde_error::SerdeError::new(
                                                text.to_string(),
                                                err,
                                            ),
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
                            .map_ok(|result: crate::types::InvoiceResultsPage| {
                                Some((
                                    futures::stream::iter(result.items().into_iter().map(Ok)),
                                    (new_result.next_page_token(), result),
                                ))
                            })
                            .await
                        } else {
                            Ok(None)
                        }
                    },
                )
                .try_flatten();
                items.chain(next_pages)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Redirect to a fresh Stripe-hosted payment-method update link for your user.\n\nIf the request is not authenticated, this redirects to website login with a callback back to this endpoint. If authenticated, it creates a fresh hosted Stripe portal session and redirects the browser to it.\n\n**Parameters:**\n\n- `return_url: Option<String>`: The URL Stripe should offer after the hosted flow completes.\n\nIf omitted, this defaults to the account page.\n\n```rust,no_run\nasync fn example_payments_redirect_method_portal_link_for_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client\n        .payments()\n        .redirect_method_portal_link_for_user(Some(\"https://example.com/foo/bar\".to_string()))\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn redirect_method_portal_link_for_user<'a>(
        &'a self,
        return_url: Option<String>,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url, "user/payment/method-portal-link"
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = return_url {
            query_params.push(("return_url", p));
        }

        req = req.query(&query_params);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Delete a payment method for your user.\n\nThis endpoint requires authentication by \
             any Zoo user. It deletes the specified payment method for the authenticated \
             user.\n\n**Parameters:**\n\n- `force: Option<bool>`: Force deletion even when it is \
             the last payment method on file.\n- `id: &'astr`: Stripe payment method identifier. \
             (required)\n\n```rust,no_run\nasync fn example_payments_delete_method_for_user() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    \
             client\n        .payments()\n        .delete_method_for_user(Some(true), \
             \"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_method_for_user<'a>(
        &'a self,
        force: Option<bool>,
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
        let mut query_params = vec![];
        if let Some(p) = force {
            query_params.push(("force", format!("{}", p)));
        }

        req = req.query(&query_params);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Set the default payment method for your user.\n\nThis endpoint requires \
             authentication by any Zoo user. It sets the default payment method for the \
             authenticated user.\n\n**Parameters:**\n\n- `id: &'astr`: Stripe payment method \
             identifier. (required)\n\n```rust,no_run\nasync fn \
             example_payments_set_default_method_for_user() -> anyhow::Result<()> {\n    let \
             client = kittycad::Client::new_from_env();\n    client\n        .payments()\n        \
             .set_default_method_for_user(\"some-string\")\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn set_default_method_for_user<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/payment/methods/{id}/default".replace("{id}", id)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            let text = resp.text().await.unwrap_or_default();
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Update the user's subscription.\n\nThis endpoint requires authentication by any Zoo user. It updates the subscription for the user.\n\n```rust,no_run\nasync fn example_payments_update_user_subscription() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ZooProductSubscriptions = client\n        .payments()\n        .update_user_subscription(&kittycad::types::ZooProductSubscriptionsUserRequest {\n            modeling_app: \"some-string\".to_string(),\n            pay_annually: Some(true),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Create the subscription for a user.\n\nThis endpoint requires authentication by any Zoo user. It creates the subscription for the user.\n\n```rust,no_run\nasync fn example_payments_create_user_subscription() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ZooProductSubscriptions = client\n        .payments()\n        .create_user_subscription(&kittycad::types::ZooProductSubscriptionsUserRequest {\n            modeling_app: \"some-string\".to_string(),\n            pay_annually: Some(true),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Get balance for an user.\n\nThis endpoint requires authentication by a Zoo employee. \
             It gets the balance information for the specified user.\n\n**Parameters:**\n\n- `id: \
             &'astr`: The user's identifier (uuid or email). (required)\n- `include_total_due: \
             Option<bool>`: If you would like to return the total due for a user. This makes the \
             API call take longer so it is off by default.\n\n```rust,no_run\nasync fn \
             example_payments_get_balance_for_any_user() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: kittycad::types::CustomerBalance = \
             client\n        .payments()\n        .get_balance_for_any_user(\"some-string\", \
             Some(true))\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_balance_for_any_user<'a>(
        &'a self,
        id: &'a str,
        include_total_due: Option<bool>,
    ) -> Result<crate::types::CustomerBalance, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "users/{id}/payment/balance".replace("{id}", id)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = include_total_due {
            query_params.push(("include_total_due", format!("{}", p)));
        }

        req = req.query(&query_params);
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

    #[doc = "Update balance for an user.\n\nThis endpoint requires authentication by a Zoo employee. It updates the balance information for the specified user.\n\n**Parameters:**\n\n- `id: &'astr`: The user's identifier (uuid or email). (required)\n- `include_total_due: Option<bool>`: If you would like to return the total due for a user. This makes the API call take longer so it is off by default.\n\n```rust,no_run\nasync fn example_payments_update_balance_for_any_user() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::CustomerBalance = client\n        .payments()\n        .update_balance_for_any_user(\n            \"some-string\",\n            Some(true),\n            &kittycad::types::UpdatePaymentBalance {\n                monthly_api_credits_remaining_monetary_value: Some(3.14 as f64),\n                stable_api_credits_remaining_monetary_value: Some(3.14 as f64),\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_balance_for_any_user<'a>(
        &'a self,
        id: &'a str,
        include_total_due: Option<bool>,
        body: &crate::types::UpdatePaymentBalance,
    ) -> Result<crate::types::CustomerBalance, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "users/{id}/payment/balance".replace("{id}", id)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = include_total_due {
            query_params.push(("include_total_due", format!("{}", p)));
        }

        req = req.query(&query_params);
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }
}

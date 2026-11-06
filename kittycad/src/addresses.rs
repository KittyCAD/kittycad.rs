use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Addresses {
    pub client: Client,
}

impl Addresses {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "List saved addresses belonging to your active organization.\n\nCurrent members can \
             read delivery destinations; only admins see billing cards or manage the book. One-off \
             and archived destinations are omitted.\n\n**Parameters:**\n\n- `limit: Option<u32>`: \
             Maximum number of items returned by a single call\n- `page_token: Option<String>`: \
             Token returned by previous call to retrieve the subsequent \
             page\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn \
             example_addresses_list_org_stream() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let mut addresses = client.addresses();\n    \
             let mut stream = addresses.list_org_stream(Some(4 as u32));\n    loop {\n        \
             match stream.try_next().await {\n            Ok(Some(item)) => {\n                \
             println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                \
             break;\n            }\n            Err(err) => {\n                return \
             Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_org<'a>(
        &'a self,
        limit: Option<u32>,
        page_token: Option<String>,
    ) -> Result<crate::types::AddressResponseResultsPage, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "org/addresses"),
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
                    crate::types::error::SerdeError::new(text.to_string(), err),
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

    #[doc = "List saved addresses belonging to your active organization.\n\nCurrent members can \
             read delivery destinations; only admins see billing cards or manage the book. One-off \
             and archived destinations are omitted.\n\n**Parameters:**\n\n- `limit: Option<u32>`: \
             Maximum number of items returned by a single call\n- `page_token: Option<String>`: \
             Token returned by previous call to retrieve the subsequent \
             page\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn \
             example_addresses_list_org_stream() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let mut addresses = client.addresses();\n    \
             let mut stream = addresses.list_org_stream(Some(4 as u32));\n    loop {\n        \
             match stream.try_next().await {\n            Ok(Some(item)) => {\n                \
             println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                \
             break;\n            }\n            Err(err) => {\n                return \
             Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_org_stream<'a>(
        &'a self,
        limit: Option<u32>,
    ) -> impl futures::Stream<Item = Result<crate::types::AddressResponse, crate::types::error::Error>>
           + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        let pagination_url_path = ("org/addresses").to_string();
        let mut pagination_query_params: Vec<(&str, String)> = Vec::new();
        if let Some(p) = limit.as_ref() {
            pagination_query_params.push(("limit", format!("{}", p)));
        }

        let stream = self
            .list_org(limit, None)
            .map_ok(move |result| {
                let items = futures::stream::iter(result.items().into_iter().map(Ok));
                let next_pages = futures::stream::try_unfold(
                    (None, result),
                    move |(prev_page_token, new_result)| {
                        let pagination_url_path = pagination_url_path.clone();
                        let pagination_query_params = pagination_query_params.clone();
                        async move {
                            if new_result.has_more_pages()
                                && !new_result.items().is_empty()
                                && prev_page_token != new_result.next_page_token()
                            {
                                async {
                                    let mut req = self.client.client.request(
                                        http::Method::GET,
                                        format!(
                                            "{}/{}",
                                            self.client.base_url,
                                            pagination_url_path.clone()
                                        ),
                                    );
                                    req = req.bearer_auth(&self.client.token);
                                    let query_params = pagination_query_params.clone();
                                    req = req.query(&query_params);
                                    let mut request = req.build()?;
                                    request =
                                        new_result.next_page_with_param(request, "page_token")?;
                                    let resp = self.client.client.execute(request).await?;
                                    let status = resp.status();
                                    if status.is_success() {
                                        let text = resp.text().await.unwrap_or_default();
                                        serde_json::from_str(&text).map_err(|err| {
                                            crate::types::error::Error::from_serde_error(
                                                crate::types::error::SerdeError::new(
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
                                .map_ok(|result: crate::types::AddressResponseResultsPage| {
                                    Some((
                                        futures::stream::iter(result.items().into_iter().map(Ok)),
                                        (new_result.next_page_token(), result),
                                    ))
                                })
                                .await
                            } else {
                                Ok(None)
                            }
                        }
                    },
                )
                .try_flatten();
                items.chain(next_pages)
            })
            .try_flatten_stream();
        #[cfg(target_arch = "wasm32")]
        {
            stream.boxed_local()
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            stream.boxed()
        }
    }

    #[doc = "Create a saved or one-off destination belonging to your organization.\n\nAdmins can \
             save cards or select shipping defaults. Other current members can create one-off \
             destinations with is_saved=false without modifying the book.\n\n```rust,no_run\nasync \
             fn example_addresses_create_org_address() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: kittycad::types::AddressResponse = \
             client\n        .addresses()\n        \
             .create_org_address(&kittycad::types::CreateAddressRequest {\n            contact: \
             kittycad::types::AddressContact {\n                company: \
             Some(\"some-string\".to_string()),\n                phone: \
             Some(\"some-string\".to_string()),\n                recipient: \
             Some(\"some-string\".to_string()),\n            },\n            is_default_shipping: \
             true,\n            is_saved: true,\n            label: \
             Some(\"some-string\".to_string()),\n            postal: \
             kittycad::types::AddressDetails {\n                city: \
             Some(\"some-string\".to_string()),\n                country: \
             \"some-string\".to_string(),\n                state: \
             Some(\"some-string\".to_string()),\n                street_1: \
             Some(\"some-string\".to_string()),\n                street_2: \
             Some(\"some-string\".to_string()),\n                zip: \
             Some(\"some-string\".to_string()),\n            },\n        })\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_org_address<'a>(
        &'a self,
        body: &crate::types::CreateAddressRequest,
    ) -> Result<crate::types::AddressResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "org/addresses"),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.json(body);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    crate::types::error::SerdeError::new(text.to_string(), err),
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

    #[doc = "Get an address from your active organization's address book.\n\nRequires current \
             membership. Billing cards require organization-admin access.\n\n**Parameters:**\n\n- \
             `id: uuid::Uuid`: The identifier. (required)\n\n```rust,no_run\nuse \
             std::str::FromStr;\nasync fn example_addresses_get_org_address() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::AddressResponse = client\n        .addresses()\n        \
             .get_org_address(uuid::Uuid::from_str(\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_org_address<'a>(
        &'a self,
        id: uuid::Uuid,
    ) -> Result<crate::types::AddressResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "org/addresses/{id}".replace("{id}", &format!("{}", id))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    crate::types::error::SerdeError::new(text.to_string(), err),
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

    #[doc = "Replace an organization delivery address and its shipping-default selection.\n\nRequires organization-admin access and the current revision_id as expected_revision. Previously pinned order contents remain unchanged.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The identifier. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_addresses_update_org_address() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::AddressResponse = client\n        .addresses()\n        .update_org_address(\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            &kittycad::types::UpdateAddressRequest {\n                contact: kittycad::types::AddressContact {\n                    company: Some(\"some-string\".to_string()),\n                    phone: Some(\"some-string\".to_string()),\n                    recipient: Some(\"some-string\".to_string()),\n                },\n                expected_revision: uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n                is_default_shipping: true,\n                label: \"some-string\".to_string(),\n                postal: kittycad::types::AddressDetails {\n                    city: Some(\"some-string\".to_string()),\n                    country: \"some-string\".to_string(),\n                    state: Some(\"some-string\".to_string()),\n                    street_1: Some(\"some-string\".to_string()),\n                    street_2: Some(\"some-string\".to_string()),\n                    zip: Some(\"some-string\".to_string()),\n                },\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_org_address<'a>(
        &'a self,
        id: uuid::Uuid,
        body: &crate::types::UpdateAddressRequest,
    ) -> Result<crate::types::AddressResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "org/addresses/{id}".replace("{id}", &format!("{}", id))
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
                    crate::types::error::SerdeError::new(text.to_string(), err),
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

    #[doc = "Remove an organization delivery address from future selection.\n\nRequires \
             organization-admin access and the current expected_revision. Previously pinned \
             revisions remain available until organization erasure.\n\n**Parameters:**\n\n- \
             `expected_revision: uuid::Uuid`: revision_id returned when the card was loaded. \
             (required)\n- `id: uuid::Uuid`: The identifier. (required)\n\n```rust,no_run\nuse \
             std::str::FromStr;\nasync fn example_addresses_delete_org_address() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    \
             client\n        .addresses()\n        .delete_org_address(\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        )\n        \
             .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_org_address<'a>(
        &'a self,
        expected_revision: uuid::Uuid,
        id: uuid::Uuid,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "org/addresses/{id}".replace("{id}", &format!("{}", id))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("expected_revision", format!("{}", expected_revision))];
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

    #[doc = "List your saved addresses, including the current billing address.\n\nOne-off and \
             archived cards are omitted. Billing cards may contain incomplete historical details \
             and use the payment endpoints for edits. Follow next_page with page_token to read the \
             next page. Results are ordered by stable card ID.\n\n**Parameters:**\n\n- `limit: \
             Option<u32>`: Maximum number of items returned by a single call\n- `page_token: \
             Option<String>`: Token returned by previous call to retrieve the subsequent \
             page\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn \
             example_addresses_list_user_stream() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let mut addresses = client.addresses();\n    \
             let mut stream = addresses.list_user_stream(Some(4 as u32));\n    loop {\n        \
             match stream.try_next().await {\n            Ok(Some(item)) => {\n                \
             println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                \
             break;\n            }\n            Err(err) => {\n                return \
             Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_user<'a>(
        &'a self,
        limit: Option<u32>,
        page_token: Option<String>,
    ) -> Result<crate::types::AddressResponseResultsPage, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "user/addresses"),
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
                    crate::types::error::SerdeError::new(text.to_string(), err),
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

    #[doc = "List your saved addresses, including the current billing address.\n\nOne-off and \
             archived cards are omitted. Billing cards may contain incomplete historical details \
             and use the payment endpoints for edits. Follow next_page with page_token to read the \
             next page. Results are ordered by stable card ID.\n\n**Parameters:**\n\n- `limit: \
             Option<u32>`: Maximum number of items returned by a single call\n- `page_token: \
             Option<String>`: Token returned by previous call to retrieve the subsequent \
             page\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn \
             example_addresses_list_user_stream() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let mut addresses = client.addresses();\n    \
             let mut stream = addresses.list_user_stream(Some(4 as u32));\n    loop {\n        \
             match stream.try_next().await {\n            Ok(Some(item)) => {\n                \
             println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                \
             break;\n            }\n            Err(err) => {\n                return \
             Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_user_stream<'a>(
        &'a self,
        limit: Option<u32>,
    ) -> impl futures::Stream<Item = Result<crate::types::AddressResponse, crate::types::error::Error>>
           + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        let pagination_url_path = ("user/addresses").to_string();
        let mut pagination_query_params: Vec<(&str, String)> = Vec::new();
        if let Some(p) = limit.as_ref() {
            pagination_query_params.push(("limit", format!("{}", p)));
        }

        let stream = self
            .list_user(limit, None)
            .map_ok(move |result| {
                let items = futures::stream::iter(result.items().into_iter().map(Ok));
                let next_pages = futures::stream::try_unfold(
                    (None, result),
                    move |(prev_page_token, new_result)| {
                        let pagination_url_path = pagination_url_path.clone();
                        let pagination_query_params = pagination_query_params.clone();
                        async move {
                            if new_result.has_more_pages()
                                && !new_result.items().is_empty()
                                && prev_page_token != new_result.next_page_token()
                            {
                                async {
                                    let mut req = self.client.client.request(
                                        http::Method::GET,
                                        format!(
                                            "{}/{}",
                                            self.client.base_url,
                                            pagination_url_path.clone()
                                        ),
                                    );
                                    req = req.bearer_auth(&self.client.token);
                                    let query_params = pagination_query_params.clone();
                                    req = req.query(&query_params);
                                    let mut request = req.build()?;
                                    request =
                                        new_result.next_page_with_param(request, "page_token")?;
                                    let resp = self.client.client.execute(request).await?;
                                    let status = resp.status();
                                    if status.is_success() {
                                        let text = resp.text().await.unwrap_or_default();
                                        serde_json::from_str(&text).map_err(|err| {
                                            crate::types::error::Error::from_serde_error(
                                                crate::types::error::SerdeError::new(
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
                                .map_ok(|result: crate::types::AddressResponseResultsPage| {
                                    Some((
                                        futures::stream::iter(result.items().into_iter().map(Ok)),
                                        (new_result.next_page_token(), result),
                                    ))
                                })
                                .await
                            } else {
                                Ok(None)
                            }
                        }
                    },
                )
                .try_flatten();
                items.chain(next_pages)
            })
            .try_flatten_stream();
        #[cfg(target_arch = "wasm32")]
        {
            stream.boxed_local()
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            stream.boxed()
        }
    }

    #[doc = "Create a personal delivery address.\n\nSet is_saved=false for a one-off destination. \
             It remains owned and erasable while staying out of the picker. Creating a delivery \
             address never changes billing.\n\n```rust,no_run\nasync fn \
             example_addresses_create_user_address() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: kittycad::types::AddressResponse = \
             client\n        .addresses()\n        \
             .create_user_address(&kittycad::types::CreateAddressRequest {\n            contact: \
             kittycad::types::AddressContact {\n                company: \
             Some(\"some-string\".to_string()),\n                phone: \
             Some(\"some-string\".to_string()),\n                recipient: \
             Some(\"some-string\".to_string()),\n            },\n            is_default_shipping: \
             true,\n            is_saved: true,\n            label: \
             Some(\"some-string\".to_string()),\n            postal: \
             kittycad::types::AddressDetails {\n                city: \
             Some(\"some-string\".to_string()),\n                country: \
             \"some-string\".to_string(),\n                state: \
             Some(\"some-string\".to_string()),\n                street_1: \
             Some(\"some-string\".to_string()),\n                street_2: \
             Some(\"some-string\".to_string()),\n                zip: \
             Some(\"some-string\".to_string()),\n            },\n        })\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_user_address<'a>(
        &'a self,
        body: &crate::types::CreateAddressRequest,
    ) -> Result<crate::types::AddressResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "user/addresses"),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.json(body);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    crate::types::error::SerdeError::new(text.to_string(), err),
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

    #[doc = "Get a personal address and its current immutable revision.\n\n**Parameters:**\n\n- \
             `id: uuid::Uuid`: The identifier. (required)\n\n```rust,no_run\nuse \
             std::str::FromStr;\nasync fn example_addresses_get_user_address() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::AddressResponse = client\n        .addresses()\n        \
             .get_user_address(uuid::Uuid::from_str(\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_user_address<'a>(
        &'a self,
        id: uuid::Uuid,
    ) -> Result<crate::types::AddressResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/addresses/{id}".replace("{id}", &format!("{}", id))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    crate::types::error::SerdeError::new(text.to_string(), err),
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

    #[doc = "Replace a personal delivery address and its shipping-default selection.\n\nexpected_revision must match the current revision_id. Edits create immutable revisions, preserving any revision already selected by an order. Billing cards must be updated through the payment endpoints.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The identifier. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_addresses_update_user_address() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::AddressResponse = client\n        .addresses()\n        .update_user_address(\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            &kittycad::types::UpdateAddressRequest {\n                contact: kittycad::types::AddressContact {\n                    company: Some(\"some-string\".to_string()),\n                    phone: Some(\"some-string\".to_string()),\n                    recipient: Some(\"some-string\".to_string()),\n                },\n                expected_revision: uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n                is_default_shipping: true,\n                label: \"some-string\".to_string(),\n                postal: kittycad::types::AddressDetails {\n                    city: Some(\"some-string\".to_string()),\n                    country: \"some-string\".to_string(),\n                    state: Some(\"some-string\".to_string()),\n                    street_1: Some(\"some-string\".to_string()),\n                    street_2: Some(\"some-string\".to_string()),\n                    zip: Some(\"some-string\".to_string()),\n                },\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_user_address<'a>(
        &'a self,
        id: uuid::Uuid,
        body: &crate::types::UpdateAddressRequest,
    ) -> Result<crate::types::AddressResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/addresses/{id}".replace("{id}", &format!("{}", id))
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
                    crate::types::error::SerdeError::new(text.to_string(), err),
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

    #[doc = "Remove a personal address from future selection.\n\nRequires its current \
             expected_revision. Revisions pinned by orders remain available until account erasure. \
             An active billing address cannot be removed.\n\n**Parameters:**\n\n- \
             `expected_revision: uuid::Uuid`: revision_id returned when the card was loaded. \
             (required)\n- `id: uuid::Uuid`: The identifier. (required)\n\n```rust,no_run\nuse \
             std::str::FromStr;\nasync fn example_addresses_delete_user_address() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    \
             client\n        .addresses()\n        .delete_user_address(\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n        )\n        \
             .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_user_address<'a>(
        &'a self,
        expected_revision: uuid::Uuid,
        id: uuid::Uuid,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/addresses/{id}".replace("{id}", &format!("{}", id))
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let query_params = vec![("expected_revision", format!("{}", expected_revision))];
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
}

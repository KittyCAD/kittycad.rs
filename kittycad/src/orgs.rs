use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Orgs {
    pub client: Client,
}

impl Orgs {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get an org.\n\nThis endpoint requires authentication by an org admin. It gets the \
             authenticated user's org.\n\n```rust,no_run\nasync fn example_orgs_get() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::Org = client.orgs().get().await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get<'a>(&'a self) -> Result<crate::types::Org, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "org"),
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

    #[doc = "Update an org.\n\nThis endpoint requires authentication by an org admin. It updates the authenticated user's org.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_orgs_update() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::Org = client\n        .orgs()\n        .update(&kittycad::types::OrgDetails {\n            allow_users_in_domain_to_auto_join: Some(false),\n            billing_email: Some(\"email@example.com\".to_string()),\n            domain: Some(\"some-string\".to_string()),\n            image: Some(\"https://example.com/foo/bar\".to_string()),\n            name: Some(\"some-string\".to_string()),\n            phone: kittycad::types::phone_number::PhoneNumber::from_str(\"+1555-555-5555\")?,\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update<'a>(
        &'a self,
        body: &crate::types::OrgDetails,
    ) -> Result<crate::types::Org, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!("{}/{}", self.client.base_url, "org"),
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

    #[doc = "Create an org.\n\nThis endpoint requires authentication by a Zoo user that is not already in an org. It creates a new org for the authenticated user and makes them an admin.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_orgs_create() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::Org = client\n        .orgs()\n        .create(&kittycad::types::OrgDetails {\n            allow_users_in_domain_to_auto_join: Some(false),\n            billing_email: Some(\"email@example.com\".to_string()),\n            domain: Some(\"some-string\".to_string()),\n            image: Some(\"https://example.com/foo/bar\".to_string()),\n            name: Some(\"some-string\".to_string()),\n            phone: kittycad::types::phone_number::PhoneNumber::from_str(\"+1555-555-5555\")?,\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create<'a>(
        &'a self,
        body: &crate::types::OrgDetails,
    ) -> Result<crate::types::Org, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "org"),
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

    #[doc = "Delete an org.\n\nIn order to delete an org, you must first delete all of its \
             members, except yourself.\nYou must also have no outstanding invoices or unpaid \
             balances.\nThis endpoint requires authentication by an org admin. It deletes the \
             authenticated user's org.\n\n```rust,no_run\nasync fn example_orgs_delete() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    \
             client.orgs().delete().await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete<'a>(&'a self) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!("{}/{}", self.client.base_url, "org"),
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

    #[doc = "List members of your org.\n\nThis endpoint requires authentication by an org admin. It lists the members of the authenticated user's org.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `role: Option<crate::types::OrgRole>`: The organization role to filter by.\n- `sort_by: Option<crate::types::CreatedAtSortMode>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_orgs_list_members_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut orgs = client.orgs();\n    let mut stream = orgs.list_members_stream(\n        Some(4 as u32),\n        Some(kittycad::types::OrgRole::Member),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_members<'a>(
        &'a self,
        limit: Option<u32>,
        page_token: Option<String>,
        role: Option<crate::types::OrgRole>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> Result<crate::types::OrgMemberResultsPage, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "org/members"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = limit {
            query_params.push(("limit", format!("{}", p)));
        }

        if let Some(p) = page_token {
            query_params.push(("page_token", p));
        }

        if let Some(p) = role {
            query_params.push(("role", format!("{}", p)));
        }

        if let Some(p) = sort_by {
            query_params.push(("sort_by", format!("{}", p)));
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
            return Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            });
        }
    }

    #[doc = "List members of your org.\n\nThis endpoint requires authentication by an org admin. It lists the members of the authenticated user's org.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `role: Option<crate::types::OrgRole>`: The organization role to filter by.\n- `sort_by: Option<crate::types::CreatedAtSortMode>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_orgs_list_members_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut orgs = client.orgs();\n    let mut stream = orgs.list_members_stream(\n        Some(4 as u32),\n        Some(kittycad::types::OrgRole::Member),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_members_stream<'a>(
        &'a self,
        limit: Option<u32>,
        role: Option<crate::types::OrgRole>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> impl futures::Stream<Item = Result<crate::types::OrgMember, crate::types::error::Error>>
           + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        self.list_members(limit, None, role, sort_by)
            .map_ok(move |result| {
                let items = futures::stream::iter(result.items().into_iter().map(Ok));
                let next_pages =
                    futures::stream::try_unfold(result, move |new_result| async move {
                        if new_result.has_more_pages() {
                            async {
                                let mut req = self.client.client.request(
                                    http::Method::GET,
                                    format!("{}/{}", self.client.base_url, "org/members"),
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
                            .map_ok(|result: crate::types::OrgMemberResultsPage| {
                                Some((
                                    futures::stream::iter(result.items().into_iter().map(Ok)),
                                    result,
                                ))
                            })
                            .await
                        } else {
                            Ok(None)
                        }
                    })
                    .try_flatten();
                items.chain(next_pages)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Add a member to your org.\n\nThis endpoint requires authentication by an org admin. \
             It adds the specified member to the authenticated user's \
             org.\n\n```rust,no_run\nasync fn example_orgs_create_member() -> anyhow::Result<()> \
             {\n    let client = kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::OrgMember = client\n        .orgs()\n        \
             .create_member(&kittycad::types::AddOrgMember {\n            email: \
             \"email@example.com\".to_string(),\n            role: \
             kittycad::types::OrgRole::Member,\n        })\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_member<'a>(
        &'a self,
        body: &crate::types::AddOrgMember,
    ) -> Result<crate::types::OrgMember, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "org/members"),
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

    #[doc = "Get a member of your org.\n\nThis endpoint requires authentication by an org admin. \
             It gets the specified member of the authenticated user's org.\n\n**Parameters:**\n\n- \
             `user_id: uuid::Uuid`: The user id of the org member. \
             (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_orgs_get_member() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: kittycad::types::OrgMember = \
             client\n        .orgs()\n        .get_member(uuid::Uuid::from_str(\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_member<'a>(
        &'a self,
        user_id: uuid::Uuid,
    ) -> Result<crate::types::OrgMember, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "org/members/{user_id}".replace("{user_id}", &format!("{}", user_id))
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

    #[doc = "Update a member of your org.\n\nThis endpoint requires authentication by an org \
             admin. It updates the specified member of the authenticated user's \
             org.\n\n**Parameters:**\n\n- `user_id: uuid::Uuid`: The user id of the org member. \
             (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_orgs_update_member() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: kittycad::types::OrgMember = \
             client\n        .orgs()\n        .update_member(\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            \
             &kittycad::types::UpdateMemberToOrgBody {\n                role: \
             kittycad::types::OrgRole::Member,\n            },\n        )\n        .await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_member<'a>(
        &'a self,
        user_id: uuid::Uuid,
        body: &crate::types::UpdateMemberToOrgBody,
    ) -> Result<crate::types::OrgMember, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "org/members/{user_id}".replace("{user_id}", &format!("{}", user_id))
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

    #[doc = "Remove a member from your org.\n\nThis endpoint requires authentication by an org \
             admin. It removes the specified member from the authenticated user's \
             org.\n\n**Parameters:**\n\n- `user_id: uuid::Uuid`: The user id of the org member. \
             (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_orgs_delete_member() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    client\n        .orgs()\n        \
             .delete_member(uuid::Uuid::from_str(\n            \
             \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_member<'a>(
        &'a self,
        user_id: uuid::Uuid,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "org/members/{user_id}".replace("{user_id}", &format!("{}", user_id))
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

    #[doc = "Get a user's org.\n\nThis endpoint requires authentication by any Zoo user. It gets \
             the authenticated user's org.\nIf the user is not a member of an org, this endpoint \
             will return a 404.\n\n```rust,no_run\nasync fn example_orgs_get_user() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::UserOrgInfo = client.orgs().get_user().await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_user<'a>(
        &'a self,
    ) -> Result<crate::types::UserOrgInfo, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "user/org"),
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
}
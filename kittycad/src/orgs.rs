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

    #[doc = "List members of your org.\n\nThis endpoint requires authentication by an org admin. It lists the members of the authenticated user's org.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `role: Option<crate::types::UserOrgRole>`: The organization role to filter by.\n- `sort_by: Option<crate::types::CreatedAtSortMode>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_orgs_list_members_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut orgs = client.orgs();\n    let mut stream = orgs.list_members_stream(\n        Some(4 as u32),\n        Some(kittycad::types::UserOrgRole::Member),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_members<'a>(
        &'a self,
        limit: Option<u32>,
        page_token: Option<String>,
        role: Option<crate::types::UserOrgRole>,
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

    #[doc = "List members of your org.\n\nThis endpoint requires authentication by an org admin. It lists the members of the authenticated user's org.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `role: Option<crate::types::UserOrgRole>`: The organization role to filter by.\n- `sort_by: Option<crate::types::CreatedAtSortMode>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_orgs_list_members_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut orgs = client.orgs();\n    let mut stream = orgs.list_members_stream(\n        Some(4 as u32),\n        Some(kittycad::types::UserOrgRole::Member),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_members_stream<'a>(
        &'a self,
        limit: Option<u32>,
        role: Option<crate::types::UserOrgRole>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> impl futures::Stream<Item = Result<crate::types::OrgMember, crate::types::error::Error>>
           + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        self.list_members(limit, None, role, sort_by)
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

    #[doc = "Add a member to your org.\n\nIf the user exists, this will add them to your org. If \
             they do not exist, this will create a new user and add them to your org.\nIn both \
             cases the user gets an email that they have been added to the org.\nIf the user is \
             already in your org, this will return a 400 and a message.\nIf the user is already in \
             a different org, this will return a 400 and a message.\nThis endpoint requires \
             authentication by an org admin. It adds the specified member to the authenticated \
             user's org.\n\n```rust,no_run\nasync fn example_orgs_create_member() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::OrgMember = client\n        .orgs()\n        \
             .create_member(&kittycad::types::AddOrgMember {\n            email: \
             \"email@example.com\".to_string(),\n            role: \
             kittycad::types::UserOrgRole::Member,\n        })\n        .await?;\n    \
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

    #[doc = "Update a member of your org.\n\nThis endpoint requires authentication by an org admin. It updates the specified member of the authenticated user's org.\n\n**Parameters:**\n\n- `user_id: uuid::Uuid`: The user id of the org member. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_orgs_update_member() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::OrgMember = client\n        .orgs()\n        .update_member(\n            uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            &kittycad::types::UpdateMemberToOrgBody {\n                role: kittycad::types::UserOrgRole::Member,\n            },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
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

    #[doc = "Get the privacy settings for an org.\n\nThis endpoint requires authentication by an \
             org admin. It gets the privacy settings for the authenticated user's \
             org.\n\n```rust,no_run\nasync fn example_orgs_get_privacy_settings() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::PrivacySettings = \
             client.orgs().get_privacy_settings().await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_privacy_settings<'a>(
        &'a self,
    ) -> Result<crate::types::PrivacySettings, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "org/privacy"),
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

    #[doc = "Update the privacy settings for an org.\n\nThis endpoint requires authentication by \
             an org admin. It updates the privacy settings for the authenticated user's \
             org.\n\n```rust,no_run\nasync fn example_orgs_update_privacy_settings() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::PrivacySettings = client\n        .orgs()\n        \
             .update_privacy_settings(&kittycad::types::PrivacySettings {\n            \
             can_train_on_data: false,\n        })\n        .await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_privacy_settings<'a>(
        &'a self,
        body: &crate::types::PrivacySettings,
    ) -> Result<crate::types::PrivacySettings, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!("{}/{}", self.client.base_url, "org/privacy"),
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

    #[doc = "Get the SAML identity provider.\n\nThis endpoint requires authentication by an org \
             admin.\n\n```rust,no_run\nasync fn example_orgs_get_saml_idp() -> anyhow::Result<()> \
             {\n    let client = kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::SamlIdentityProvider = client.orgs().get_saml_idp().await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_saml_idp<'a>(
        &'a self,
    ) -> Result<crate::types::SamlIdentityProvider, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "org/saml/idp"),
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

    #[doc = "Update the SAML identity provider.\n\nThis endpoint requires authentication by an org admin.\n\n```rust,no_run\nasync fn example_orgs_update_saml_idp() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::SamlIdentityProvider = client\n        .orgs()\n        .update_saml_idp(&kittycad::types::SamlIdentityProviderCreate {\n            idp_entity_id: Some(\"some-string\".to_string()),\n            idp_metadata_source: kittycad::types::IdpMetadataSource::Base64EncodedXml {\n                data: kittycad::types::base64::Base64Data(\n                    \"some-base64-encoded-string\".as_bytes().to_vec(),\n                ),\n            },\n            signing_keypair: Some(kittycad::types::DerEncodedKeyPair {\n                private_key: kittycad::types::base64::Base64Data(\n                    \"some-base64-encoded-string\".as_bytes().to_vec(),\n                ),\n                public_cert: kittycad::types::base64::Base64Data(\n                    \"some-base64-encoded-string\".as_bytes().to_vec(),\n                ),\n            }),\n            technical_contact_email: Some(\"email@example.com\".to_string()),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_saml_idp<'a>(
        &'a self,
        body: &crate::types::SamlIdentityProviderCreate,
    ) -> Result<crate::types::SamlIdentityProvider, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!("{}/{}", self.client.base_url, "org/saml/idp"),
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

    #[doc = "Create a SAML identity provider.\n\nThis endpoint requires authentication by an org admin.\n\n```rust,no_run\nasync fn example_orgs_create_saml_idp() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::SamlIdentityProvider = client\n        .orgs()\n        .create_saml_idp(&kittycad::types::SamlIdentityProviderCreate {\n            idp_entity_id: Some(\"some-string\".to_string()),\n            idp_metadata_source: kittycad::types::IdpMetadataSource::Base64EncodedXml {\n                data: kittycad::types::base64::Base64Data(\n                    \"some-base64-encoded-string\".as_bytes().to_vec(),\n                ),\n            },\n            signing_keypair: Some(kittycad::types::DerEncodedKeyPair {\n                private_key: kittycad::types::base64::Base64Data(\n                    \"some-base64-encoded-string\".as_bytes().to_vec(),\n                ),\n                public_cert: kittycad::types::base64::Base64Data(\n                    \"some-base64-encoded-string\".as_bytes().to_vec(),\n                ),\n            }),\n            technical_contact_email: Some(\"email@example.com\".to_string()),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_saml_idp<'a>(
        &'a self,
        body: &crate::types::SamlIdentityProviderCreate,
    ) -> Result<crate::types::SamlIdentityProvider, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "org/saml/idp"),
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

    #[doc = "Delete an SAML identity provider.\n\nThis endpoint requires authentication by an org \
             admin.\n\n```rust,no_run\nasync fn example_orgs_delete_saml_idp() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    \
             client.orgs().delete_saml_idp().await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_saml_idp<'a>(&'a self) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!("{}/{}", self.client.base_url, "org/saml/idp"),
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

    #[doc = "List orgs.\n\nThis endpoint requires authentication by a Zoo employee. The orgs are returned in order of creation, with the most recently created orgs first.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `sort_by: Option<crate::types::CreatedAtSortMode>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_orgs_list_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut orgs = client.orgs();\n    let mut stream = orgs.list_stream(\n        Some(4 as u32),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list<'a>(
        &'a self,
        limit: Option<u32>,
        page_token: Option<String>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> Result<crate::types::OrgResultsPage, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "orgs"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = limit {
            query_params.push(("limit", format!("{}", p)));
        }

        if let Some(p) = page_token {
            query_params.push(("page_token", p));
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

    #[doc = "List orgs.\n\nThis endpoint requires authentication by a Zoo employee. The orgs are returned in order of creation, with the most recently created orgs first.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `sort_by: Option<crate::types::CreatedAtSortMode>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_orgs_list_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut orgs = client.orgs();\n    let mut stream = orgs.list_stream(\n        Some(4 as u32),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_stream<'a>(
        &'a self,
        limit: Option<u32>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> impl futures::Stream<Item = Result<crate::types::Org, crate::types::error::Error>> + Unpin + '_
    {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        self.list(limit, None, sort_by)
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
                                    format!("{}/{}", self.client.base_url, "orgs"),
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
                            .map_ok(|result: crate::types::OrgResultsPage| {
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

    #[doc = "Get an org.\n\nThis endpoint requires authentication by a Zoo employee. It gets the information for the specified org.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The organization ID. (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_orgs_get_any() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::Org = client\n        .orgs()\n        .get_any(uuid::Uuid::from_str(\n            \"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\",\n        )?)\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_any<'a>(
        &'a self,
        id: uuid::Uuid,
    ) -> Result<crate::types::Org, crate::types::error::Error> {
        let mut req =
            self.client.client.request(
                http::Method::GET,
                format!(
                    "{}/{}",
                    self.client.base_url,
                    "orgs/{id}".replace("{id}", &format!("{}", id))
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

    #[doc = "Set the enterprise price for an organization.\n\nYou must be a Zoo employee to \
             perform this request.\n\n**Parameters:**\n\n- `id: uuid::Uuid`: The organization ID. \
             (required)\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn \
             example_orgs_update_enterprise_pricing_for() -> anyhow::Result<()> {\n    let client \
             = kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::ZooProductSubscriptions = client\n        .orgs()\n        \
             .update_enterprise_pricing_for(\n            \
             uuid::Uuid::from_str(\"d9797f8d-9ad6-4e08-90d7-2ec17e13471c\")?,\n            \
             &kittycad::types::SubscriptionTierPrice::Enterprise {},\n        )\n        \
             .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_enterprise_pricing_for<'a>(
        &'a self,
        id: uuid::Uuid,
        body: &crate::types::SubscriptionTierPrice,
    ) -> Result<crate::types::ZooProductSubscriptions, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "orgs/{id}/enterprise/pricing".replace("{id}", &format!("{}", id))
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

use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Users {
    pub client: Client,
}

impl Users {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get your user.\n\nGet the user information for the authenticated \
             user.\n\nAlternatively, you can also use the `/users/me` \
             endpoint.\n\n```rust,no_run\nasync fn example_users_get_self() -> anyhow::Result<()> \
             {\n    let client = kittycad::Client::new_from_env();\n    let result: \
             kittycad::types::User = client.users().get_self().await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_self<'a>(&'a self) -> Result<crate::types::User, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "user"),
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

    #[doc = "Update your user.\n\nThis endpoint requires authentication by any Zoo user. It updates information about the authenticated user.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_users_update_self() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::User = client\n        .users()\n        .update_self(&kittycad::types::UpdateUser {\n            company: Some(\"some-string\".to_string()),\n            discord: Some(\"some-string\".to_string()),\n            first_name: Some(\"some-string\".to_string()),\n            github: Some(\"some-string\".to_string()),\n            image: \"https://example.com/foo/bar\".to_string(),\n            is_onboarded: Some(true),\n            last_name: Some(\"some-string\".to_string()),\n            phone: kittycad::types::phone_number::PhoneNumber::from_str(\"+1555-555-5555\")?,\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_self<'a>(
        &'a self,
        body: &crate::types::UpdateUser,
    ) -> Result<crate::types::User, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!("{}/{}", self.client.base_url, "user"),
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

    #[doc = "Delete your user.\n\nThis endpoint requires authentication by any Zoo user. It \
             deletes the authenticated user from Zoo's database.\n\nThis call will only succeed if \
             all invoices associated with the user have been paid in full and there is no \
             outstanding balance.\n\n```rust,no_run\nasync fn example_users_delete_self() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    \
             client.users().delete_self().await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_self<'a>(&'a self) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!("{}/{}", self.client.base_url, "user"),
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

    #[doc = "Update properties in the CRM\n\n```rust,no_run\nasync fn example_users_patch_crm() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    \
             client\n        .users()\n        .patch_crm(&kittycad::types::CrmData {\n            \
             cad_industry: Some(\"some-string\".to_string()),\n            cad_user_count: \
             Some(\"some-string\".to_string()),\n            cad_user_type: \
             Some(\"some-string\".to_string()),\n        })\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn patch_crm<'a>(
        &'a self,
        body: &crate::types::CrmData,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PATCH,
            format!("{}/{}", self.client.base_url, "user/crm"),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.json(body);
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

    #[doc = "Get extended information about your user.\n\nGet the user information for the authenticated user.\n\nAlternatively, you can also use the `/users-extended/me` endpoint.\n\n```rust,no_run\nasync fn example_users_get_self_extended() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ExtendedUser = client.users().get_self_extended().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_self_extended<'a>(
        &'a self,
    ) -> Result<crate::types::ExtendedUser, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "user/extended"),
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

    #[doc = "Create a new support/sales ticket from the website contact form. This endpoint is authenticated.\n\nIt gets attached to the user's account.\n\n```rust,no_run\nasync fn example_users_put_form_self() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client\n        .users()\n        .put_form_self(&kittycad::types::InquiryForm {\n            company: Some(\"some-string\".to_string()),\n            email: \"email@example.com\".to_string(),\n            first_name: \"some-string\".to_string(),\n            industry: Some(\"some-string\".to_string()),\n            inquiry_type: kittycad::types::InquiryType::OtherSalesInquiry,\n            last_name: \"some-string\".to_string(),\n            message: \"some-string\".to_string(),\n            phone: Some(\"some-string\".to_string()),\n        })\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn put_form_self<'a>(
        &'a self,
        body: &crate::types::InquiryForm,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!("{}/{}", self.client.base_url, "user/form"),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.json(body);
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

    #[doc = "Get the OAuth2 providers for your user.\n\nIf this returns an empty array, then the user has not connected any OAuth2 providers and uses raw email authentication.\n\nThis endpoint requires authentication by any Zoo user. It gets the providers for the authenticated user.\n\n```rust,no_run\nasync fn example_users_get_oauth_2_providers_for() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: Vec<kittycad::types::AccountProvider> =\n        client.users().get_oauth_2_providers_for().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_oauth_2_providers_for<'a>(
        &'a self,
    ) -> Result<Vec<crate::types::AccountProvider>, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "user/oauth2/providers"),
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

    #[doc = "Get your user's onboarding status.\n\nChecks key part of their api usage to determine \
             their onboarding progress\n\n```rust,no_run\nasync fn \
             example_users_get_onboarding_self() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: kittycad::types::Onboarding = \
             client.users().get_onboarding_self().await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_onboarding_self<'a>(
        &'a self,
    ) -> Result<crate::types::Onboarding, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "user/onboarding"),
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

    #[doc = "Get the privacy settings for a user.\n\nThis endpoint requires authentication by any \
             Zoo user. It gets the privacy settings for the user.\n\n```rust,no_run\nasync fn \
             example_users_get_privacy_settings() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: kittycad::types::PrivacySettings = \
             client.users().get_privacy_settings().await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_privacy_settings<'a>(
        &'a self,
    ) -> Result<crate::types::PrivacySettings, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "user/privacy"),
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

    #[doc = "Update the user's privacy settings.\n\nThis endpoint requires authentication by any \
             Zoo user. It updates the privacy settings for the user.\n\n```rust,no_run\nasync fn \
             example_users_update_privacy_settings() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: kittycad::types::PrivacySettings = \
             client\n        .users()\n        \
             .update_privacy_settings(&kittycad::types::PrivacySettings {\n            \
             can_train_on_data: true,\n        })\n        .await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_privacy_settings<'a>(
        &'a self,
        body: &crate::types::PrivacySettings,
    ) -> Result<crate::types::PrivacySettings, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!("{}/{}", self.client.base_url, "user/privacy"),
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

    #[doc = "Get a session for your user.\n\nThis endpoint requires authentication by any Zoo \
             user. It returns details of the requested API token for the \
             user.\n\n**Parameters:**\n\n- `token: &'astr`: The API token. \
             (required)\n\n```rust,no_run\nasync fn example_users_get_session_for() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::Session = \
             client.users().get_session_for(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_session_for<'a>(
        &'a self,
        token: &'a str,
    ) -> Result<crate::types::Session, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/session/{token}".replace("{token}", token)
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Get the shortlinks for a user.\n\nThis endpoint requires authentication by any Zoo user. It gets the shortlinks for the user.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `sort_by: Option<crate::types::CreatedAtSortMode>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_users_get_shortlinks_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut users = client.users();\n    let mut stream = users.get_shortlinks_stream(\n        Some(4 as u32),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_shortlinks<'a>(
        &'a self,
        limit: Option<u32>,
        page_token: Option<String>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> Result<crate::types::ShortlinkResultsPage, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "user/shortlinks"),
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Get the shortlinks for a user.\n\nThis endpoint requires authentication by any Zoo user. It gets the shortlinks for the user.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `sort_by: Option<crate::types::CreatedAtSortMode>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_users_get_shortlinks_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut users = client.users();\n    let mut stream = users.get_shortlinks_stream(\n        Some(4 as u32),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn get_shortlinks_stream<'a>(
        &'a self,
        limit: Option<u32>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> impl futures::Stream<Item = Result<crate::types::Shortlink, crate::types::error::Error>>
           + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        self.get_shortlinks(limit, None, sort_by)
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
                                    format!("{}/{}", self.client.base_url, "user/shortlinks"),
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
                            .map_ok(|result: crate::types::ShortlinkResultsPage| {
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

    #[doc = "Create a shortlink for a user.\n\nThis endpoint requires authentication by any Zoo user. It creates a shortlink for the user.\n\n```rust,no_run\nasync fn example_users_create_shortlink() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::CreateShortlinkResponse = client\n        .users()\n        .create_shortlink(&kittycad::types::CreateShortlinkRequest {\n            password: Some(\"some-string\".to_string()),\n            restrict_to_org: true,\n            url: \"https://example.com/foo/bar\".to_string(),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn create_shortlink<'a>(
        &'a self,
        body: &crate::types::CreateShortlinkRequest,
    ) -> Result<crate::types::CreateShortlinkResponse, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "user/shortlinks"),
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

    #[doc = "Update a shortlink for a user.\n\nThis endpoint requires authentication by any Zoo user. It updates a shortlink for the user.\n\nThis endpoint really only allows you to change the `restrict_to_org` setting of a shortlink. Thus it is only useful for folks who are part of an org. If you are not part of an org, you will not be able to change the `restrict_to_org` status.\n\n**Parameters:**\n\n- `key: &'astr`: The key of the shortlink. (required)\n\n```rust,no_run\nasync fn example_users_update_shortlink() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client\n        .users()\n        .update_shortlink(\n            \"some-string\",\n            &kittycad::types::UpdateShortlinkRequest {\n                password: Some(\"some-string\".to_string()),\n                restrict_to_org: true,\n            },\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_shortlink<'a>(
        &'a self,
        key: &'a str,
        body: &crate::types::UpdateShortlinkRequest,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/shortlinks/{key}".replace("{key}", key)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.json(body);
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

    #[doc = "Delete a shortlink for a user.\n\nThis endpoint requires authentication by any Zoo \
             user. It deletes a shortlink for the user.\n\n**Parameters:**\n\n- `key: &'astr`: The \
             key of the shortlink. (required)\n\n```rust,no_run\nasync fn \
             example_users_delete_shortlink() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    \
             client.users().delete_shortlink(\"some-string\").await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn delete_shortlink<'a>(
        &'a self,
        key: &'a str,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            format!(
                "{}/{}",
                self.client.base_url,
                "user/shortlinks/{key}".replace("{key}", key)
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

    #[doc = "List users.\n\nThis endpoint requires authentication by a Zoo employee. The users are returned in order of creation, with the most recently created users first.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `sort_by: Option<crate::types::CreatedAtSortMode>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_users_list_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut users = client.users();\n    let mut stream = users.list_stream(\n        Some(4 as u32),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list<'a>(
        &'a self,
        limit: Option<u32>,
        page_token: Option<String>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> Result<crate::types::UserResultsPage, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "users"),
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "List users.\n\nThis endpoint requires authentication by a Zoo employee. The users are returned in order of creation, with the most recently created users first.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `sort_by: Option<crate::types::CreatedAtSortMode>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_users_list_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut users = client.users();\n    let mut stream = users.list_stream(\n        Some(4 as u32),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_stream<'a>(
        &'a self,
        limit: Option<u32>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> impl futures::Stream<Item = Result<crate::types::User, crate::types::error::Error>> + Unpin + '_
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
                                    format!("{}/{}", self.client.base_url, "users"),
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
                            .map_ok(|result: crate::types::UserResultsPage| {
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

    #[doc = "List users with extended information.\n\nThis endpoint requires authentication by a Zoo employee. The users are returned in order of creation, with the most recently created users first.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `sort_by: Option<crate::types::CreatedAtSortMode>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_users_list_extended_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut users = client.users();\n    let mut stream = users.list_extended_stream(\n        Some(4 as u32),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn list_extended<'a>(
        &'a self,
        limit: Option<u32>,
        page_token: Option<String>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> Result<crate::types::ExtendedUserResultsPage, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "users-extended"),
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "List users with extended information.\n\nThis endpoint requires authentication by a Zoo employee. The users are returned in order of creation, with the most recently created users first.\n\n**Parameters:**\n\n- `limit: Option<u32>`: Maximum number of items returned by a single call\n- `page_token: Option<String>`: Token returned by previous call to retrieve the subsequent page\n- `sort_by: Option<crate::types::CreatedAtSortMode>`\n\n```rust,no_run\nuse futures_util::TryStreamExt;\nasync fn example_users_list_extended_stream() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let mut users = client.users();\n    let mut stream = users.list_extended_stream(\n        Some(4 as u32),\n        Some(kittycad::types::CreatedAtSortMode::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err.into());\n            }\n        }\n    }\n\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    #[cfg(not(feature = "js"))]
    pub fn list_extended_stream<'a>(
        &'a self,
        limit: Option<u32>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> impl futures::Stream<Item = Result<crate::types::ExtendedUser, crate::types::error::Error>>
           + Unpin
           + '_ {
        use futures::{StreamExt, TryFutureExt, TryStreamExt};

        use crate::types::paginate::Pagination;
        self.list_extended(limit, None, sort_by)
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
                                    format!("{}/{}", self.client.base_url, "users-extended"),
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
                            .map_ok(|result: crate::types::ExtendedUserResultsPage| {
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

    #[doc = "Get extended information about a user.\n\nTo get information about yourself, use \
             `/users-extended/me` as the endpoint. By doing so you will get the user information \
             for the authenticated user.\n\nAlternatively, to get information about the \
             authenticated user, use `/user/extended` endpoint.\n\nTo get information about any \
             Zoo user, you must be a Zoo employee.\n\n**Parameters:**\n\n- `id: &'astr`: The \
             user's identifier (uuid or email). (required)\n\n```rust,no_run\nasync fn \
             example_users_get_extended() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: kittycad::types::ExtendedUser = \
             client.users().get_extended(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_extended<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<crate::types::ExtendedUser, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "users-extended/{id}".replace("{id}", id)
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Get a user.\n\nTo get information about yourself, use `/users/me` as the endpoint. By \
             doing so you will get the user information for the authenticated \
             user.\n\nAlternatively, to get information about the authenticated user, use `/user` \
             endpoint.\n\nTo get information about any Zoo user, you must be a Zoo \
             employee.\n\n**Parameters:**\n\n- `id: &'astr`: The user's identifier (uuid or \
             email). (required)\n\n```rust,no_run\nasync fn example_users_get() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::User = client.users().get(\"some-string\").await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<crate::types::User, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "users/{id}".replace("{id}", id)
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
            Err(crate::types::error::Error::Server {
                body: text.to_string(),
                status,
            })
        }
    }

    #[doc = "Creates a new support/sales ticket from the website contact form. This endpoint is for untrusted\n\nusers and is not authenticated.\n\n```rust,no_run\nasync fn example_users_put_public_form() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client\n        .users()\n        .put_public_form(&kittycad::types::InquiryForm {\n            company: Some(\"some-string\".to_string()),\n            email: \"email@example.com\".to_string(),\n            first_name: \"some-string\".to_string(),\n            industry: Some(\"some-string\".to_string()),\n            inquiry_type: kittycad::types::InquiryType::OtherSalesInquiry,\n            last_name: \"some-string\".to_string(),\n            message: \"some-string\".to_string(),\n            phone: Some(\"some-string\".to_string()),\n        })\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn put_public_form<'a>(
        &'a self,
        body: &crate::types::InquiryForm,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!("{}/{}", self.client.base_url, "website/form"),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.json(body);
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

    #[doc = "Subscribes a user to the newsletter.\n\n```rust,no_run\nasync fn \
             example_users_put_public_subscribe() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    client\n        .users()\n        \
             .put_public_subscribe(&kittycad::types::Subscribe {\n            email: \
             \"email@example.com\".to_string(),\n        })\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn put_public_subscribe<'a>(
        &'a self,
        body: &crate::types::Subscribe,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!("{}/{}", self.client.base_url, "website/subscribe"),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.json(body);
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

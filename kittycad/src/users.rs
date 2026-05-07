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
             kittycad::types::UserResponse = client.users().get_self().await?;\n    \
             println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_self<'a>(
        &'a self,
    ) -> Result<crate::types::UserResponse, crate::types::error::Error> {
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

    #[doc = "Update your user.\n\nThis endpoint requires authentication by any Zoo user. It updates information about the authenticated user.\n\n```rust,no_run\nuse std::str::FromStr;\nasync fn example_users_update_self() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UserResponse = client\n        .users()\n        .update_self(&kittycad::types::UpdateUser {\n            company: Some(\"some-string\".to_string()),\n            discord: Some(\"some-string\".to_string()),\n            first_name: Some(\"some-string\".to_string()),\n            github: Some(\"some-string\".to_string()),\n            image: \"https://example.com/foo/bar\".to_string(),\n            is_onboarded: Some(true),\n            last_name: Some(\"some-string\".to_string()),\n            phone: kittycad::types::phone_number::PhoneNumber::from_str(\"+1555-555-5555\")?,\n            username: Some(\"some-string\".to_string()),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_self<'a>(
        &'a self,
        body: &crate::types::UpdateUser,
    ) -> Result<crate::types::UserResponse, crate::types::error::Error> {
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

    #[doc = "Gets authenticated CAD user info form data for the current \
             user.\n\n```rust,no_run\nasync fn example_users_get_cad_info_form() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::WebsiteCadUserInfoForm = \
             client.users().get_cad_info_form().await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get_cad_info_form<'a>(
        &'a self,
    ) -> Result<crate::types::WebsiteCadUserInfoForm, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "user/cad-user-info"),
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

    #[doc = "Report a client-originated error.\n\nThis endpoint requires authentication by any Zoo user. It accepts a structured client error payload and writes it to the server logs for triage.\n\n```rust,no_run\nasync fn example_users_report_client_error() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::ClientErrorReportAccepted = client\n        .users()\n        .report_client_error(&kittycad::types::ClientErrorReport {\n            client: \"some-string\".to_string(),\n            code: Some(\"some-string\".to_string()),\n            error_name: Some(\"some-string\".to_string()),\n            message: \"some-string\".to_string(),\n            release: \"some-string\".to_string(),\n            route: Some(\"some-string\".to_string()),\n            stack: Some(\"some-string\".to_string()),\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn report_client_error<'a>(
        &'a self,
        body: &crate::types::ClientErrorReport,
    ) -> Result<crate::types::ClientErrorReportAccepted, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!("{}/{}", self.client.base_url, "user/client-errors"),
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

    #[doc = "Get email marketing consent state for the authenticated \
             user.\n\n```rust,no_run\nasync fn example_users_email_marketing_consent_get() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::EmailMarketingConsentState =\n        \
             client.users().email_marketing_consent_get().await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn email_marketing_consent_get<'a>(
        &'a self,
    ) -> Result<crate::types::EmailMarketingConsentState, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url, "user/email-marketing-consent"
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

    #[doc = "Record explicit decline for email marketing consent.\n\n```rust,no_run\nasync fn \
             example_users_email_marketing_consent_decline_post() -> anyhow::Result<()> {\n    let \
             client = kittycad::Client::new_from_env();\n    client\n        .users()\n        \
             .email_marketing_consent_decline_post()\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn email_marketing_consent_decline_post<'a>(
        &'a self,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url, "user/email-marketing-consent/decline"
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

    #[doc = "Request email marketing opt-in and send a confirmation \
             email.\n\n```rust,no_run\nasync fn \
             example_users_email_marketing_consent_request_post() -> anyhow::Result<()> {\n    let \
             client = kittycad::Client::new_from_env();\n    client\n        .users()\n        \
             .email_marketing_consent_request_post()\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn email_marketing_consent_request_post<'a>(
        &'a self,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url, "user/email-marketing-consent/request"
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

    #[doc = "Mark the email-marketing modal as seen/dismissed for the authenticated \
             user.\n\n```rust,no_run\nasync fn example_users_email_marketing_consent_seen_post() \
             -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    \
             client.users().email_marketing_consent_seen_post().await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn email_marketing_consent_seen_post<'a>(
        &'a self,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::POST,
            format!(
                "{}/{}",
                self.client.base_url, "user/email-marketing-consent/seen"
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

    #[doc = "List user-visible feature flags enabled for the authenticated user.\n\nReturns only \
             features that are marked as safe for exposure to clients and currently resolved to \
             `true` for the requesting user (including org overrides).\n\n```rust,no_run\nasync fn \
             example_users_features_get() -> anyhow::Result<()> {\n    let client = \
             kittycad::Client::new_from_env();\n    let result: kittycad::types::UserFeatureList = \
             client.users().features_get().await?;\n    println!(\"{:?}\", result);\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn features_get<'a>(
        &'a self,
    ) -> Result<crate::types::UserFeatureList, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "user/features"),
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
        let pagination_url_path = ("user/shortlinks").to_string();
        let mut pagination_query_params: Vec<(&str, String)> = Vec::new();
        if let Some(p) = limit.as_ref() {
            pagination_query_params.push(("limit", format!("{}", p)));
        }

        if let Some(p) = sort_by.as_ref() {
            pagination_query_params.push(("sort_by", format!("{}", p)));
        }

        self.get_shortlinks(limit, None, sort_by)
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

    #[doc = "Get extended information about a user.\n\nTo get information about yourself, use \
             `/users-extended/me` as the endpoint. By doing so you will get the user information \
             for the authenticated user.\n\nAlternatively, to get information about the \
             authenticated user, use `/user/extended` endpoint.\n\n**Parameters:**\n\n- `id: \
             &'astr`: The user's identifier (uuid or email). (required)\n\n```rust,no_run\nasync \
             fn example_users_get_extended() -> anyhow::Result<()> {\n    let client = \
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

    #[doc = "Get a user.\n\nTo get information about yourself, use `/users/me` as the endpoint. By doing so you will get the user information for the authenticated user.\n\nAlternatively, to get information about the authenticated user, use `/user` endpoint.\n\n**Parameters:**\n\n- `id: &'astr`: The user's identifier (uuid or email). (required)\n\n```rust,no_run\nasync fn example_users_get() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let result: kittycad::types::UserResponse = client.users().get(\"some-string\").await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn get<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<crate::types::UserResponse, crate::types::error::Error> {
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

    #[doc = "Get admin-only details for a user.\n\nZoo admins can retrieve extended information \
             about any user, while non-admins receive a 404 to avoid leaking the existence of the \
             resource.\n\n**Parameters:**\n\n- `id: &'astr`: The user's identifier (uuid or \
             email). (required)\n\n```rust,no_run\nasync fn example_users_admin_details_get() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::UserAdminDetails =\n        \
             client.users().admin_details_get(\"some-string\").await?;\n    println!(\"{:?}\", \
             result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn admin_details_get<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<crate::types::UserAdminDetails, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            format!(
                "{}/{}",
                self.client.base_url,
                "users/{id}/admin/details".replace("{id}", id)
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

    #[doc = "Update a subscription for a user.\n\nYou must be a Zoo admin to perform this \
             request.\n\n**Parameters:**\n\n- `id: &'astr`: The user's identifier (uuid or email). \
             (required)\n\n```rust,no_run\nasync fn example_users_update_subscription_for() -> \
             anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    let \
             result: kittycad::types::ZooProductSubscriptions = client\n        .users()\n        \
             .update_subscription_for(\n            \"some-string\",\n            \
             &kittycad::types::ZooProductSubscriptionsUserRequest {\n                modeling_app: \
             \"some-string\".to_string(),\n                pay_annually: Some(true),\n            \
             },\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn update_subscription_for<'a>(
        &'a self,
        id: &'a str,
        body: &crate::types::ZooProductSubscriptionsUserRequest,
    ) -> Result<crate::types::ZooProductSubscriptions, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "users/{id}/payment/subscriptions".replace("{id}", id)
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

    #[doc = "Requests public email marketing consent for an email address.\n\n```rust,no_run\nasync fn example_users_put_public_email_marketing_consent_request() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client\n        .users()\n        .put_public_email_marketing_consent_request(\n            &kittycad::types::PublicEmailMarketingConsentRequest {\n                email: \"email@example.com\".to_string(),\n            },\n        )\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn put_public_email_marketing_consent_request<'a>(
        &'a self,
        body: &crate::types::PublicEmailMarketingConsentRequest,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url, "website/email-marketing-consent/request"
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

    #[doc = "Publicly subscribe an email address to a mailing list by \
             slug.\n\n**Parameters:**\n\n- `slug: &'astr`: Stable public list slug. \
             (required)\n\n```rust,no_run\nasync fn \
             example_users_put_public_mailing_list_subscribe() -> anyhow::Result<()> {\n    let \
             client = kittycad::Client::new_from_env();\n    client\n        .users()\n        \
             .put_public_mailing_list_subscribe(\n            \"some-string\",\n            \
             &kittycad::types::PublicMailingListMembershipRequest {\n                email: \
             \"email@example.com\".to_string(),\n            },\n        )\n        .await?;\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn put_public_mailing_list_subscribe<'a>(
        &'a self,
        slug: &'a str,
        body: &crate::types::PublicMailingListMembershipRequest,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "website/email-marketing-lists/{slug}/subscribe".replace("{slug}", slug)
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

    #[doc = "Publicly remove an email address from a mailing list by slug.\n\n**Parameters:**\n\n- \
             `slug: &'astr`: Stable public list slug. (required)\n\n```rust,no_run\nasync fn \
             example_users_put_public_mailing_list_unsubscribe() -> anyhow::Result<()> {\n    let \
             client = kittycad::Client::new_from_env();\n    client\n        .users()\n        \
             .put_public_mailing_list_unsubscribe(\n            \"some-string\",\n            \
             &kittycad::types::PublicMailingListMembershipRequest {\n                email: \
             \"email@example.com\".to_string(),\n            },\n        )\n        .await?;\n    \
             Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn put_public_mailing_list_unsubscribe<'a>(
        &'a self,
        slug: &'a str,
        body: &crate::types::PublicMailingListMembershipRequest,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!(
                "{}/{}",
                self.client.base_url,
                "website/email-marketing-lists/{slug}/unsubscribe".replace("{slug}", slug)
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

    #[doc = "Stores authenticated CAD user info form data for the current user.\n\n```rust,no_run\nasync fn example_users_put_cad_info_form() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client\n        .users()\n        .put_cad_info_form(&kittycad::types::WebsiteCadUserInfoForm {\n            cad_experience_level: Some(kittycad::types::CadExperienceLevel::Intermediate),\n            cad_industry: Some(kittycad::types::CadIndustry::Construction),\n            cad_user_type: Some(kittycad::types::CadUserType::Hobbyist),\n            company_size: Some(kittycad::types::CompanySize::FiveHundredOneToOneThousand),\n            design_workflow: Some(kittycad::types::CadDesignWorkflow::Ai),\n            has_used_zoo_design_studio_or_api_before: Some(true),\n            how_did_you_find_us: Some(kittycad::types::CadDiscoverySource::Instagram),\n            how_did_you_find_us_other: Some(\"some-string\".to_string()),\n            location_city: Some(\"some-string\".to_string()),\n            location_country: Some(\"some-string\".to_string()),\n            location_state: Some(\"some-string\".to_string()),\n            number_of_cad_users: Some(\"some-string\".to_string()),\n            what_are_you_building: Some(\"some-string\".to_string()),\n        })\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn put_cad_info_form<'a>(
        &'a self,
        body: &crate::types::WebsiteCadUserInfoForm,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!("{}/{}", self.client.base_url, "website/forms/cad-user-info"),
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

    #[doc = "Creates a new sales ticket in the internal help desk from the website sales form.\n\nThis endpoint accepts optional authentication.\n\n```rust,no_run\nasync fn example_users_put_public_sales_form() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client\n        .users()\n        .put_public_sales_form(&kittycad::types::WebsiteSalesForm {\n            cad_platforms: Some(vec![\"some-string\".to_string()]),\n            company: Some(\"some-string\".to_string()),\n            email: \"email@example.com\".to_string(),\n            first_name: \"some-string\".to_string(),\n            industry: Some(\"some-string\".to_string()),\n            inquiry_type: kittycad::types::SalesInquiryType::DeveloperInquiry,\n            job_title: Some(\"some-string\".to_string()),\n            last_name: \"some-string\".to_string(),\n            message: \"some-string\".to_string(),\n            num_cad_users: Some(\"some-string\".to_string()),\n            phone: Some(\"some-string\".to_string()),\n        })\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn put_public_sales_form<'a>(
        &'a self,
        body: &crate::types::WebsiteSalesForm,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!("{}/{}", self.client.base_url, "website/forms/sales"),
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

    #[doc = "Creates a new support ticket in the internal help desk from the website support form.\n\nThis endpoint accepts optional authentication.\n\n```rust,no_run\nasync fn example_users_put_public_support_form() -> anyhow::Result<()> {\n    let client = kittycad::Client::new_from_env();\n    client\n        .users()\n        .put_public_support_form(&kittycad::types::WebsiteSupportForm {\n            company: Some(\"some-string\".to_string()),\n            email: \"email@example.com\".to_string(),\n            first_name: \"some-string\".to_string(),\n            inquiry_type: kittycad::types::SupportInquiryType::AccountManagement,\n            last_name: \"some-string\".to_string(),\n            message: \"some-string\".to_string(),\n            phone: Some(\"some-string\".to_string()),\n        })\n        .await?;\n    Ok(())\n}\n```"]
    #[tracing::instrument]
    pub async fn put_public_support_form<'a>(
        &'a self,
        body: &crate::types::WebsiteSupportForm,
    ) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            format!("{}/{}", self.client.base_url, "website/forms/support"),
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

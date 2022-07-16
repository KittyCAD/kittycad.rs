use crate::Client;
use anyhow::Result;
pub struct Users {
    pub client: Client,
}

impl Users {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get your user.\n\nGet the user information for the authenticated user.\nAlternatively, you can also use the `/users/me` endpoint."]
    pub async fn get_user_self(&self) -> Result<crate::types::User> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "user"),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        if status.is_success() {
            serde_json::from_str(&text)
                .map_err(|err| format_serde_error::SerdeError::new(text.to_string(), err).into())
        } else {
            Err(anyhow::anyhow!(
                "response was not successful `{}` -> `{}`",
                status,
                text
            ))
        }
    }

    #[doc = "Update your user.\n\nThis endpoint requires authentication by any KittyCAD user. It updates information about the authenticated user."]
    pub async fn update_user_self(
        &self,
        body: &crate::types::UpdateUser,
    ) -> Result<crate::types::User> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            &format!("{}/{}", self.client.base_url, "user"),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.json(body);
        let resp = req.send().await?;
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        if status.is_success() {
            serde_json::from_str(&text)
                .map_err(|err| format_serde_error::SerdeError::new(text.to_string(), err).into())
        } else {
            Err(anyhow::anyhow!(
                "response was not successful `{}` -> `{}`",
                status,
                text
            ))
        }
    }

    #[doc = "Delete your user.\n\nThis endpoint requires authentication by any KittyCAD user. It deletes the authenticated user from KittyCAD's database.\nThis call will only succeed if all invoices associated with the user have been paid in full and there is no outstanding balance."]
    pub async fn delete_user_self(&self) -> Result<()> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            &format!("{}/{}", self.client.base_url, "user"),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        if status.is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "response was not successful `{}` -> `{}`",
                status,
                text
            ))
        }
    }

    #[doc = "Get extended information about your user.\n\nGet the user information for the authenticated user.\nAlternatively, you can also use the `/users-extended/me` endpoint."]
    pub async fn get_user_self_extended(&self) -> Result<crate::types::ExtendedUser> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "user/extended"),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        if status.is_success() {
            serde_json::from_str(&text)
                .map_err(|err| format_serde_error::SerdeError::new(text.to_string(), err).into())
        } else {
            Err(anyhow::anyhow!(
                "response was not successful `{}` -> `{}`",
                status,
                text
            ))
        }
    }

    #[doc = "List users.\n\nThis endpoint required authentication by a KittyCAD employee. The users are returned in order of creation, with the most recently created users first."]
    pub async fn list(
        &self,
        limit: Option<u32>,
        page_token: Option<String>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> Result<crate::types::UserResultsPage> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "users"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
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
        let text = resp.text().await.unwrap_or_default();
        if status.is_success() {
            serde_json::from_str(&text)
                .map_err(|err| format_serde_error::SerdeError::new(text.to_string(), err).into())
        } else {
            Err(anyhow::anyhow!(
                "response was not successful `{}` -> `{}`",
                status,
                text
            ))
        }
    }

    #[doc = "List users.\n\nThis endpoint required authentication by a KittyCAD employee. The users are returned in order of creation, with the most recently created users first."]
    pub async fn list_stream(
        &self,
        limit: Option<u32>,
        page_token: Option<String>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> Result<crate::types::UserResultsPage> {
        use crate::types::paginate::Pagination;
        let mut result = self
            .list(limit, page_token.clone(), sort_by.clone())
            .await?;
        if result.has_more_pages()? {
            result = {
                let mut req = self.client.client.request(
                    http::Method::GET,
                    &format!("{}/{}", self.client.base_url, "users"),
                );
                req = req.bearer_auth(&self.client.token);
                let resp = req.send().await?;
                let status = resp.status();
                let text = resp.text().await.unwrap_or_default();
                if status.is_success() {
                    serde_json::from_str(&text).map_err(|err| {
                        format_serde_error::SerdeError::new(text.to_string(), err).into()
                    })
                } else {
                    Err(anyhow::anyhow!(
                        "response was not successful `{}` -> `{}`",
                        status,
                        text
                    ))
                }
            }?;
        }

        Ok(result)
    }

    #[doc = "List users with extended information.\n\nThis endpoint required authentication by a KittyCAD employee. The users are returned in order of creation, with the most recently created users first."]
    pub async fn list_extended(
        &self,
        limit: Option<u32>,
        page_token: Option<String>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> Result<crate::types::ExtendedUserResultsPage> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "users-extended"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
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
        let text = resp.text().await.unwrap_or_default();
        if status.is_success() {
            serde_json::from_str(&text)
                .map_err(|err| format_serde_error::SerdeError::new(text.to_string(), err).into())
        } else {
            Err(anyhow::anyhow!(
                "response was not successful `{}` -> `{}`",
                status,
                text
            ))
        }
    }

    #[doc = "List users with extended information.\n\nThis endpoint required authentication by a KittyCAD employee. The users are returned in order of creation, with the most recently created users first."]
    pub async fn list_extended_stream(
        &self,
        limit: Option<u32>,
        page_token: Option<String>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> Result<crate::types::ExtendedUserResultsPage> {
        use crate::types::paginate::Pagination;
        let mut result = self
            .list_extended(limit, page_token.clone(), sort_by.clone())
            .await?;
        if result.has_more_pages()? {
            result = {
                let mut req = self.client.client.request(
                    http::Method::GET,
                    &format!("{}/{}", self.client.base_url, "users-extended"),
                );
                req = req.bearer_auth(&self.client.token);
                let resp = req.send().await?;
                let status = resp.status();
                let text = resp.text().await.unwrap_or_default();
                if status.is_success() {
                    serde_json::from_str(&text).map_err(|err| {
                        format_serde_error::SerdeError::new(text.to_string(), err).into()
                    })
                } else {
                    Err(anyhow::anyhow!(
                        "response was not successful `{}` -> `{}`",
                        status,
                        text
                    ))
                }
            }?;
        }

        Ok(result)
    }

    #[doc = "Get extended information about a user.\n\nTo get information about yourself, use `/users-extended/me` as the endpoint. By doing so you will get the user information for the authenticated user.\nAlternatively, to get information about the authenticated user, use `/user/extended` endpoint.\nTo get information about any KittyCAD user, you must be a KittyCAD employee."]
    pub async fn get_user_extended(&self, id: String) -> Result<crate::types::ExtendedUser> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "users-extended/{id}".replace("{id}", &id)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        if status.is_success() {
            serde_json::from_str(&text)
                .map_err(|err| format_serde_error::SerdeError::new(text.to_string(), err).into())
        } else {
            Err(anyhow::anyhow!(
                "response was not successful `{}` -> `{}`",
                status,
                text
            ))
        }
    }

    #[doc = "Get a user.\n\nTo get information about yourself, use `/users/me` as the endpoint. By doing so you will get the user information for the authenticated user.\nAlternatively, to get information about the authenticated user, use `/user` endpoint.\nTo get information about any KittyCAD user, you must be a KittyCAD employee."]
    pub async fn get_user(&self, id: String) -> Result<crate::types::User> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "users/{id}".replace("{id}", &id)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        if status.is_success() {
            serde_json::from_str(&text)
                .map_err(|err| format_serde_error::SerdeError::new(text.to_string(), err).into())
        } else {
            Err(anyhow::anyhow!(
                "response was not successful `{}` -> `{}`",
                status,
                text
            ))
        }
    }
}

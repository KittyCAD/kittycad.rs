use anyhow::Result;

use crate::Client;

pub struct Users {
    pub client: Client,
}

impl Users {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Users { client }
    }

    #[doc = "Get your user.\n\nGet the user information for the authenticated user.\nAlternatively, you can also use the `/users/me` endpoint."]
    pub async fn get_user_self(&self) -> Result<crate::types::User> {
        let mut rb = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "user"),
        );
        rb = rb.bearer_auth(self.client.token);
        let req = rb.build()?;
        let resp = self.client.client.execute(req).await?;
        resp.json()?
    }

    #[doc = "Update your user.\n\nThis endpoint requires authentication by any KittyCAD user. It updates information about the authenticated user."]
    pub async fn update_user_self(
        &self,
        body: &crate::types::UpdateUser,
    ) -> Result<crate::types::User> {
        let mut rb = self.client.client.request(
            http::Method::PUT,
            &format!("{}/{}", self.client.base_url, "user"),
        );
        rb = rb.bearer_auth(self.client.token);
        rb = rb.json(body);
        let req = rb.build()?;
        let resp = self.client.client.execute(req).await?;
        resp.json()?
    }

    #[doc = "Delete your user.\n\nThis endpoint requires authentication by any KittyCAD user. It deletes the authenticated user from KittyCAD's database.\nThis call will only succeed if all invoices associated with the user have been paid in full and there is no outstanding balance."]
    pub async fn delete_user_self(&self) -> Result<()> {
        let mut rb = self.client.client.request(
            http::Method::DELETE,
            &format!("{}/{}", self.client.base_url, "user"),
        );
        rb = rb.bearer_auth(self.client.token);
        let req = rb.build()?;
        let resp = self.client.client.execute(req).await?;
        resp.json()?
    }

    #[doc = "Get extended information about your user.\n\nGet the user information for the authenticated user.\nAlternatively, you can also use the `/users-extended/me` endpoint."]
    pub async fn get_user_self_extended(&self) -> Result<crate::types::ExtendedUser> {
        let mut rb = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "user/extended"),
        );
        rb = rb.bearer_auth(self.client.token);
        let req = rb.build()?;
        let resp = self.client.client.execute(req).await?;
        resp.json()?
    }

    #[doc = "List users.\n\nThis endpoint required authentication by a KittyCAD employee. The users are returned in order of creation, with the most recently created users first."]
    pub async fn list(
        &self,
        limit: Option<u32>,
        page_token: Option<String>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> Result<crate::types::UserResultsPage> {
        let mut rb = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "users"),
        );
        rb = rb.bearer_auth(self.client.token);
        let req = rb.build()?;
        let resp = self.client.client.execute(req).await?;
        resp.json()?
    }

    #[doc = "List users with extended information.\n\nThis endpoint required authentication by a KittyCAD employee. The users are returned in order of creation, with the most recently created users first."]
    pub async fn list_extended(
        &self,
        limit: Option<u32>,
        page_token: Option<String>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> Result<crate::types::ExtendedUserResultsPage> {
        let mut rb = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "users-extended"),
        );
        rb = rb.bearer_auth(self.client.token);
        let req = rb.build()?;
        let resp = self.client.client.execute(req).await?;
        resp.json()?
    }

    #[doc = "Get extended information about a user.\n\nTo get information about yourself, use `/users-extended/me` as the endpoint. By doing so you will get the user information for the authenticated user.\nAlternatively, to get information about the authenticated user, use `/user/extended` endpoint.\nTo get information about any KittyCAD user, you must be a KittyCAD employee."]
    pub async fn get_user_extended(&self, id: String) -> Result<crate::types::ExtendedUser> {
        let mut rb = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "users-extended/{id}"),
        );
        rb = rb.bearer_auth(self.client.token);
        let req = rb.build()?;
        let resp = self.client.client.execute(req).await?;
        resp.json()?
    }

    #[doc = "Get a user.\n\nTo get information about yourself, use `/users/me` as the endpoint. By doing so you will get the user information for the authenticated user.\nAlternatively, to get information about the authenticated user, use `/user` endpoint.\nTo get information about any KittyCAD user, you must be a KittyCAD employee."]
    pub async fn get_user(&self, id: String) -> Result<crate::types::User> {
        let mut rb = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "users/{id}"),
        );
        rb = rb.bearer_auth(self.client.token);
        let req = rb.build()?;
        let resp = self.client.client.execute(req).await?;
        resp.json()?
    }
}

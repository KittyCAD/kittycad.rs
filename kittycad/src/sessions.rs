use anyhow::Result;

use crate::Client;

pub struct Sessions {
    pub client: Client,
}

impl Sessions {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Sessions { client }
    }

    /**
     * Get a session for your user.
     *
     * This function performs a `GET` to the `/user/session/{token}` endpoint.
     *
     * This endpoint requires authentication by any KittyCAD user. It returns details of the requested API token for the user.
     *
     * **Parameters:**
     *
     * * `token: &str` -- The API token.
     */
    pub async fn get_for_user(&self, token: &str) -> Result<crate::types::Session> {
        let url = format!(
            "/user/session/{}",
            crate::progenitor_support::encode_path(token),
        );

        self.client.get(&url, None).await
    }
}

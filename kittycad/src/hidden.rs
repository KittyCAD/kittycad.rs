use anyhow::Result;

use crate::Client;

pub struct Hidden {
    pub client: Client,
}

impl Hidden {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Hidden { client }
    }

    /**
     * This endpoint sets a session cookie for a user.
     *
     * This function performs a `POST` to the `/login` endpoint.
     */
    pub async fn login(&self, body: &crate::types::LoginParams) -> Result<()> {
        let url = "/login".to_string();
        self.client
            .post(&url, Some(reqwest::Body::from(serde_json::to_vec(body)?)))
            .await
    }
}

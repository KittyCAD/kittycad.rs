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

    #[doc = "Create an email verification request for a user."]
    pub fn listen_auth_email(&self) -> Result<crate::types::VerificationToken> {
        todo!()
    }

    #[doc = "Listen for callbacks for email verification for users."]
    pub fn listen_auth_email_callback(&self) -> Result<()> {
        todo!()
    }

    #[doc = "This endpoint removes the session cookie for a user.\n\nThis is used in logout scenarios."]
    pub fn logout(&self) -> Result<()> {
        todo!()
    }
}

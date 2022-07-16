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

    #[doc = "Get a session for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It returns details of the requested API token for the user."]
    pub fn get_session_for_user(&self) -> Result<Session> {
        todo!()
    }
}

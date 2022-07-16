use anyhow::Result;

use crate::Client;

pub struct Oauth2 {
    pub client: Client,
}

impl Oauth2 {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Oauth2 { client }
    }
}

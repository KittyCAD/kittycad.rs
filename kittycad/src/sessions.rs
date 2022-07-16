

use crate::Client;

pub struct Sessions {
    pub client: Client,
}

impl Sessions {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Sessions { client }
    }
}

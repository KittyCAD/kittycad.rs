

use crate::Client;

pub struct File {
    pub client: Client,
}

impl File {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        File { client }
    }
}

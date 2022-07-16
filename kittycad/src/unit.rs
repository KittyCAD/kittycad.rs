

use crate::Client;

pub struct Unit {
    pub client: Client,
}

impl Unit {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Unit { client }
    }
}

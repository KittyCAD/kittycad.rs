

use crate::Client;

pub struct Payments {
    pub client: Client,
}

impl Payments {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Payments { client }
    }
}

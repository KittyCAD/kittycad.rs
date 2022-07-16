use anyhow::Result;

            use crate::Client;

            pub struct ApiTokens {
                pub client: Client,
            }

            impl ApiTokens {
                #[doc(hidden)]
                pub fn new(client: Client) -> Self
                {
                    ApiTokens {
                        client,
                    }
                }

                
            }
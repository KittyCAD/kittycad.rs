use anyhow::Result;

            use crate::Client;

            pub struct ApiCalls {
                pub client: Client,
            }

            impl ApiCalls {
                #[doc(hidden)]
                pub fn new(client: Client) -> Self
                {
                    ApiCalls {
                        client,
                    }
                }

                
            }
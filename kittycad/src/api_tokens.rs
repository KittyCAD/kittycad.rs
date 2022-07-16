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

                #[doc = "List API tokens for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It returns the API tokens for the authenticated user.\nThe API tokens are returned in order of creation, with the most recently created API tokens first."]
pub fn list_for_user(
    &self,
    limit: Option<u32>,
    page_token: Option<String>,
    sort_by: Option<crate::types::CreatedAtSortMode>,
) -> Result<crate::types::ApiTokenResultsPage> {
    todo!()
}


#[doc = "Create a new API token for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It creates a new API token for the authenticated user."]
pub fn create_api_token_for_user(&self) -> Result<crate::types::ApiToken> {
    todo!()
}


#[doc = "Get an API token for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It returns details of the requested API token for the user."]
pub fn get_api_token_for_user(&self, token: uuid::Uuid) -> Result<crate::types::ApiToken> {
    todo!()
}


#[doc = "Delete an API token for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It deletes the requested API token for the user.\nThis endpoint does not actually delete the API token from the database. It merely marks the token as invalid. We still want to keep the token in the database for historical purposes."]
pub fn delete_api_token_for_user(&self, token: uuid::Uuid) -> Result<()> {
    todo!()
}

            }
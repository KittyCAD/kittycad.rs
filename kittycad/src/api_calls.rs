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

                #[doc = "Get API call metrics.\n\nThis endpoint requires authentication by a KittyCAD employee. The API calls are grouped by the parameter passed."]
pub fn get_api_call_metrics(
    &self,
    group_by: crate::types::ApiCallQueryGroupBy,
) -> Result<Vec<crate::types::ApiCallQueryGroup>> {
    todo!()
}


#[doc = "List API calls.\n\nThis endpoint requires authentication by a KittyCAD employee. The API calls are returned in order of creation, with the most recently created API calls first."]
pub fn list(
    &self,
    limit: Option<u32>,
    page_token: Option<String>,
    sort_by: Option<crate::types::CreatedAtSortMode>,
) -> Result<crate::types::ApiCallWithPriceResultsPage> {
    todo!()
}


#[doc = "Get details of an API call.\n\nThis endpoint requires authentication by any KittyCAD user. It returns details of the requested API call for the user.\nIf the user is not authenticated to view the specified API call, then it is not returned.\nOnly KittyCAD employees can view API calls for other users."]
pub fn get_api_call(&self, id: String) -> Result<crate::types::ApiCallWithPrice> {
    todo!()
}


#[doc = "List async operations.\n\nFor async file conversion operations, this endpoint does not return the contents of converted files (`output`). To get the contents use the `/async/operations/{id}` endpoint.\nThis endpoint requires authentication by a KittyCAD employee."]
pub fn list_async_operations(
    &self,
    limit: Option<u32>,
    page_token: Option<String>,
    sort_by: Option<crate::types::CreatedAtSortMode>,
    status: Option<crate::types::ApiCallStatus>,
) -> Result<crate::types::AsyncApiCallResultsPage> {
    todo!()
}


#[doc = "Get an async operation.\n\nGet the status and output of an async operation.\nThis endpoint requires authentication by any KittyCAD user. It returns details of the requested async operation for the user.\nIf the user is not authenticated to view the specified async operation, then it is not returned.\nOnly KittyCAD employees with the proper access can view async operations for other users."]
pub fn get_async_operation(&self, id: String) -> Result<crate::types::AsyncApiCallOutput> {
    todo!()
}


#[doc = "List API calls for your user.\n\nThis endpoint requires authentication by any KittyCAD user. It returns the API calls for the authenticated user.\nThe API calls are returned in order of creation, with the most recently created API calls first."]
pub fn user_list(
    &self,
    limit: Option<u32>,
    page_token: Option<String>,
    sort_by: Option<crate::types::CreatedAtSortMode>,
) -> Result<crate::types::ApiCallWithPriceResultsPage> {
    todo!()
}


#[doc = "Get an API call for a user.\n\nThis endpoint requires authentication by any KittyCAD user. It returns details of the requested API call for the user."]
pub fn get_api_call_for_user(&self, id: String) -> Result<crate::types::ApiCallWithPrice> {
    todo!()
}


#[doc = "List API calls for a user.\n\nThis endpoint requires authentication by any KittyCAD user. It returns the API calls for the authenticated user if \"me\" is passed as the user id.\nAlternatively, you can use the `/user/api-calls` endpoint to get the API calls for your user.\nIf the authenticated user is a KittyCAD employee, then the API calls are returned for the user specified by the user id.\nThe API calls are returned in order of creation, with the most recently created API calls first."]
pub fn list_for_user(
    &self,
    id: String,
    limit: Option<u32>,
    page_token: Option<String>,
    sort_by: Option<crate::types::CreatedAtSortMode>,
) -> Result<crate::types::ApiCallWithPriceResultsPage> {
    todo!()
}

            }
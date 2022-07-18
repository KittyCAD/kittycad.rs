use crate::Client;
use anyhow::Result;
pub struct Users {
    pub client: Client,
}

impl Users {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Get your user.\n\nGet the user information for the authenticated user.\nAlternatively, you can also use the `/users/me` endpoint.\n\n```\n/// Get your user.\n/// \n/// Get the user information for the authenticated user.\n/// Alternatively, you can also use the `/users/me` endpoint.\nasync fn example_get_user_self() -> anyhow::Result<()> {\n    let result: crate::types::User = client.users().get_user_self().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n```"]
    pub async fn get_user_self<'a>(
        &'a self,
    ) -> Result<crate::types::User, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "user"),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    format_serde_error::SerdeError::new(text.to_string(), err),
                    status,
                )
                .into()
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Update your user.\n\nThis endpoint requires authentication by any KittyCAD user. It updates information about the authenticated user.\n\n```\n/// Update your user.\n/// \n/// This endpoint requires authentication by any KittyCAD user. It updates information about the authenticated user.\nasync fn example_update_user_self() -> anyhow::Result<()> {\n    let result: crate::types::User = client\n        .users()\n        .update_user_self(&crate::types::UpdateUser {\n            company: \"tl\".to_string(),\n            discord: \"oqko\".to_string(),\n            first_name: \"\".to_string(),\n            github: \"nlvcdwmk\".to_string(),\n            last_name: \"g\".to_string(),\n            phone: crate::types::phone_number::PhoneNumber::from_str(\"+1 503-463-1352\")?,\n        })\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n```"]
    pub async fn update_user_self<'a>(
        &'a self,
        body: &crate::types::UpdateUser,
    ) -> Result<crate::types::User, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::PUT,
            &format!("{}/{}", self.client.base_url, "user"),
        );
        req = req.bearer_auth(&self.client.token);
        req = req.json(body);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    format_serde_error::SerdeError::new(text.to_string(), err),
                    status,
                )
                .into()
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Delete your user.\n\nThis endpoint requires authentication by any KittyCAD user. It deletes the authenticated user from KittyCAD's database.\nThis call will only succeed if all invoices associated with the user have been paid in full and there is no outstanding balance.\n\n```\n/// Delete your user.\n/// \n/// This endpoint requires authentication by any KittyCAD user. It deletes the authenticated user from KittyCAD's database.\n/// This call will only succeed if all invoices associated with the user have been paid in full and there is no outstanding balance.\nasync fn example_delete_user_self() -> anyhow::Result<()> {\n    client.users().delete_user_self().await?;\n    Ok(())\n}\n\n```"]
    pub async fn delete_user_self<'a>(&'a self) -> Result<(), crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::DELETE,
            &format!("{}/{}", self.client.base_url, "user"),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            Ok(())
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Get extended information about your user.\n\nGet the user information for the authenticated user.\nAlternatively, you can also use the `/users-extended/me` endpoint.\n\n```\n/// Get extended information about your user.\n/// \n/// Get the user information for the authenticated user.\n/// Alternatively, you can also use the `/users-extended/me` endpoint.\nasync fn example_get_user_self_extended() -> anyhow::Result<()> {\n    let result: crate::types::ExtendedUser = client.users().get_user_self_extended().await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n```"]
    pub async fn get_user_self_extended<'a>(
        &'a self,
    ) -> Result<crate::types::ExtendedUser, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "user/extended"),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    format_serde_error::SerdeError::new(text.to_string(), err),
                    status,
                )
                .into()
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "List users.\n\nThis endpoint required authentication by a KittyCAD employee. The users are returned in order of creation, with the most recently created users first.\n\n```\n/// List users.\n/// \n/// This endpoint required authentication by a KittyCAD employee. The users are returned in order of creation, with the most recently created users first.\nasync fn example_list() -> anyhow::Result<()> {\n    let result: crate::types::UserResultsPage = client\n        .users()\n        .list(\n            Some(4 as u32),\n            Some(\"fwhpfqhj\".to_string()),\n            Some(crate::types::SortBy::CreatedAtAscending),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n\n/// - OR -\n\n/// Get a stream of results.\n///\n/// This allows you to paginate through all the items.\nasync fn example_list_stream() -> anyhow::Result<()> {\n    let stream = client.users().list_stream(\n        Some(4 as u32),\n        Some(crate::types::SortBy::CreatedAtAscending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err);\n            }\n        }\n    }\n\n    Ok(())\n}\n\n```"]
    pub async fn list<'a>(
        &'a self,
        limit: Option<u32>,
        page_token: Option<String>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> Result<crate::types::UserResultsPage, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "users"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        if let Some(p) = limit {
            query_params.push(("limit", format!("{}", p)));
        }

        if let Some(p) = page_token {
            query_params.push(("page_token", p));
        }

        if let Some(p) = sort_by {
            query_params.push(("sort_by", format!("{}", p)));
        }

        req = req.query(&query_params);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    format_serde_error::SerdeError::new(text.to_string(), err),
                    status,
                )
                .into()
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "List users.\n\nThis endpoint required authentication by a KittyCAD employee. The users are returned in order of creation, with the most recently created users first.\n\n```\n/// List users.\n/// \n/// This endpoint required authentication by a KittyCAD employee. The users are returned in order of creation, with the most recently created users first.\nasync fn example_list() -> anyhow::Result<()> {\n    let result: crate::types::UserResultsPage = client\n        .users()\n        .list(\n            Some(4 as u32),\n            Some(\"fwhpfqhj\".to_string()),\n            Some(crate::types::SortBy::CreatedAtAscending),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n\n/// - OR -\n\n/// Get a stream of results.\n///\n/// This allows you to paginate through all the items.\nasync fn example_list_stream() -> anyhow::Result<()> {\n    let stream = client.users().list_stream(\n        Some(4 as u32),\n        Some(crate::types::SortBy::CreatedAtAscending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err);\n            }\n        }\n    }\n\n    Ok(())\n}\n\n```"]
    pub fn list_stream<'a>(
        &'a self,
        limit: Option<u32>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> impl futures::Stream<Item = Result<crate::types::User, crate::types::error::Error>> + Unpin + '_
    {
        use crate::types::paginate::Pagination;
        use futures::{StreamExt, TryFutureExt, TryStreamExt};
        self.list(limit, None, sort_by)
            .map_ok(move |result| {
                let items = futures::stream::iter(result.items().into_iter().map(Ok));
                let next_pages =
                    futures::stream::try_unfold(result, move |new_result| async move {
                        if new_result.has_more_pages() {
                            async {
                                let mut req = self.client.client.request(
                                    http::Method::GET,
                                    &format!("{}/{}", self.client.base_url, "users"),
                                );
                                req = req.bearer_auth(&self.client.token);
                                let mut request = req.build()?;
                                request = new_result.next_page(request)?;
                                let resp = self.client.client.execute(request).await?;
                                let status = resp.status();
                                if status.is_success() {
                                    let text = resp.text().await.unwrap_or_default();
                                    serde_json::from_str(&text).map_err(|err| {
                                        crate::types::error::Error::from_serde_error(
                                            format_serde_error::SerdeError::new(
                                                text.to_string(),
                                                err,
                                            ),
                                            status,
                                        )
                                        .into()
                                    })
                                } else {
                                    Err(crate::types::error::Error::UnexpectedResponse(resp))
                                }
                            }
                            .map_ok(|result: crate::types::UserResultsPage| {
                                Some((
                                    futures::stream::iter(result.items().into_iter().map(Ok)),
                                    result,
                                ))
                            })
                            .await
                        } else {
                            Ok(None)
                        }
                    })
                    .try_flatten();
                items.chain(next_pages)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "List users with extended information.\n\nThis endpoint required authentication by a KittyCAD employee. The users are returned in order of creation, with the most recently created users first.\n\n```\n/// List users with extended information.\n/// \n/// This endpoint required authentication by a KittyCAD employee. The users are returned in order of creation, with the most recently created users first.\nasync fn example_list_extended() -> anyhow::Result<()> {\n    let result: crate::types::ExtendedUserResultsPage = client\n        .users()\n        .list_extended(\n            Some(4 as u32),\n            Some(\"r\".to_string()),\n            Some(crate::types::SortBy::CreatedAtDescending),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n\n/// - OR -\n\n/// Get a stream of results.\n///\n/// This allows you to paginate through all the items.\nasync fn example_list_extended_stream() -> anyhow::Result<()> {\n    let stream = client.users().list_extended_stream(\n        Some(4 as u32),\n        Some(crate::types::SortBy::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err);\n            }\n        }\n    }\n\n    Ok(())\n}\n\n```"]
    pub async fn list_extended<'a>(
        &'a self,
        limit: Option<u32>,
        page_token: Option<String>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> Result<crate::types::ExtendedUserResultsPage, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!("{}/{}", self.client.base_url, "users-extended"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = Vec::new();
        if let Some(p) = limit {
            query_params.push(("limit", format!("{}", p)));
        }

        if let Some(p) = page_token {
            query_params.push(("page_token", p));
        }

        if let Some(p) = sort_by {
            query_params.push(("sort_by", format!("{}", p)));
        }

        req = req.query(&query_params);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    format_serde_error::SerdeError::new(text.to_string(), err),
                    status,
                )
                .into()
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "List users with extended information.\n\nThis endpoint required authentication by a KittyCAD employee. The users are returned in order of creation, with the most recently created users first.\n\n```\n/// List users with extended information.\n/// \n/// This endpoint required authentication by a KittyCAD employee. The users are returned in order of creation, with the most recently created users first.\nasync fn example_list_extended() -> anyhow::Result<()> {\n    let result: crate::types::ExtendedUserResultsPage = client\n        .users()\n        .list_extended(\n            Some(4 as u32),\n            Some(\"r\".to_string()),\n            Some(crate::types::SortBy::CreatedAtDescending),\n        )\n        .await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n\n/// - OR -\n\n/// Get a stream of results.\n///\n/// This allows you to paginate through all the items.\nasync fn example_list_extended_stream() -> anyhow::Result<()> {\n    let stream = client.users().list_extended_stream(\n        Some(4 as u32),\n        Some(crate::types::SortBy::CreatedAtDescending),\n    );\n    loop {\n        match stream.try_next().await {\n            Ok(Some(item)) => {\n                println!(\"{:?}\", item);\n            }\n            Ok(None) => {\n                break;\n            }\n            Err(err) => {\n                return Err(err);\n            }\n        }\n    }\n\n    Ok(())\n}\n\n```"]
    pub fn list_extended_stream<'a>(
        &'a self,
        limit: Option<u32>,
        sort_by: Option<crate::types::CreatedAtSortMode>,
    ) -> impl futures::Stream<Item = Result<crate::types::ExtendedUser, crate::types::error::Error>>
           + Unpin
           + '_ {
        use crate::types::paginate::Pagination;
        use futures::{StreamExt, TryFutureExt, TryStreamExt};
        self.list_extended(limit, None, sort_by)
            .map_ok(move |result| {
                let items = futures::stream::iter(result.items().into_iter().map(Ok));
                let next_pages =
                    futures::stream::try_unfold(result, move |new_result| async move {
                        if new_result.has_more_pages() {
                            async {
                                let mut req = self.client.client.request(
                                    http::Method::GET,
                                    &format!("{}/{}", self.client.base_url, "users-extended"),
                                );
                                req = req.bearer_auth(&self.client.token);
                                let mut request = req.build()?;
                                request = new_result.next_page(request)?;
                                let resp = self.client.client.execute(request).await?;
                                let status = resp.status();
                                if status.is_success() {
                                    let text = resp.text().await.unwrap_or_default();
                                    serde_json::from_str(&text).map_err(|err| {
                                        crate::types::error::Error::from_serde_error(
                                            format_serde_error::SerdeError::new(
                                                text.to_string(),
                                                err,
                                            ),
                                            status,
                                        )
                                        .into()
                                    })
                                } else {
                                    Err(crate::types::error::Error::UnexpectedResponse(resp))
                                }
                            }
                            .map_ok(|result: crate::types::ExtendedUserResultsPage| {
                                Some((
                                    futures::stream::iter(result.items().into_iter().map(Ok)),
                                    result,
                                ))
                            })
                            .await
                        } else {
                            Ok(None)
                        }
                    })
                    .try_flatten();
                items.chain(next_pages)
            })
            .try_flatten_stream()
            .boxed()
    }

    #[doc = "Get extended information about a user.\n\nTo get information about yourself, use `/users-extended/me` as the endpoint. By doing so you will get the user information for the authenticated user.\nAlternatively, to get information about the authenticated user, use `/user/extended` endpoint.\nTo get information about any KittyCAD user, you must be a KittyCAD employee.\n\n```\n/// Get extended information about a user.\n/// \n/// To get information about yourself, use `/users-extended/me` as the endpoint. By doing so you will get the user information for the authenticated user.\n/// Alternatively, to get information about the authenticated user, use `/user/extended` endpoint.\n/// To get information about any KittyCAD user, you must be a KittyCAD employee.\nasync fn example_get_user_extended() -> anyhow::Result<()> {\n    let result: crate::types::ExtendedUser =\n        client.users().get_user_extended(\"\".to_string()).await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n```"]
    pub async fn get_user_extended<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<crate::types::ExtendedUser, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "users-extended/{id}".replace("{id}", &id)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    format_serde_error::SerdeError::new(text.to_string(), err),
                    status,
                )
                .into()
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }

    #[doc = "Get a user.\n\nTo get information about yourself, use `/users/me` as the endpoint. By doing so you will get the user information for the authenticated user.\nAlternatively, to get information about the authenticated user, use `/user` endpoint.\nTo get information about any KittyCAD user, you must be a KittyCAD employee.\n\n```\n/// Get a user.\n/// \n/// To get information about yourself, use `/users/me` as the endpoint. By doing so you will get the user information for the authenticated user.\n/// Alternatively, to get information about the authenticated user, use `/user` endpoint.\n/// To get information about any KittyCAD user, you must be a KittyCAD employee.\nasync fn example_get_user() -> anyhow::Result<()> {\n    let result: crate::types::User = client.users().get_user(\"oujsl\".to_string()).await?;\n    println!(\"{:?}\", result);\n    Ok(())\n}\n\n```"]
    pub async fn get_user<'a>(
        &'a self,
        id: &'a str,
    ) -> Result<crate::types::User, crate::types::error::Error> {
        let mut req = self.client.client.request(
            http::Method::GET,
            &format!(
                "{}/{}",
                self.client.base_url,
                "users/{id}".replace("{id}", &id)
            ),
        );
        req = req.bearer_auth(&self.client.token);
        let resp = req.send().await?;
        let status = resp.status();
        if status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            serde_json::from_str(&text).map_err(|err| {
                crate::types::error::Error::from_serde_error(
                    format_serde_error::SerdeError::new(text.to_string(), err),
                    status,
                )
                .into()
            })
        } else {
            Err(crate::types::error::Error::UnexpectedResponse(resp))
        }
    }
}

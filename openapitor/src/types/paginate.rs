//! Utility functions used for pagination.
#![allow(clippy::result_large_err)]

use anyhow::Result;

/// A trait for types that allow pagination.
pub trait Pagination {
    /// The item that is paginated.
    type Item: serde::de::DeserializeOwned;

    /// Returns true if the response has more pages.
    fn has_more_pages(&self) -> bool;

    /// Returns the next page token.
    fn next_page_token(&self) -> Option<String>;

    /// Modify a request to get the next page.
    fn next_page(
        &self,
        req: reqwest::Request,
    ) -> Result<reqwest::Request, crate::types::error::Error>;

    /// Modify a request to get the next page using the operation's page parameter.
    fn next_page_with_param(
        &self,
        req: reqwest::Request,
        _page_param: &str,
    ) -> Result<reqwest::Request, crate::types::error::Error> {
        self.next_page(req)
    }

    /// Get the items from a page.
    fn items(&self) -> Vec<Self::Item>;
}

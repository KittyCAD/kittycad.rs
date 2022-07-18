//! Utility functions used for pagination.

use anyhow::Result;

/// A trait for types that allow pagination.
pub trait Pagination {
    /// The item that is paginated.
    type Item: serde::de::DeserializeOwned;

    /// Returns true if the response has more pages.
    fn has_more_pages(&self) -> Result<bool>;

    /// Modify a request to get the next page.
    fn next_page(&self, req: reqwest::Request) -> Result<reqwest::Request>;

    /// Get the items from a page.
    fn items(&self) -> Vec<Self::Item>;
}

use serde::{Deserialize, Serialize};

/// A single page of a paginated list of an object.
#[derive(Debug, Deserialize, Serialize)]
pub struct List<T> {
    pub data: Vec<T>,
    pub has_more: bool,
    pub total_count: Option<u64>,
    pub url: String,
}

impl<T> Default for List<T> {
    fn default() -> Self {
        List {
            data: Vec::new(),
            has_more: false,
            total_count: None,
            url: String::new(),
        }
    }
}

impl<T: Clone> Clone for List<T> {
    fn clone(&self) -> Self {
        List {
            data: self.data.clone(),
            has_more: self.has_more,
            total_count: self.total_count,
            url: self.url.clone(),
        }
    }
}

impl<T> List<T> {
    pub fn paginate<P>(self, params: P) -> ListPaginator<T, P> {
        ListPaginator { page: self, params }
    }
}

impl<
        T: Paginate + DeserializeOwned + Send + Sync + 'static + Clone + std::fmt::Debug,
        P: Clone + Serialize + Send + 'static + std::fmt::Debug,
    > ListPaginator<T, P>
where
    P: Paginable<O = T>,
{
    /// Repeatedly queries Stripe for more data until all elements in list are fetched, using
    /// Stripe's default page size.
    ///
    /// Requires `feature = "blocking"`.
    #[cfg(feature = "blocking")]
    pub fn get_all(self, client: &Client) -> Response<Vec<T>> {
        let mut data = Vec::with_capacity(self.page.total_count.unwrap_or(0) as usize);
        let mut paginator = self;
        loop {
            if !paginator.page.has_more {
                data.extend(paginator.page.data.into_iter());
                break;
            }
            let next_paginator = paginator.next(client)?;
            data.extend(paginator.page.data.into_iter());
            paginator = next_paginator
        }
        Ok(data)
    }

    /// Get all values in this List, consuming self and lazily paginating until all values are fetched.
    ///
    /// This function repeatedly queries Stripe for more data until all elements in list are fetched, using
    /// the page size specified in params, or Stripe's default page size if none is specified.
    ///
    /// ```no_run
    /// # use stripe::{Customer, ListCustomers, StripeError, Client};
    /// # use futures_util::TryStreamExt;
    /// # fn main() {
    /// # tokio_test::block_on(run());
    /// # }
    /// # async fn run() -> Result<(), StripeError> {
    /// # let client = Client::new("sk_test_123");
    /// # let params = ListCustomers { ..Default::default() };
    ///
    /// let list = Customer::list(&client, &params).await.unwrap().paginate(params);
    /// let mut stream = list.stream(&client);
    ///
    /// // take a value out from the stream
    /// if let Some(val) = stream.try_next().await? {
    ///     println!("GOT = {:?}", val);
    /// }
    ///
    /// // alternatively, you can use stream combinators
    /// let all_values = stream.try_collect::<Vec<_>>().await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Requires `feature = ["async", "stream"]`.
    #[cfg(all(feature = "async", feature = "stream"))]
    pub fn stream(
        mut self,
        client: &Client,
    ) -> impl futures_util::Stream<Item = Result<T, StripeError>> + Unpin {
        // We are going to be popping items off the end of the list, so we need to reverse it.
        self.page.data.reverse();

        Box::pin(futures_util::stream::unfold(
            Some((self, client.clone())),
            Self::unfold_stream,
        ))
    }

    /// unfold a single item from the stream
    #[cfg(all(feature = "async", feature = "stream"))]
    async fn unfold_stream(
        state: Option<(Self, Client)>,
    ) -> Option<(Result<T, StripeError>, Option<(Self, Client)>)> {
        let (mut paginator, client) = state?; // If none, we sent the last item in the last iteration

        if paginator.page.data.len() > 1 {
            return Some((Ok(paginator.page.data.pop()?), Some((paginator, client))));
            // We have more data on this page
        }

        if !paginator.page.has_more {
            return Some((Ok(paginator.page.data.pop()?), None)); // Final value of the stream, no errors
        }

        match paginator.next(&client).await {
            Ok(mut next_paginator) => {
                let data = paginator.page.data.pop()?;
                next_paginator.page.data.reverse();

                // Yield last value of thimuts page, the next page (and client) becomes the state
                Some((Ok(data), Some((next_paginator, client))))
            }
            Err(e) => Some((Err(e), None)), // We ran into an error. The last value of the stream will be the error.
        }
    }

    /// Fetch an additional page of data from stripe.
    pub fn next(&self, client: &Client) -> Response<Self> {
        if let Some(last) = self.page.data.last() {
            if self.page.url.starts_with("/v1/") {
                let path = self.page.url.trim_start_matches("/v1/").to_string(); // the url we get back is prefixed

                // clone the params and set the cursor
                let params_next = {
                    let mut p = self.params.clone();
                    p.set_last(last.clone());
                    p
                };

                println!("next");
                let page = client.get_query(&path, &params_next);

                ListPaginator::create_paginator(page, params_next)
            } else {
                err(StripeError::UnsupportedVersion)
            }
        } else {
            ok(ListPaginator {
                page: List {
                    data: Vec::new(),
                    has_more: false,
                    total_count: self.page.total_count,
                    url: self.page.url.clone(),
                },
                params: self.params.clone(),
            })
        }
    }

    /// Pin a new future which maps the result inside the page future into
    /// a ListPaginator
    #[cfg(feature = "async")]
    fn create_paginator(page: Response<List<T>>, params: P) -> Response<Self> {
        use futures_util::FutureExt;
        Box::pin(page.map(|page| page.map(|page| ListPaginator { page, params })))
    }

    #[cfg(feature = "blocking")]
    fn create_paginator(page: Response<List<T>>, params: P) -> Response<Self> {
        page.map(|page| ListPaginator { page, params })
    }
}

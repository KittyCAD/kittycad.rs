use anyhow::Result;

use crate::Client;

pub struct Oauth2 {
    pub client: Client,
}

impl Oauth2 {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Oauth2 { client }
    }

    #[doc = "Start an OAuth 2.0 Device Authorization Grant.\n\nThis endpoint is designed to be accessed from an *unauthenticated* API client. It generates and records a `device_code` and `user_code` which must be verified and confirmed prior to a token being granted."]
    pub fn device_auth_request(&self) -> Result<()> {
        todo!()
    }

    #[doc = "Confirm an OAuth 2.0 Device Authorization Grant.\n\nThis endpoint is designed to be accessed by the user agent (browser), not the client requesting the token. So we do not actually return the token here; it will be returned in response to the poll on `/oauth2/device/token`."]
    pub fn device_auth_confirm(&self) -> Result<()> {
        todo!()
    }

    #[doc = "Request a device access token.\n\nThis endpoint should be polled by the client until the user code is verified and the grant is confirmed."]
    pub fn device_access_token(&self) -> Result<()> {
        todo!()
    }

    #[doc = "Verify an OAuth 2.0 Device Authorization Grant.\n\nThis endpoint should be accessed in a full user agent (e.g., a browser). If the user is not logged in, we redirect them to the login page and use the `callback_url` parameter to get them to the UI verification form upon logging in. If they are logged in, we redirect them to the UI verification form on the website."]
    pub fn device_auth_verify(&self) -> Result<()> {
        todo!()
    }

    #[doc = "Listen for callbacks for the OAuth 2.0 provider."]
    pub fn listen_provider_callback(&self) -> Result<()> {
        todo!()
    }

    #[doc = "Get the consent URL and other information for the OAuth 2.0 provider."]
    pub fn listen_provider_consent(&self) -> Result<crate::types::Oauth2ClientInfo> {
        todo!()
    }
}

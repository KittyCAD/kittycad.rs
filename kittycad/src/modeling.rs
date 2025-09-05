use anyhow::Result;

use crate::Client;
#[derive(Clone, Debug)]
pub struct Modeling {
    pub client: Client,
}

impl Modeling {
    #[doc(hidden)]
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    #[doc = "Open a websocket which accepts modeling commands.\n\nPass those commands to the \
             engine via websocket, and pass responses back to the client. Basically, this is a \
             websocket proxy between the frontend/client and the engine.\n\n**Parameters:**\n\n- \
             `api_call_id: Option<String>`: API Call ID for distributed tracing\n- `fps: \
             Option<u32>`: Frames per second of the video feed.\n- `pool: Option<String>`: An \
             optional identifier for a pool of engine instances. The 'default' pool is used when \
             none is specified.\n- `post_effect: Option<crate::types::PostEffectType>`: Engine \
             Post effects (such as SSAO)\n- `replay: Option<String>`: If given, when the session \
             ends, the modeling commands sent during the session will be written out to this \
             filename. For debugging.\n- `show_grid: Option<bool>`: If true, will show the grid at \
             the start of the session.\n- `unlocked_framerate: Option<bool>`: If true, engine will \
             render video frames as fast as it can.\n- `video_res_height: Option<u32>`: Height of \
             the video feed. Must be a multiple of 4.\n- `video_res_width: Option<u32>`: Width of \
             the video feed. Must be a multiple of 4.\n- `webrtc: Option<bool>`: If true, will \
             start a webrtc connection."]
    #[tracing::instrument]
    #[cfg(not(target_arch = "wasm32"))]
    pub async fn commands_ws<'a>(
        &'a self,
        api_call_id: Option<String>,
        fps: Option<u32>,
        pool: Option<String>,
        post_effect: Option<crate::types::PostEffectType>,
        replay: Option<String>,
        show_grid: Option<bool>,
        unlocked_framerate: Option<bool>,
        video_res_height: Option<u32>,
        video_res_width: Option<u32>,
        webrtc: Option<bool>,
    ) -> Result<(reqwest::Upgraded, http::HeaderMap), crate::types::error::Error> {
        let mut req = self.client.client_http1_only.request(
            http::Method::GET,
            format!("{}/{}", self.client.base_url, "ws/modeling/commands"),
        );
        req = req.bearer_auth(&self.client.token);
        let mut query_params = vec![];
        if let Some(p) = api_call_id {
            query_params.push(("api_call_id", p));
        }

        if let Some(p) = fps {
            query_params.push(("fps", format!("{p}")));
        }

        if let Some(p) = pool {
            query_params.push(("pool", p));
        }

        if let Some(p) = post_effect {
            query_params.push(("post_effect", format!("{p}")));
        }

        if let Some(p) = replay {
            query_params.push(("replay", p));
        }

        if let Some(p) = show_grid {
            query_params.push(("show_grid", format!("{p}")));
        }

        if let Some(p) = unlocked_framerate {
            query_params.push(("unlocked_framerate", format!("{p}")));
        }

        if let Some(p) = video_res_height {
            query_params.push(("video_res_height", format!("{p}")));
        }

        if let Some(p) = video_res_width {
            query_params.push(("video_res_width", format!("{p}")));
        }

        if let Some(p) = webrtc {
            query_params.push(("webrtc", format!("{p}")));
        }

        req = req.query(&query_params);
        req = req
            .header(reqwest::header::CONNECTION, "Upgrade")
            .header(reqwest::header::UPGRADE, "websocket")
            .header(reqwest::header::SEC_WEBSOCKET_VERSION, "13")
            .header(
                reqwest::header::SEC_WEBSOCKET_KEY,
                base64::Engine::encode(
                    &base64::engine::general_purpose::STANDARD,
                    rand::random::<[u8; 16]>(),
                ),
            );
        let resp = req.send().await?;
        if resp.status().is_client_error() || resp.status().is_server_error() {
            return Err(crate::types::error::Error::UnexpectedResponse(resp));
        }

        let headers = resp.headers().clone();
        let upgraded = resp
            .upgrade()
            .await
            .map_err(crate::types::error::Error::RequestError)?;
        Ok((upgraded, headers))
    }
}

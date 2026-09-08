#![cfg(feature = "requests")]

use std::{
    io::{Read, Write},
    net::TcpListener,
    time::Duration,
};

#[tokio::test]
async fn geometry_intent_is_explicit_and_video_options_remain_optional() {
    for geometry_only in [None, Some(false), Some(true)] {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap();
        let request = std::thread::spawn(move || {
            let (mut socket, _) = listener.accept().unwrap();
            socket
                .set_read_timeout(Some(Duration::from_secs(5)))
                .unwrap();
            let mut bytes = Vec::new();
            let mut chunk = [0; 1024];
            while !bytes.windows(4).any(|value| value == b"\r\n\r\n") {
                let len = socket.read(&mut chunk).unwrap();
                assert!(len > 0);
                bytes.extend_from_slice(&chunk[..len]);
            }
            socket
                .write_all(
                    b"HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                )
                .unwrap();
            String::from_utf8(bytes).unwrap()
        });
        let mut client = kittycad::Client::new("test-token");
        client.set_base_url(format!("http://{address}"));
        let result = client
            .modeling()
            .commands_ws(kittycad::modeling::CommandsWsParams {
                geometry_only,
                webrtc: Some(false),
                ..Default::default()
            })
            .await;
        assert!(result.is_err()); // The local capture server deliberately rejects the upgrade.
        let request = request.join().unwrap();
        let target = request
            .lines()
            .next()
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap();
        let url = url::Url::parse(&format!("http://localhost{target}")).unwrap();
        let params: std::collections::HashMap<_, _> = url.query_pairs().collect();
        assert_eq!(
            params.get("geometry_only").map(|v| v.as_ref()),
            geometry_only.map(|v| if v { "true" } else { "false" })
        );
        assert_eq!(params.get("webrtc").map(|v| v.as_ref()), Some("false"));
        for unused in ["video_res_width", "video_res_height", "post_effect"] {
            assert!(!params.contains_key(unused));
        }
    }
}

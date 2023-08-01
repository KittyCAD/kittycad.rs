use futures::TryStreamExt;
use pretty_assertions::assert_eq;
use tokio_tungstenite::tungstenite::Message as WsMsg;

use crate::types::{
    ExtendPath, ModelingCmd, ModelingCmdExtendPath, ModelingCmdReq, PathSegment, Point3D,
};

fn test_client() -> crate::Client {
    crate::Client::new_from_env()
}

#[tokio::test]
async fn test_create_file_conversion() {
    let client = test_client();
    let body = include_bytes!("../../assets/in_obj.obj");

    let conversion = client
        .file()
        .create_conversion(
            crate::types::FileExportFormat::Step,
            crate::types::FileImportFormat::Obj,
            &body.to_vec().into(),
        )
        .await
        .unwrap();

    assert!(conversion.output.is_some());

    assert_eq!(conversion.src_format, crate::types::FileImportFormat::Obj);
    assert_eq!(conversion.status, crate::types::ApiCallStatus::Completed);
}

#[tokio::test]
async fn test_create_file_volume() {
    let client = test_client();
    let body = include_bytes!("../../assets/in_obj.obj");

    let result = client
        .file()
        .create_volume(
            Some(crate::types::UnitVolume::M3),
            crate::types::FileImportFormat::Obj,
            &body.to_vec().into(),
        )
        .await
        .unwrap();

    assert_eq!(result.volume, Some(53.601147));

    assert_eq!(result.src_format, crate::types::FileImportFormat::Obj);
    assert_eq!(result.status, crate::types::ApiCallStatus::Completed);
}

#[tokio::test]
async fn test_get_status_of_async_operation() {
    let client = test_client();

    let _result = client
        .api_calls()
        .get_async_operation("23a9759f-ee9b-47de-9a55-deb1ed035793")
        .await
        .unwrap();
}

#[tokio::test]
async fn serialize_one_of() {
    let client = test_client();

    let result = client
        .api_calls()
        .get_async_operation("23a9759f-ee9b-47de-9a55-deb1ed035793")
        .await
        .unwrap();

    expectorate::assert_contents(
        "tests/one_of.json",
        &serde_json::to_string_pretty(&result).unwrap(),
    );
}

#[cfg(feature = "tabled")]
#[tokio::test]
async fn tabled_one_of() {
    let client = test_client();

    let result = client
        .api_calls()
        .get_async_operation("23a9759f-ee9b-47de-9a55-deb1ed035793")
        .await
        .unwrap();

    expectorate::assert_contents(
        "tests/tabled_one_of.txt",
        &tabled::Table::new(vec![result]).to_string(),
    );
}

#[tokio::test]
async fn test_stream() {
    let client = test_client();

    let limit = 2;
    let api_calls = client.api_calls();
    let mut stream = api_calls.user_list_stream(Some(limit), None);

    let mut ids: Vec<String> = Default::default();
    loop {
        match stream.try_next().await {
            Ok(Some(item)) => {
                // Make sure we are not repeating items.
                if !ids.contains(&item.id.to_string()) {
                    ids.push(item.id.to_string());
                }
                if ids.len() > (limit * 20) as usize {
                    break;
                }
            }
            Ok(None) => {
                break;
            }
            Err(err) => panic!("{}", err),
        }
    }
}

#[test]
fn test_empty_phone_number() {
    let user_info = crate::types::UpdateUser {
        first_name: Some("John".to_string()),
        last_name: Some("Doe".to_string()),
        phone: Default::default(),
        company: Some("Example Company".to_string()),
        github: Some("@example-company".to_string()),
        discord: Some("@example-company".to_string()),
    };

    assert_eq!(
        serde_json::to_string_pretty(&user_info).unwrap(),
        r#"{
  "company": "Example Company",
  "discord": "@example-company",
  "first_name": "John",
  "github": "@example-company",
  "last_name": "Doe"
}"#
    );

    let user_info_str = r#"{"first_name":"John","last_name":"Doe","company":"Example Company","github":"@example-company","discord":"@example-company"}"#;
    assert_eq!(
        serde_json::from_str::<crate::types::UpdateUser>(user_info_str).unwrap(),
        user_info
    );

    let billing_info = crate::types::BillingInfo {
        name: Some("John".to_string()),
        phone: Default::default(),
        address: None,
    };

    assert_eq!(
        serde_json::to_string_pretty(&billing_info).unwrap(),
        r#"{
  "name": "John"
}"#
    );

    let billing_info_str = r#"{"name":"John"}"#;
    assert_eq!(
        serde_json::from_str::<crate::types::BillingInfo>(billing_info_str).unwrap(),
        billing_info
    );
}

#[tokio::test]
async fn test_user_self() {
    let client = test_client();

    let _result = client.users().get_self().await.unwrap();
}

#[tokio::test]
async fn test_modeling_websocket() {
    use futures::{SinkExt, StreamExt};
    use uuid::Uuid;

    let client = test_client();

    let ws = match client.modeling().commands_ws().await {
        Ok(ws) => ws,
        Err(crate::types::error::Error::UnexpectedResponse(resp)) => {
            let txt = resp.text().await.unwrap();
            panic!("Failed to connect to modeling websocket: {}", txt);
        }
        err => panic!("Failed to connect to modeling websocket: {:?}", err),
    };
    let (mut write, mut read) = tokio_tungstenite::WebSocketStream::from_raw_socket(
        ws,
        tokio_tungstenite::tungstenite::protocol::Role::Client,
        None,
    )
    .await
    .split();

    // Start a path
    let path_id = Uuid::new_v4();
    write
        .send(WsMsg::Text(
            serde_json::to_string(&ModelingCmdReq {
                cmd: ModelingCmd::ModelingCmdStartPath(
                    crate::types::ModelingCmdStartPath::StartPath,
                ),
                cmd_id: path_id,
                file_id: Default::default(),
            })
            .unwrap(),
        ))
        .await
        .unwrap();

    // Draw the path in a triangle shape.
    let points = [
        Point3D {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        Point3D {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        Point3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    ];
    for point in points {
        write
            .send(WsMsg::Text(
                serde_json::to_string(&ModelingCmdReq {
                    cmd: ModelingCmd::ModelingCmdExtendPath(ModelingCmdExtendPath {
                        extend_path: ExtendPath {
                            path: path_id,
                            segment: PathSegment::Line { end: point },
                        },
                    }),
                    cmd_id: Uuid::new_v4(),
                    file_id: Default::default(),
                })
                .unwrap(),
            ))
            .await
            .unwrap();
    }

    // Finish sending
    drop(write);

    // Get Websocket messages from API server
    let num_modeling_cmds = 1;
    let mut text_resps = 0;
    while let Some(msg) = read.next().await {
        match msg.unwrap() {
            WsMsg::Text(resp) => {
                eprintln!("Got a websocket response: {resp}");
                text_resps += 1;
                if text_resps == num_modeling_cmds {
                    break;
                }
            }
            // MDN says that when you're writing a Websocket server, "You might also get a pong
            // without ever sending a ping; ignore this if it happens."
            // See <https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API/Writing_WebSocket_servers#pings_and_pongs_the_heartbeat_of_websockets>
            WsMsg::Pong(_) => {}
            other => {
                panic!("Unexpected websocket message from server: {}", other)
            }
        }
    }
}

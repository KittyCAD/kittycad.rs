use std::str::FromStr;

#[cfg(not(feature = "js"))]
use futures::TryStreamExt;
use pretty_assertions::assert_eq;
use tokio_tungstenite::tungstenite::Message as WsMsg;

use crate::types::{ModelingCmd, PathSegment, Point3D, WebSocketRequest::ModelingCmdReq};

#[test]
fn test_mlcopilot_server_message_serde_roundtrip() {
    use pretty_assertions::assert_eq;
    use serde_json::json;

    use crate::types::{MlCopilotServerMessage, MlToolResult, ReasoningMessage};

    // Delta
    let msg = MlCopilotServerMessage::Delta {
        delta: "hello".to_string(),
    };
    let expected = json!({
        "delta": { "delta": "hello" }
    });
    let ser = serde_json::to_value(&msg).unwrap();
    assert_eq!(ser, expected);
    let de: MlCopilotServerMessage = serde_json::from_value(expected.clone()).unwrap();
    assert_eq!(de, msg);

    // Tool output (TextToCad)
    let msg = MlCopilotServerMessage::ToolOutput {
        result: MlToolResult::TextToCad {
            error: None,
            outputs: None,
            project_name: Some("proj".into()),
            status_code: 0,
        },
    };
    let expected = json!({
        "tool_output": {
            "result": {
                "type": "text_to_cad",
                "status_code": 0,
                "project_name": "proj"
            }
        }
    });
    let ser = serde_json::to_value(&msg).unwrap();
    assert_eq!(ser, expected);
    let de: MlCopilotServerMessage = serde_json::from_value(expected.clone()).unwrap();
    assert_eq!(de, msg);

    // Error
    let msg = MlCopilotServerMessage::Error {
        detail: "boom".into(),
    };
    let expected = json!({
        "error": { "detail": "boom" }
    });
    let ser = serde_json::to_value(&msg).unwrap();
    assert_eq!(ser, expected);
    let de: MlCopilotServerMessage = serde_json::from_value(expected.clone()).unwrap();
    assert_eq!(de, msg);

    // Info
    let msg = MlCopilotServerMessage::Info {
        text: "note".into(),
    };
    let expected = json!({
        "info": { "text": "note" }
    });
    let ser = serde_json::to_value(&msg).unwrap();
    assert_eq!(ser, expected);
    let de: MlCopilotServerMessage = serde_json::from_value(expected.clone()).unwrap();
    assert_eq!(de, msg);

    // Reasoning (text)
    let msg = MlCopilotServerMessage::Reasoning(ReasoningMessage::Text {
        content: "thinking".into(),
    });
    let expected = json!({
        "reasoning": { "type": "text", "content": "thinking" }
    });
    let ser = serde_json::to_value(&msg).unwrap();
    assert_eq!(ser, expected);
    let de: MlCopilotServerMessage = serde_json::from_value(expected.clone()).unwrap();
    assert_eq!(de, msg);

    // End of stream (with whole_response)
    let msg = MlCopilotServerMessage::EndOfStream {
        whole_response: Some("done".into()),
        started_at: None,
        completed_at: None,
        conversation_id: None,
        id: None,
    };
    let expected = json!({
        "end_of_stream": { "whole_response": "done" }
    });
    let ser = serde_json::to_value(&msg).unwrap();
    assert_eq!(ser, expected);
    let de: MlCopilotServerMessage = serde_json::from_value(expected).unwrap();
    assert_eq!(de, msg);
}

fn test_client() -> crate::Client {
    crate::Client::new_from_env()
}

#[cfg(not(feature = "js"))]
#[tokio::test]
async fn test_list_org_members_stream() {
    let client = test_client();

    let orgs = client.orgs();
    let mut stream = orgs.list_members_stream(None, None, None);

    let mut members: Vec<crate::types::OrgMember> = Default::default();
    loop {
        match stream.try_next().await {
            Ok(Some(item)) => {
                members.push(item);
            }
            Ok(None) => {
                break;
            }
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

    assert!(!members.is_empty());
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

    assert!(conversion.outputs.is_some());
    assert!(!conversion.outputs.unwrap().is_empty());

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

    assert_eq!(result.volume.map(|x| (x * 1000000.0).round()), Some(0.0));

    assert_eq!(result.src_format, crate::types::FileImportFormat::Obj);
    assert_eq!(result.status, crate::types::ApiCallStatus::Completed);
}

#[tokio::test]
async fn test_get_status_of_async_operation() {
    let client = test_client();

    let _result = client
        .api_calls()
        .get_async_operation(uuid::Uuid::from_str("23a9759f-ee9b-47de-9a55-deb1ed035793").unwrap())
        .await
        .unwrap();
}

#[tokio::test]
async fn serialize_one_of() {
    let client = test_client();

    let result = client
        .api_calls()
        .get_async_operation(uuid::Uuid::from_str("23a9759f-ee9b-47de-9a55-deb1ed035793").unwrap())
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
        .get_async_operation(uuid::Uuid::from_str("23a9759f-ee9b-47de-9a55-deb1ed035793").unwrap())
        .await
        .unwrap();

    expectorate::assert_contents(
        "tests/tabled_one_of.txt",
        &tabled::Table::new(vec![result]).to_string(),
    );
}

#[tokio::test]
#[cfg(not(feature = "js"))]
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
        image: "".to_string(),
        is_onboarded: Default::default(),
    };

    assert_eq!(
        serde_json::to_string_pretty(&user_info).unwrap(),
        r#"{
  "company": "Example Company",
  "discord": "@example-company",
  "first_name": "John",
  "github": "@example-company",
  "image": "",
  "last_name": "Doe"
}"#
    );

    let user_info_str = r#"{"first_name":"John","last_name":"Doe","company":"Example Company","github":"@example-company","discord":"@example-company","image": ""}"#;
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

    let ws = match client
        .modeling()
        .commands_ws(crate::modeling::CommandsWsParams::default())
        .await
    {
        Ok((ws, _headers)) => ws,
        Err(crate::types::error::Error::UnexpectedResponse(resp)) => {
            let txt = resp.text().await.unwrap();
            panic!("Failed to connect to modeling websocket: {txt}");
        }
        err => panic!("Failed to connect to modeling websocket: {err:?}"),
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
                cmd: ModelingCmd::StartPath {},
                cmd_id: path_id,
            })
            .unwrap()
            .into(),
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
                    cmd: ModelingCmd::ExtendPath {
                        path: path_id,
                        segment: PathSegment::Line {
                            end: point,
                            relative: false,
                        },
                        label: None,
                    },
                    cmd_id: Uuid::new_v4(),
                })
                .unwrap()
                .into(),
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
                panic!("Unexpected websocket message from server: {other}")
            }
        }
    }
}

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

fn test_client(test_name: &str) -> Option<crate::Client> {
    let has_token = std::env::var("KITTYCAD_API_TOKEN")
        .ok()
        .filter(|token| !token.is_empty())
        .or_else(|| {
            std::env::var("ZOO_API_TOKEN")
                .ok()
                .filter(|token| !token.is_empty())
        })
        .is_some();

    if !has_token {
        eprintln!("skipping {test_name}: set KITTYCAD_API_TOKEN or ZOO_API_TOKEN");
        return None;
    }

    Some(crate::Client::new_from_env())
}

fn one_of_fixture() -> crate::types::AsyncApiCallOutput {
    serde_json::from_str(include_str!("../tests/one_of.json")).unwrap()
}

fn kittycad_coord_system() -> crate::types::System {
    crate::types::System {
        forward: crate::types::AxisDirectionPair {
            axis: crate::types::Axis::Y,
            direction: crate::types::Direction::Negative,
        },
        up: crate::types::AxisDirectionPair {
            axis: crate::types::Axis::Z,
            direction: crate::types::Direction::Positive,
        },
    }
}

async fn create_async_file_conversion(client: &crate::Client) -> crate::types::FileConversion {
    let body = include_bytes!("../../assets/in_obj.obj");
    let conversion_params = crate::types::ConversionParams {
        output_format: crate::types::OutputFormat3D::Step {
            coords: Some(kittycad_coord_system()),
            created: None,
            presentation: None,
            units: None,
        },
        src_format: crate::types::InputFormat3D::Obj {
            coords: kittycad_coord_system(),
            units: crate::types::UnitLength::Mm,
        },
    };

    let mut form = reqwest::multipart::Form::new();
    let mut json_part =
        reqwest::multipart::Part::text(serde_json::to_string(&conversion_params).unwrap());
    json_part = json_part.file_name("body.json");
    json_part = json_part.mime_str("application/json").unwrap();
    form = form.part("body", json_part);

    let mut file_part = reqwest::multipart::Part::bytes(body.to_vec());
    file_part = file_part.file_name("in_obj.obj");
    file_part = file_part.mime_str("application/octet-stream").unwrap();
    form = form.part("file", file_part);

    let response = client
        .client
        .request(
            http::Method::POST,
            format!("{}/{}", client.base_url, "file/conversion"),
        )
        .bearer_auth(&client.token)
        .header(reqwest::header::CACHE_CONTROL, "no-cache")
        .multipart(form)
        .send()
        .await
        .unwrap();

    let status = response.status();
    let text = response.text().await.unwrap_or_default();
    if !status.is_success() {
        panic!("create_async_file_conversion failed with status {status}: {text}");
    }

    serde_json::from_str(&text).unwrap()
}

#[cfg(not(feature = "js"))]
#[tokio::test]
async fn test_list_org_members_stream() {
    let Some(client) = test_client("test_list_org_members_stream") else {
        return;
    };

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
    let Some(client) = test_client("test_create_file_conversion") else {
        return;
    };
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
    let Some(client) = test_client("test_create_file_volume") else {
        return;
    };
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
    let Some(client) = test_client("test_get_status_of_async_operation") else {
        return;
    };
    let conversion = create_async_file_conversion(&client).await;

    let result = client
        .api_calls()
        .get_async_operation(conversion.id)
        .await
        .unwrap();

    match result {
        crate::types::AsyncApiCallOutput::FileConversion { id, .. } => {
            assert_eq!(id, conversion.id);
        }
        other => panic!("expected file_conversion result, got {other:?}"),
    }
}

#[tokio::test]
async fn serialize_one_of() {
    let result = one_of_fixture();

    expectorate::assert_contents(
        "tests/one_of.json",
        &serde_json::to_string_pretty(&result).unwrap(),
    );
}

#[cfg(feature = "tabled")]
#[tokio::test]
async fn tabled_one_of() {
    let result = one_of_fixture();

    expectorate::assert_contents(
        "tests/tabled_one_of.txt",
        &tabled::Table::new(vec![result]).to_string(),
    );
}

#[tokio::test]
#[cfg(not(feature = "js"))]
async fn test_stream() {
    let Some(client) = test_client("test_stream") else {
        return;
    };

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

#[tokio::test]
#[cfg(not(feature = "js"))]
async fn test_list_dataset_conversions_stream_preserves_query_params() {
    let dataset_id = uuid::Uuid::new_v4();
    let conversion_id_1 = uuid::Uuid::new_v4();
    let conversion_id_2 = uuid::Uuid::new_v4();
    let (base_url, requests, server) =
        dataset_conversions_test_server(dataset_id, conversion_id_1, conversion_id_2);

    let mut client = crate::Client::new("test-token");
    client.set_base_url(base_url);

    let orgs = client.orgs();
    let mut stream = orgs.list_dataset_conversions_stream(
        Some("status=success".to_string()),
        dataset_id,
        Some(1),
        Some(crate::types::ConversionSortMode::StatusDescending),
    );

    let mut ids = Vec::new();
    while let Some(item) = stream.try_next().await.unwrap() {
        ids.push(item.id);
    }

    assert_eq!(ids, vec![conversion_id_1, conversion_id_2]);

    let first = requests
        .recv_timeout(std::time::Duration::from_secs(5))
        .unwrap();
    let second = requests
        .recv_timeout(std::time::Duration::from_secs(5))
        .unwrap();
    server.join().unwrap();

    let first_url = request_url(&first);
    let second_url = request_url(&second);
    let expected_path = format!("/org/datasets/{dataset_id}/conversions");
    assert_eq!(first_url.path(), expected_path);
    assert_eq!(second_url.path(), expected_path);

    assert_query_pair(&first_url, "filter", "status=success");
    assert_query_pair(&first_url, "limit", "1");
    assert_query_pair(&first_url, "sort_by", "status_descending");
    assert_no_query_pair(&first_url, "page_token");
    assert_no_query_pair(&first_url, "next_page");

    assert_query_pair(&second_url, "filter", "status=success");
    assert_query_pair(&second_url, "limit", "1");
    assert_query_pair(&second_url, "sort_by", "status_descending");
    assert_query_pair(&second_url, "page_token", "token-1");
    assert_no_query_pair(&second_url, "next_page");
}

#[cfg(not(feature = "js"))]
fn dataset_conversions_test_server(
    dataset_id: uuid::Uuid,
    conversion_id_1: uuid::Uuid,
    conversion_id_2: uuid::Uuid,
) -> (
    String,
    std::sync::mpsc::Receiver<String>,
    std::thread::JoinHandle<()>,
) {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let base_url = format!("http://{}", listener.local_addr().unwrap());
    let (requests_tx, requests_rx) = std::sync::mpsc::channel();

    let server = std::thread::spawn(move || {
        let pages = [
            dataset_conversion_page(dataset_id, conversion_id_1, Some("token-1")),
            dataset_conversion_page(dataset_id, conversion_id_2, None),
        ];

        for body in pages {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(std::time::Duration::from_secs(5)))
                .unwrap();

            let mut buffer = [0; 4096];
            let bytes_read = std::io::Read::read(&mut stream, &mut buffer).unwrap();
            let request = String::from_utf8_lossy(&buffer[..bytes_read]);
            let path = request
                .lines()
                .next()
                .and_then(|line| line.split_whitespace().nth(1))
                .unwrap()
                .to_string();
            requests_tx.send(path).unwrap();

            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: \
                 {}\r\nconnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            std::io::Write::write_all(&mut stream, response.as_bytes()).unwrap();
        }
    });

    (base_url, requests_rx, server)
}

#[cfg(not(feature = "js"))]
fn dataset_conversion_page(
    dataset_id: uuid::Uuid,
    conversion_id: uuid::Uuid,
    next_page: Option<&str>,
) -> String {
    serde_json::json!({
        "items": [{
            "created_at": "2026-05-07T00:00:00Z",
            "dataset_id": dataset_id,
            "file_etag": "etag",
            "file_path": "parts/bracket.step",
            "file_size": 42,
            "id": conversion_id,
            "manual_kcl_override_active": false,
            "phase": "completed",
            "status": "success",
            "updated_at": "2026-05-07T00:00:00Z"
        }],
        "next_page": next_page
    })
    .to_string()
}

#[cfg(not(feature = "js"))]
fn request_url(path: &str) -> url::Url {
    url::Url::parse(&format!("http://localhost{path}")).unwrap()
}

#[cfg(not(feature = "js"))]
fn assert_query_pair(url: &url::Url, name: &str, value: &str) {
    let pairs = url.query_pairs().collect::<Vec<_>>();
    assert!(
        pairs.iter().any(|(k, v)| k == name && v == value),
        "missing query pair {name}={value}, got {pairs:?}"
    );
}

#[cfg(not(feature = "js"))]
fn assert_no_query_pair(url: &url::Url, name: &str) {
    let pairs = url.query_pairs().collect::<Vec<_>>();
    assert!(
        pairs.iter().all(|(k, _)| k != name),
        "unexpected query pair {name}, got {pairs:?}"
    );
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
        username: None,
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
    let Some(client) = test_client("test_user_self") else {
        return;
    };

    let _result = client.users().get_self().await.unwrap();
}

#[tokio::test]
async fn test_modeling_websocket() {
    use futures::{SinkExt, StreamExt};
    use uuid::Uuid;

    let Some(client) = test_client("test_modeling_websocket") else {
        return;
    };

    let ws = match client
        .modeling()
        .commands_ws(crate::modeling::CommandsWsParams::default())
        .await
    {
        Ok((ws, _headers)) => ws,
        Err(crate::types::error::Error::UnexpectedResponse { body, .. }) => {
            panic!("Failed to connect to modeling websocket: {body}");
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

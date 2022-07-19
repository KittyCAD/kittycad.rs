use futures::TryStreamExt;
use pretty_assertions::assert_eq;

fn test_client() -> crate::Client {
    crate::Client::new_from_env()
}

#[tokio::test]
async fn test_create_file_conversion() {
    let client = test_client();
    let body = include_bytes!("../assets/in_obj.obj");

    let conversion = client
        .file()
        .create_conversion(
            crate::types::FileOutputFormat::Step,
            crate::types::FileSourceFormat::Obj,
            &body.to_vec().into(),
        )
        .await
        .unwrap();

    assert!(conversion.output.is_some());

    assert_eq!(conversion.src_format, crate::types::FileSourceFormat::Obj);
    assert_eq!(conversion.status, crate::types::ApiCallStatus::Completed);
}

#[tokio::test]
async fn test_create_file_conversion_with_base64_helper() {
    let client = test_client();
    let body = include_bytes!("../assets/in_obj.obj");

    let conversion = client
        .file()
        .create_conversion(
            crate::types::FileOutputFormat::Step,
            crate::types::FileSourceFormat::Obj,
            &body.to_vec().into(),
        )
        .await
        .unwrap();

    assert!(conversion.output.is_some());
    if let Some(output) = conversion.output {
        assert!(!output.is_empty());
    }

    assert_eq!(conversion.src_format, crate::types::FileSourceFormat::Obj);
    assert_eq!(conversion.status, crate::types::ApiCallStatus::Completed);
}

#[tokio::test]
async fn test_create_file_volume() {
    let client = test_client();
    let body = include_bytes!("../assets/in_obj.obj");

    let result = client
        .file()
        .create_volume(crate::types::FileSourceFormat::Obj, &body.to_vec().into())
        .await
        .unwrap();

    assert_eq!(result.volume, Some(48.800293));

    assert_eq!(result.src_format, crate::types::FileSourceFormat::Obj);
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
    let mut stream = api_calls.list_stream(Some(limit), None);

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
            Err(err) => std::panic::panic_any(err),
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

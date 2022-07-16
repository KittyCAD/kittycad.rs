use pretty_assertions::assert_eq;

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
    let body = include_bytes!("../../assets/in_obj.obj");

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
    let body = include_bytes!("../../assets/in_obj.obj");

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
        .get_async_operation("23a9759f-ee9b-47de-9a55-deb1ed035793".to_string())
        .await
        .unwrap();
}

#[tokio::test]
async fn serialize_one_of() {
    let client = test_client();

    let result = client
        .api_calls()
        .get_async_operation("23a9759f-ee9b-47de-9a55-deb1ed035793".to_string())
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
        .get_async_operation("23a9759f-ee9b-47de-9a55-deb1ed035793".to_string())
        .await
        .unwrap();

    expectorate::assert_contents(
        "tests/one_of_tabled.txt",
        &tabled::Table::new(vec![result]).to_string(),
    );
}

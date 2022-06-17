use pretty_assertions::assert_eq;

use crate::traits::Base64Ops;

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
            body.to_vec(),
        )
        .await
        .unwrap();

    assert!(!conversion.output.is_empty());

    assert_eq!(conversion.src_format, crate::types::FileSourceFormat::Obj);
    assert_eq!(conversion.status, crate::types::ApiCallStatus::Completed);
}

#[tokio::test]
async fn test_create_file_conversion_with_base64_helper() {
    let client = test_client();
    let body = include_bytes!("../../assets/in_obj.obj");

    let (conversion, output) = client
        .file()
        .create_conversion_with_decode(
            crate::types::FileOutputFormat::Step,
            crate::types::FileSourceFormat::Obj,
            body.to_vec(),
        )
        .await
        .unwrap();

    assert!(!conversion.output.is_empty());
    assert!(!output.is_empty());

    assert_eq!(conversion.src_format, crate::types::FileSourceFormat::Obj);
    assert_eq!(conversion.status, crate::types::ApiCallStatus::Completed);
}

#[tokio::test]
async fn test_create_file_volume() {
    let client = test_client();
    let body = include_bytes!("../../assets/in_obj.obj");

    let result = client
        .file()
        .create_volume(crate::types::FileSourceFormat::Obj, body.to_vec())
        .await
        .unwrap();

    assert_eq!(result.volume, 48.800293);

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

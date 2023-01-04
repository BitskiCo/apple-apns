use apple_apns::*;
use serde_json::json;
use tokio::test;
use wiremock::{
    matchers::{body_json, header, method, path},
    Mock, MockServer, ResponseTemplate,
};

const USER_AGENT: &str = "test/1.0.0";
const DEVICE_TOKEN: &str = "a863a50a904a4bb79380aae1e6c80b4dad25fcf8552848599d979b020aece5ae";
const APS_ID: &str = "4d947500-498e-4524-8aa8-7220c4e65d75";
const TOPIC: &str = "com.example.myapp";

fn create_apns_client(mock_server_uri: &str) -> Client {
    ClientBuilder {
        endpoint: Endpoint::Custom(format!("{mock_server_uri}/3/device/").parse().unwrap()),
        user_agent: USER_AGENT,
        ..Default::default()
    }
    .build()
    .unwrap()
}

#[test]
async fn client() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path(format!("/3/device/{DEVICE_TOKEN}")))
        .and(header("user-agent", USER_AGENT))
        .and(header("apns-push-type", "alert"))
        .and(header("apns-id", APS_ID))
        .and(header("apns-topic", TOPIC))
        .and(header("content-type", "application/json"))
        .and(body_json(json!({
            "aps": {
                "alert": {
                    "title": "You've Got Mail 🎉",
                    "body": "Hello World!",
                },
            },
        })))
        .respond_with(ResponseTemplate::new(200).insert_header("apns-id", APS_ID))
        .mount(&mock_server)
        .await;

    let client = create_apns_client(&mock_server.uri());

    let request = Request::<()> {
        device_token: DEVICE_TOKEN.into(),
        id: Some(APS_ID.parse().unwrap()),
        topic: Some(TOPIC.into()),
        alert: Some(Alert {
            title: Some("You've Got Mail 🎉".into()),
            body: Some("Hello World!".into()),
            ..Default::default()
        }),
        ..Default::default()
    };

    let aps_id = client.post(request).await;

    drop(mock_server);

    assert_eq!(APS_ID, aps_id.unwrap().hyphenated().to_string());
}

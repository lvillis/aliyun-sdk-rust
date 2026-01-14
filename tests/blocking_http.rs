#![cfg(feature = "blocking")]

use alibabacloud::{Auth, BlockingClient};
use http::StatusCode;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path, query_param},
};

const STS_IDENTITY_BODY: &str = r#"{"IdentityType":"Account","RequestId":"req","AccountId":"1","PrincipalId":"p","UserId":"u","Arn":"arn","RoleId":null}"#;

#[tokio::test(flavor = "current_thread")]
async fn sts_get_caller_identity_hits_configured_endpoint() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/"))
        .and(query_param("Action", "GetCallerIdentity"))
        .and(query_param("Version", "2015-04-01"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "application/json")
                .set_body_raw(STS_IDENTITY_BODY, "application/json"),
        )
        .mount(&server)
        .await;

    let client = BlockingClient::builder()
        .auth(Auth::access_key("id", "secret"))
        .sts_endpoint(server.uri())
        .build()
        .unwrap();

    let identity = tokio::task::spawn_blocking({
        let client = client.clone();
        move || client.sts().get_caller_identity()
    })
    .await
    .expect("blocking task join")
    .unwrap();
    assert_eq!(identity.request_id, "req");

    let requests = server.received_requests().await.expect("received requests");
    assert_eq!(requests.len(), 1);
    let request = &requests[0];
    assert_eq!(request.method.as_str(), "GET");
    assert_eq!(request.url.path(), "/");

    let query = request.url.query().expect("query string");
    assert!(query.contains("Action=GetCallerIdentity"));
    assert!(query.contains("Version=2015-04-01"));
    assert!(query.contains("Format=JSON"));
    assert!(query.contains("AccessKeyId=id"));
    assert!(query.contains("SignatureNonce="));
    assert!(query.contains("Signature="));

    let user_agent = request
        .headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();
    assert!(user_agent.starts_with("alibabacloud/"));
}

#[tokio::test(flavor = "current_thread")]
async fn http_error_body_is_redacted() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/"))
        .and(query_param("Action", "GetCallerIdentity"))
        .and(query_param("Version", "2015-04-01"))
        .respond_with(
            ResponseTemplate::new(403)
                .insert_header("content-type", "application/json")
                .insert_header("x-acs-request-id", "header-request-id")
                .set_body_raw(
                    r#"{"Code":"SignatureDoesNotMatch","Message":"bad signature","RequestId":"body-request-id","AccessKeySecret":"supersecret"}"#,
                    "application/json",
                ),
        )
        .mount(&server)
        .await;

    let client = BlockingClient::builder()
        .auth(Auth::access_key("id", "secret"))
        .sts_endpoint(server.uri())
        .build()
        .unwrap();

    let err = tokio::task::spawn_blocking({
        let client = client.clone();
        move || client.sts().get_caller_identity()
    })
    .await
    .expect("blocking task join")
    .unwrap_err();
    assert!(err.is_auth_error());
    assert_eq!(err.status(), Some(StatusCode::FORBIDDEN));
    assert_eq!(err.request_id(), Some("header-request-id"));

    let snippet = err.body_snippet().unwrap_or_default();
    assert!(!snippet.contains("supersecret"));
}

#[tokio::test]
async fn test_health_check() {
    spawn_server();

    let response = reqwest::get("http://127.0.0.1:8080/health_check")
        .await
        .expect("Failed to send a request to /health_check");
    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

//Launch the app in the background
fn spawn_server() {
    let server = ractixbe::run_server().expect("Couldn't spawn test server");
    let _ = tokio::spawn(server);
}

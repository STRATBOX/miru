#[actix_rt::test]
async fn test_ping_endpoint() {
    // arrange
    spawn_app().await.expect("Failed to spawn our app");

    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:3002")
        .send()
        .await
        .expect("Failed to execute request");

    // assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch the application in background
async fn spawn_app() -> std::io::Result<()> {
    miru::run().await
}
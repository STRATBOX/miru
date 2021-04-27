#[actix_rt::test]
async fn test_ping_endpoint() {
    // arrange
    spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:3002")
        .send()
        .await
        .expect("Failed to execute request");

        println!("made it here");

    // assert
    assert!(response.status().is_success());
    // assert_eq!(Some(0), response.content_length());
}

// Launch the application in background
fn spawn_app() {
    let server =  miru::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}
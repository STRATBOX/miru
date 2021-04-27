use std::net::TcpListener;
#[actix_rt::test]
async fn test_ping_endpoint() {
    // arrange
    spawn_app();

    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/", &address))
        .send()
        .await
        .expect("Failed to execute request");

    println!("made it here");

    // assert
    assert!(response.status().is_success());
}

// Launch the application in background
fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    // get the random assigned port
    let port = listener.local_addr().unwrap().port();
    let server = miru::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    // Return the application address to the caller!
    format!("http://127.0.0.1:{}", port)
}

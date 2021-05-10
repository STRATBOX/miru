// use module dependencies
use miru::configuration::get_configuration;
use miru::startup::run;
use miru::telemetry::{get_subscriber, init_subscriber};

// dependencies
use once_cell::sync::Lazy;
use std::net::TcpListener;

// Ensure that the `tracing` stack is only initialised once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber = get_subscriber("test".into(), "debug".into());
    init_subscriber(subscriber);
});

// Launch the application in background
fn spawn_app() -> String {
    // Get app config settings
    let config = get_configuration();

    // The first time `initialize` is invoked the code in `TRACING` is executed.
    // All other invocations will instead skip execution.
    Lazy::force(&TRACING);

    // create test host on port 0 - allow system to pick port
    let test_host = format!("{}:{}", &config.host[..], "0");
    let listener = TcpListener::bind(test_host).expect("Failed to bind random port");

    // get the random assigned port
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    // Return the application address to the caller!
    format!("http://{}:{}", config.host, port)
}

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

    // assert
    assert!(response.status().is_success());
}

// use module dependencies
use miru::configuration::get_configuration;
use miru::startup::run;
use miru::telemetry::{get_subscriber, init_subscriber};

// dependencies
use once_cell::sync::Lazy;
use std::net::TcpListener;

// Ensure that the `tracing` stack is only initialised once using `once_cell`
static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".into();
    let subscriber_name = "test".into();

    // We cannot assign the output of `get_subscriber` to a variable based on the value of `TEST_LOG`
    // because the sink is part of the type returned by `get_subscriber`, therefore they are not the
    // same type. We could work around it, but this is the most straight-forward way of moving forward.

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    };
});

// Launch the application in background
async fn spawn_app() -> String {
    // The first time `initialize` is invoked the code in `TRACING` is executed.
    // All other invocations will instead skip execution.
    Lazy::force(&TRACING);

    // Get app config settings
    let config = get_configuration();

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
    // Arrange
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/", &address))
        .send()
        .await
        .expect("Failed to execute request");

    // assert
    assert!(response.status().is_success());
}

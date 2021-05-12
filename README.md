# H A I K U

Rust microservices skeleton + integration tests + telemetry

## Running the service as an image

This service bundles mongodb and other necessary infrastructure to run locally.

After making code updates, simply start service with the following commands:

```sh
$ docker-compose build
$ docker-compose up
```

### performance testing the service

Download and run [autocannon](https://github.com/mcollina/autocannon) with the appropriate parameters.

```sh
$ autocannon -c 100 -d 5 -p 10 http://0.0.0.0:3002
Running 5s test @ http://0.0.0.0:3002
100 connections with 10 pipelining factor

┌─────────┬──────┬──────┬────────┬────────┬──────────┬──────────┬───────────┐
│ Stat    │ 2.5% │ 50%  │ 97.5%  │ 99%    │ Avg      │ Stdev    │ Max       │
├─────────┼──────┼──────┼────────┼────────┼──────────┼──────────┼───────────┤
│ Latency │ 0 ms │ 0 ms │ 320 ms │ 361 ms │ 27.74 ms │ 86.25 ms │ 565.66 ms │
└─────────┴──────┴──────┴────────┴────────┴──────────┴──────────┴───────────┘
┌───────────┬────────┬────────┬────────┬────────┬────────┬─────────┬────────┐
│ Stat      │ 1%     │ 2.5%   │ 50%    │ 97.5%  │ Avg    │ Stdev   │ Min    │
├───────────┼────────┼────────┼────────┼────────┼────────┼─────────┼────────┤
│ Req/Sec   │ 3261   │ 3261   │ 3481   │ 3711   │ 3499   │ 162.29  │ 3260   │
├───────────┼────────┼────────┼────────┼────────┼────────┼─────────┼────────┤
│ Bytes/Sec │ 571 kB │ 571 kB │ 609 kB │ 650 kB │ 612 kB │ 28.4 kB │ 571 kB │
└───────────┴────────┴────────┴────────┴────────┴────────┴─────────┴────────┘

Req/Bytes counts sampled once per second.

17k requests in 5.06s, 3.06 MB read
```

## auto reloading while building locally

For this, we need to install `cargo-watch` written in Rust and available on [crates.io](http://crates.io), so we can install it with cargo.

```sh
$ cargo install cargo-watch
```

```rust
// src/main.rs
// dependencies
use color_eyre::Result;
use std::net::TcpListener;

// module definitions

// use module dependencies
use miru::configuration::get_configuration;
use miru::startup::run;
use miru::telemetry::{get_subscriber, init_subscriber};

#[actix_web::main]
async fn main() -> Result<()> {
    
    // setup tracing subscriber
    let subscriber = get_subscriber("miru".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // Get app config settings
    let config = get_configuration();

    // bind TCP listener to specified socket address
    let listener = TcpListener::bind(format!("{}:{}", config.app.host, config.app.port))
        .expect("Failed to bind random port");

    // Run server
    run(listener)?.await?;

    Ok(())
}
```

### links
* [Zero to Production - Are we observable yet?](https://www.lpalmieri.com/posts/2020-09-27-zero-to-production-4-are-we-observable-yet/)
* [Mongodb client](https://github.com/mongodb/mongo-rust-driver)
* [Apache pulsar client](https://github.com/wyyerd/pulsar-rs)
* [Rust fullstack](https://github.com/steadylearner/Rust-Full-Stack)
* [The Rust Programming Language - 2018 Edition](https://doc.rust-lang.org/book/2018-edition/index.html)


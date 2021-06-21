FROM lukemathwalker/cargo-chef as planner
WORKDIR app
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare  --recipe-path recipe.json

FROM lukemathwalker/cargo-chef as cacher
WORKDIR app
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependencies, not our application!
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1.50 as builder
WORKDIR app
# Copy over the cached dependencies
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
COPY . .
# Build our application, leveraging the cached deps!
RUN cargo build --release --bin miru

FROM gcr.io/distroless/cc-debian10 as runtime
WORKDIR app
COPY --from=builder /app/target/release/miru miru
COPY app.yaml .
# ENV APP_ENVIRONMENT production
EXPOSE 3002
ENTRYPOINT ["./miru"]
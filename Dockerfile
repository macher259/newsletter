FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR app
ENV SQLX_OFFLINE true

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin newsletter

# We do not need the Rust toolchain to run the binary!
FROM debian:buster-slim AS runtime
WORKDIR app
RUN apt update -y \
    && apt install -y --no-install-recommends openssl ca-certificates \
    &&  apt clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/newsletter newsletter
COPY configuration configuration
COPY migrations migrations
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./newsletter"]
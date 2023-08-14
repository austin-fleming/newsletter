FROM lukemathwalker/cargo-chef:latest-rust-1.64.0 AS chef
WORKDIR /app
#RUN apt-update && apt install lld clang -y
RUN apt-get update && apt-get install -y lld clang

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# build dependencies only
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release --bin newsletter_server

FROM debian:bullseye-slim AS runtime
WORKDIR /app
# install openssl and ca-certificates
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Copy executable from builder to runtime
COPY --from=builder /app/target/release/newsletter_server newsletter_server
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./newsletter_server"]
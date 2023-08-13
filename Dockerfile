FROM rust:1.67 AS builder

WORKDIR /app
#RUN apt-update && apt install lld clang -y
RUN apt-get update && apt-get install -y lld clang
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

FROM rust:1.67-slim AS runtime

WORKDIR /app
# Copy executable from builder to runtime
COPY --from=builder /app/target/release/newsletter_server newsletter_server
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./newsletter_server"]
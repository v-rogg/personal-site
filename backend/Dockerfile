# Build stage
FROM rust:alpine AS builder

RUN apk add --no-cache musl-dev sqlite-dev openssl-dev openssl-libs-static pkgconf

WORKDIR /app

# Cache dependencies
COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release 2>/dev/null || true
RUN rm -rf src

# Build actual application
COPY . .
RUN touch src/main.rs
RUN cargo build --release

# Runtime stage
FROM alpine:3.21

RUN apk add --no-cache sqlite-libs ca-certificates

WORKDIR /app

COPY --from=builder /app/target/release/vr-www-api /app/vr-www-api
COPY --from=builder /app/migrations /app/migrations

ENV DATABASE_URL=/data/app.db
ENV HOST=0.0.0.0
ENV PORT=8080

EXPOSE 8080

CMD ["/app/vr-www-api"]

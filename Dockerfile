FROM rust:1.70-slim as builder

WORKDIR /app
# Cache deps
RUN cargo init
RUN rustup target add x86_64-unknown-linux-musl
COPY Cargo.* ./
# Dummy build for caching
RUN cargo build --target x86_64-unknown-linux-musl --release
# Actual build
COPY ./src ./src
RUN cargo build --target x86_64-unknown-linux-musl --release

#################

FROM alpine:3.16 AS runtime 

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/interpret-deez /usr/local/bin

CMD ["/usr/local/bin/interpret-deez"]

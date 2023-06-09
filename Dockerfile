FROM rust:1.70-alpine as builder

WORKDIR /app
# Cache deps
RUN cargo init
COPY Cargo.* ./
# Dummy build for caching
RUN cargo build --release
# Actual build
COPY ./src ./src
RUN cargo build --release

#################

FROM alpine:3.16 AS runtime 

COPY --from=builder /app/target/release/interpret-deez /usr/local/bin

CMD ["/usr/local/bin/interpret-deez"]

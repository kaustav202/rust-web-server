FROM rust:1.57.0-alpine

RUN apk add --no-cache musl-dev
WORKDIR /opt/rust-proj
COPY . ./

RUN cargo build --release

EXPOSE 5000
CMD ["/opt/rust-proj/target/release/rust-web-server"]
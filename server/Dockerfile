FROM rust as builder
# Update container and install dependencies
RUN apt-get update -y
RUN apt-get install -y build-essential git openssh-client libssl-dev
RUN cargo install wasm-bindgen-cli
RUN rustup target add wasm32-unknown-unknown
WORKDIR /app
COPY ./ /app
RUN cargo build --release
#
#FROM alpine:3.12 as runtime
#WORKDIR app
#COPY --from=builder /app/target/release/server .
EXPOSE 14191
ENV RUST_LOG=debug
ENTRYPOINT ["/app/target/release/server"]
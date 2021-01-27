FROM rust as builder
# Update container and install dependencies
RUN apt-get update -y
RUN apt-get install -y build-essential git openssh-client libssl-dev

RUN cargo install wasm-bindgen-cli
RUN rustup target add wasm32-unknown-unknown

# Copy
WORKDIR /app
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
COPY ./index.html ./index.html
RUN ls ./
RUN ["cargo", "build", "--target", "wasm32-unknown-unknown", "--features", "web", "--release"]
RUN [ "wasm-bindgen","--out-dir", "/app/target", "--out-name", "wasm", "--target", "web", "--no-typescript", "/app/target/wasm32-unknown-unknown/release/client.wasm"]


# Run on nginx
FROM nginx:stable as host
WORKDIR /usr/share/nginx/html
COPY --from=builder ./app/index.html ./index.html
RUN mkdir target
COPY --from=builder ./app/target/wasm.js ./target/wasm.js
COPY --from=builder ./app/target/wasm_bg.wasm ./target/wasm_bg.wasm
EXPOSE 80
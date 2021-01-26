FROM rust:alpine3.12 as builder
RUN apk update
RUN apk add build-base git openssh-client
WORKDIR /app
COPY ./ /app
RUN cargo build --release

FROM alpine:3.12 as runtime
WORKDIR app
COPY --from=builder /app/target/release/server .
EXPOSE 14191
ENTRYPOINT ["./server"]
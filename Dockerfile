FROM rust:1-slim-buster as build-env
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM debian:12-slim
COPY --from=build-env /app/target/release/terminal-ssr /
CMD ["./terminal-ssr"]
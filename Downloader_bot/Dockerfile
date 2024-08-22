FROM rust:1.80.1

WORKDIR /usr/test_docker

COPY Cargo.toml Cargo.lock ./

COPY src ./src

RUN cargo build --release

RUN ls -l target/release/

CMD ["./target/release/Downloader_bot"]

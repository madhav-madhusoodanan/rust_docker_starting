FROM rust:latest as builder
WORKDIR '/usr/app'
COPY ./Cargo.toml .
COPY ./Cargo.lock .
COPY ./src ./src

# RUN apk add musl-tools && rustup target add x86_64-unknown-linux-musl
# RUN cargo +nightly build -Z no-index-update
RUN cargo build
# RUN ls -a -l
# RUN cargo run --production

CMD [ "cargo" "run" ]
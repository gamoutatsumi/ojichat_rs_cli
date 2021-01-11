FROM ekidd/rust-musl-builder:latest

ADD --chown=rust:rust . ./

CMD cargo build --release

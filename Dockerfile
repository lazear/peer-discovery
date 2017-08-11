FROM alpine
RUN apk update && \
    apk add rust cargo

COPY . /rust
RUN cd ./rust && \
    cargo build --release

ENTRYPOINT ["/rust/target/release/rust-bcast"]
EXPOSE 8096

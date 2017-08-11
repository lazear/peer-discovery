# FROM base/archlinux
# RUN pacman -Syu --noconfirm rustup git gcc && \
#     rustup install nightly && \
#     rustup default nightly

FROM alpine
RUN apk update && \
    apk add rust cargo

# FROM alpine:rust
COPY . /rust
RUN cd ./rust && \
    cargo build --release

ENTRYPOINT ["/rust/target/release/rust-bcast"]
EXPOSE 8096

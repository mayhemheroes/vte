FROM ghcr.io/evanrichter/cargo-fuzz as builder

ADD . /vte
WORKDIR /vte/fuzz
RUN cargo +nightly fuzz build 

FROM debian:bookworm
COPY --from=builder /vte/fuzz/target/x86_64-unknown-linux-gnu/release/vte-fuzz /
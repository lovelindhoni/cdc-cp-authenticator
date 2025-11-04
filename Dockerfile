FROM docker.io/rust:1.91 as builder
WORKDIR /app
COPY . .
RUN cargo build --release
RUN cp /app/target/release/cp-authenticator .
RUN cargo clean

FROM docker.io/ubuntu:24.04
RUN apt-get update \
    && apt-get install -y ca-certificates \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /opt
COPY --from=builder /app/cp-authenticator .
EXPOSE 3000
CMD ["./cp-authenticator"]

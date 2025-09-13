FROM rust:slim-trixie AS builder

RUN update-ca-certificates \
  && apt-get update \
  && apt-get install -y --no-install-recommends adduser \
  && rm -rf /var/lib/apt/lists/*

ENV USER=sflow_exporter
ENV UID=10001

RUN adduser \
  --disabled-password \
  --gecos "" \
  --home "/nonexistent" \
  --shell "/sbin/nologin" \
  --no-create-home \
  --uid "${UID}" \
  "${USER}"

RUN cargo new --bin sflow_exporter

WORKDIR /sflow_exporter

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release \
  && rm src/*.rs target/release/deps/sflow_exporter*

COPY ./src ./src
RUN cargo build --release

FROM debian:trixie-slim

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /sflow_exporter

COPY --from=builder /sflow_exporter/target/release/sflow_exporter ./sflow_exporter

USER sflow_exporter:sflow_exporter

CMD ["/sflow_exporter/sflow_exporter", "listen"]

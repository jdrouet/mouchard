FROM rust:1-slim-buster AS base

RUN apt-get update \
  && apt-get install -y libssl-dev pkg-config \
  && rm -rf /var/lib/apt/lists/*

ENV USER=root

WORKDIR /code
RUN cargo init
COPY Cargo.toml /code/Cargo.toml
RUN cargo fetch

COPY src /code/src

CMD [ "cargo", "test", "--offline" ]

FROM base AS builder

RUN cargo build --release --offline

FROM debian:buster-slim

LABEL org.label-schema.schema-version="1.0"
LABEL org.label-schema.docker.cmd="docker run -d -p 3200:3200 jdrouet/mouchard"
LABEL org.label-schema.vcs-url="https://jolimail.io"
LABEL org.label-schema.url="https://github.com/jdrouet/mouchard"
LABEL org.label-schema.description="Service to check if the recipient open their emails"
LABEL maintaner="Jeremie Drouet <jeremie.drouet@gmail.com>"

RUN apt-get update \
  && apt-get install -y ca-certificates libssl1.1 \
  && rm -rf /var/lib/apt/lists/*

ENV ADDRESS=0.0.0.0
ENV PORT=3200
ENV RUST_LOG=info

COPY --from=builder /code/target/release/mouchard /usr/bin/mouchard

EXPOSE 3000

ENTRYPOINT [ "/usr/bin/mouchard" ]

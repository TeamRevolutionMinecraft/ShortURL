FROM rust:latest as build

WORKDIR /app

COPY . .

RUN \
  CARGO_NET_GIT_FETCH_WITH_CLI=true \
  cargo build --release

# https://hub.docker.com/r/bitnami/minideb
FROM bitnami/minideb:latest

# microbin will be in /app
WORKDIR /app

# copy built executable
COPY --from=build \
  /app/target/release/url_shortener \
  /usr/bin/url_shortener

ENV NOT_FOUND_REDIRECT=https://en.wikipedia.org/wiki/Lp0_on_fire
ENV REDIRECT_INDEX=https://team-revolution.net

# Expose webport used for the webserver to the docker runtime
EXPOSE 8080

ENTRYPOINT ["url_shortener"]

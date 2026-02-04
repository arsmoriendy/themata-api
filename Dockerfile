# syntax=docker/dockerfile:1.21

FROM rust:1.93-alpine3.23 as build-stage

WORKDIR /usr/src/themata-api

COPY . .

RUN cargo build --release

FROM alpine:3.23

COPY --from=build-stage /usr/src/themata-api/target/release/themata-api /usr/bin/themata-api

CMD [ "/usr/bin/themata-api" ]

# syntax=docker/dockerfile:1.21

FROM rust:1.93-alpine3.23 as build-stage

WORKDIR /usr/src/themata-be

COPY . .

RUN cargo build --release

FROM alpine:3.23

COPY --from=build-stage /usr/src/themata-be/target/release/themata-be /usr/bin/themata-be

CMD [ "/usr/bin/themata-be" ]

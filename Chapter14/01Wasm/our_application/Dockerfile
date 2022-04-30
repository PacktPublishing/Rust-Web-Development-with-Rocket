FROM rust:alpine as prepare-stage
WORKDIR /app
COPY src src
COPY Cargo.toml Cargo.toml
COPY .cargo .cargo
COPY vendor vendor

FROM prepare-stage as build-stage
RUN apk add --no-cache musl-dev
RUN cargo build --release

FROM rust:alpine
EXPOSE 8000
ENV TZ=Asia/Tokyo \
    USER=staff
RUN addgroup -S $USER \
    && adduser -S -g $USER $USER
RUN apk update \
    && apk add --no-cache ca-certificates tzdata \
    && rm -rf /var/cache/apk/*

WORKDIR /app
COPY --from=build-stage /app/target/release/our_application our_application
COPY Rocket.toml .
COPY static static
COPY src/views src/views

RUN mkdir logs

RUN chown -R $USER:$USER /app

USER $USER
CMD ["./our_application"]

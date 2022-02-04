FROM rust:1.58.0-alpine as sibuilder
RUN apk add --no-cache musl-dev
WORKDIR /opt
RUN USER=root cargo new --bin social-image
WORKDIR /opt/social-image
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm ./src/*.rs
RUN rm ./target/release/deps/social?image*

ADD ./App.toml ./App.toml
ADD ./src ./src
RUN cargo build --release

FROM scratch
WORKDIR /
COPY --from=sibuilder /opt/social-image/App.toml /opt/social-image/target/release/social-image .

EXPOSE 8000
CMD ["/social-image"]


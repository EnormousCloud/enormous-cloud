FROM ekidd/rust-musl-builder:stable as builder
RUN USER=root cargo new --bin fiatprices
WORKDIR /home/rust/src/enormouscloud
COPY ./server/Cargo.lock ./Cargo.lock
COPY ./server/Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs
ADD ./server/src ./src/
RUN rm ./target/x86_64-unknown-linux-musl/release/deps/enormouscloud*
RUN cargo build --release

FROM alpine:latest
EXPOSE 8000
ENV TZ=Etc/UTC \
    APP_USER=appuser \
    RUST_LOG=sqlx=warn,tide=warn,ureq=warn,info
RUN addgroup -S $APP_USER && adduser -S -g $APP_USER $APP_USER
COPY --from=builder /home/rust/src/enormouscloud/target/x86_64-unknown-linux-musl/release/enormouscloud /usr/src/app/enormouscloud
COPY ./client/dist /home/rust/src/enormouscloud/target/x86_64-unknown-linux-musl/release/enormouscloud /usr/src/app/enormouscloud/dist
RUN chown -R $APP_USER:$APP_USER /usr/src/app
USER $APP_USER
WORKDIR /usr/src/app
ENTRYPOINT /usr/src/app/enormouscloud
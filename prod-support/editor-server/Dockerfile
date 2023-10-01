FROM rust:1.70

COPY ./editor-server ./editor-server
COPY ./editor-server/.env.production ./editor-server/.env

WORKDIR /editor-server
RUN cargo build --release

EXPOSE 4000
CMD ["./target/release/editor-server"]

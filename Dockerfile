FROM rust@sha256:bc2c4c21d4467cdbbac0ee7b95a6deb742a3467dfe0ff7647916c4b42ca56372 as builder

WORKDIR /usr/src
COPY . .
RUN cargo build --release

FROM rust:slim@sha256:7eab7ce19273ce785e0ca7578bdf7bc1fcbae621775a8197379a04c9f6e03cd5 as runner
WORKDIR /usr/src/app
COPY --from=builder /usr/src/target/release/home-rater ./

CMD ["./home-rater"]
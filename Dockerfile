FROM rust@sha256:8a4e90c7946b74626488b1446f712c0fe0ed1a596cbd3d871684bc9c5433ae07 as builder

WORKDIR /usr/src
COPY . .
RUN cargo build --release

FROM rust:slim@sha256:7eab7ce19273ce785e0ca7578bdf7bc1fcbae621775a8197379a04c9f6e03cd5 as runner
WORKDIR /usr/src/app
COPY --from=builder /usr/src/target/release/home-rater ./

CMD ["./home-rater"]
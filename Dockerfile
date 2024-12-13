FROM rust as builder

WORKDIR /usr/src
COPY . .
RUN cargo build --release

FROM rust:slim as runner
WORKDIR /usr/src/app
COPY --from=builder /usr/src/target/release/home-rater ./

CMD ["./home-rater"]
FROM rust@sha256:0b0ea86881537b3f645aace7e8e39556289f8787db057317c8d4978548213b9e as builder

WORKDIR /usr/src
COPY . .
RUN cargo build --release

FROM rust:slim@sha256:cc9edbcadf2c27013a642b1d1bb7504e26b1142d6b5ebe58f64cf2eb2bc128ae as runner
WORKDIR /usr/src/app
COPY --from=builder /usr/src/target/release/home-rater ./

CMD ["./home-rater"]
FROM rust@sha256:223cc32ddca8f3ee90c436dbc94b6b5f052ea7f9653c5201e9612f08ad7d21b8 as builder

WORKDIR /usr/src
COPY . .
RUN cargo build --release

FROM rust:slim@sha256:6f14b2b4b278ec0d50aa1d69541c304ce0f95e0f6268487aa9924af1c2e9aebe as runner
WORKDIR /usr/src/app
COPY --from=builder /usr/src/target/release/home-rater ./

CMD ["./home-rater"]
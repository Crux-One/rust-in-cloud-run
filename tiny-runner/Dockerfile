# builder stage
FROM rust:latest as builder

WORKDIR /usr/src/app
COPY . .

RUN rustup show
RUN cargo build --release

# runtime stage
FROM rust:latest as runtime

WORKDIR /usr/src/app
COPY --from=builder /usr/src/app /usr/src/app

# SEE: https://cloud.google.com/run/docs/reference/container-contract
# By default, requests are sent to 8080, but you can configure Cloud Run to send requests to the port of your choice.
ENV PORT 8080
EXPOSE 8080

ENTRYPOINT ["cargo", "run", "--release"]

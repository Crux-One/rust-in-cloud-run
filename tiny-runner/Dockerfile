# builder stage
FROM rust:latest as cargo-builder

ARG X86

WORKDIR /usr/src/app
COPY . .

ENV CC_x86_64_unknown_linux_musl=x86_64-unknown-linux-musl-gcc
ENV CXX_x86_64_unknown_linux_musl=x86_64-unknown-linux-musl-g++
ENV AR_x86_64_unknown_linux_musl=x86_64-unknown-linux-musl-ar
ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=x86_64-unknown-linux-musl-gcc

RUN rustup show
RUN cargo clean
RUN if [ -n "$X86" ] ; then \
	rustup target add x86_64-unknown-linux-musl && cargo build --target=x86_64-unknown-linux-musl --release; \
	else \
	cargo build --release; \
	fi

# runtime stage
FROM debian:buster-slim as runtime

WORKDIR /usr/src/app
COPY --from=cargo-builder /usr/src/app/target/release/runner .

# SEE: https://cloud.google.com/run/docs/reference/container-contract
# By default, requests are sent to 8080, but you can configure Cloud Run to send requests to the port of your choice.
ENV PORT 8080
EXPOSE 8080

ENTRYPOINT ["./runner"]

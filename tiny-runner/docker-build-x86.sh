#!/bin/sh

cat > ./.cargo/config << EOF
[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"
EOF
docker buildx build . --platform linux/amd64 -t tiny-runner --build-arg X86=1 --no-cache


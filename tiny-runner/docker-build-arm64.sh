#!/bin/sh

cat > ./.cargo/config << EOF
EOF
docker buildx build . --platform linux/arm64 -t tiny-runner --no-cache

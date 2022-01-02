#!/bin/sh

docker buildx build . --platform linux/arm64 -t tiny-runner --no-cache

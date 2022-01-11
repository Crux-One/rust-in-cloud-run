#!/bin/sh

docker buildx build . --platform linux/amd64 -t tiny-runner --no-cache

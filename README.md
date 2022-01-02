# [rust-in-cloud-run](https://github.com/Crux-One/rust-in-cloud-run)

[![CI](https://github.com/Crux-One/rust-in-cloud-run/workflows/CI/badge.svg)](https://github.com/Crux-One/rust-in-cloud-run/actions?query=workflow%3ACI)


## Prerequisites
- [Docker](https://www.docker.com)
- [Google Cloud SDK](https://cloud.google.com/sdk/docs/install)

## Build and deploy app to Cloud Run
```
$ cd tiny-runner/
$ ./cargo-install-build.sh
$ ./docker-build-push.sh
```

### Build on Apple M1 chip (ARM based systems)
Run `docker buildx build . --platform linux/amd64 ` so that Docker can build container for x86_64 platform because Cloud Run does NOT support any ARM-compatible images.

```
$ 
```
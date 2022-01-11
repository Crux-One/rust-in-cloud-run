# [rust-in-cloud-run](https://github.com/Crux-One/rust-in-cloud-run)

[![CI](https://github.com/Crux-One/rust-in-cloud-run/workflows/CI/badge.svg)](https://github.com/Crux-One/rust-in-cloud-run/actions?query=workflow%3ACI)


## Prerequisites
- [Docker](https://www.docker.com)
- [Google Cloud SDK](https://cloud.google.com/sdk/docs/install)

## Building and running on localhost
```
$ cd tiny-runner/
$ ./cargo-install-build.sh && ./cargo-run.sh
```

You can run `docker-run.sh` instead of `cargo-run.sh` in order to run the app on a docker container.

## Pushing an image and deploying app to Cloud Run
You need to first run these commands that will enable a Cloud Run service and configure the registry you have.
```
$ gcloud services enable run.googleapis.com --project [project id]
$ gcloud auth configure-docker asia-northeast1-docker.pkg.dev
```

For example, the following commands:
```
$ ./docker-build.sh
$ docker tag tiny-runner asia-northeast1-docker.pkg.dev/[project id]/[registry]/[image]:[tag] && \
  docker push asia-northeast1-docker.pkg.dev/[project id]/[registry]/[image]
$ gcloud run deploy cloudrun-tiny-runner --image asia-northeast1-docker.pkg.dev/[project id]/[registry]/[image]:[tag] --region [region] --platform managed
```

### :warning: Building on M1 chip (ARM based systems)
To build an image on Apple M1, [macos-cross-toolchains](https://github.com/messense/homebrew-macos-cross-toolchains) must be installed and changed a configuration, run the following commands so that cargo/Docker can build it for x86-based CPUs because Cloud Run does NOT support any ARM-compatible images at the moment.

#### Installing cross compiler toolchains
```
$ brew tap messense/macos-cross-toolchains
$ brew install x86_64-unknown-linux-musl
```

#### Setting PATH
```
export CC_x86_64_unknown_linux_musl=x86_64-unknown-linux-musl-gcc
export CXX_x86_64_unknown_linux_musl=x86_64-unknown-linux-musl-g++
export AR_x86_64_unknown_linux_musl=x86_64-unknown-linux-musl-ar
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=x86_64-unknown-linux-musl-gcc
```

#### Cargo Configuration
You will need to add or modify the following into your project's `.cargo/config` file.

```
[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"
```

#### Building app
```
$ cargo build --target=x86_64-unknown-linux-musl
```

#### Building docker image
For example, the following commands:
```
$ docker buildx build . --platform linux/amd64 -t tiny-runner --no-cache
```
You can also get the same result with the following script running.
```
$ ./docker-build-x86.sh
```


### sccache
[sccache - Shared Compilation Cache](https://github.com/mozilla/sccache)
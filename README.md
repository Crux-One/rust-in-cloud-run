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

You can run `docker-run.sh` instead of `cargo-run.sh` in order to run app on a docker container.

## Pushing an image and deploying app to Cloud Run
You need to first run this command that will enable a Cloud Run service.
```
$ gcloud services enable run.googleapis.com --project [project id]
```

For example, the following commands:
```
$ ./docker-build-x86.sh
$ docker tag tiny-runner gcr.io/[project id]/[registry]/[image]:[tag] && \
  docker push us-central1-docker.pkg.dev/[project id]/[registry]/[image]
$ gcloud run deploy cloudrun-tiny-runner --image us-central1-docker.pkg.dev/[project id]/[registry]/[image]:[tag] --region [region] --platform managed
```

### Building on M1 chip (ARM based systems)
To build an image on Apple M1, you must install [macos-cross-toolchains](https://github.com/messense/homebrew-macos-cross-toolchains) and run the following command so that Docker can build container for x86-based CPUs, because Cloud Run does NOT support any ARM-compatible images at the moment.

```
$ docker buildx build . --platform linux/amd64
```

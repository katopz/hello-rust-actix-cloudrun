# hello-rust-cloudrun

Hello Rust on CloudRun

> ref: https://github.com/knative/docs/tree/main/code-samples/community/serving/helloworld-rust

## Deploy

> This will deploy built image to `CloudRun` to prove that it works.

```shell
# Install gcloud cli and login
open https://cloud.google.com/sdk/docs/install

# Deploy samples image to `CloudRun`
gcloud run deploy --image=gcr.io/knative-samples/helloworld-rust:v0.1.0 --platform=managed
```

## Try online

> replace with your own `YOUR_ID_BLA_BLA` from previous step

```shell
curl -w '\n' https://hello-rust-cloudrun-YOUR_ID_BLA_BLA-as.a.run.app/
```

> Expected output:

```shell
Hello World
```

---

## Setup

> This will setup `homebrew`, `docker desktop` and `docker` for build image locally.

```shell
# Install Homebrew
/usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"

# Install `Docker Desktop`
open https://docs.docker.com/desktop/mac/install/

# Install Docker
brew install docker
```

> This will setup `kubectl` to run the image locally.

```shell
# Install the `Knative` CLI.
brew install kn

# Install the Knative quickstart plugin.
brew install knative-sandbox/kn-plugins/quickstart

# Install go.
brew install go

# Install kind.
go get sigs.k8s.io/kind

# Run the Knative quickstart plugin.
kn quickstart kind

# Verify you have a cluster called `knative`.
kind get clusters
```

## Build image

```shell
# Get `username` from Docker hub and sign-in via `Docker Desktop`.
open https://hub.docker.com/

# Clone this repository.
git clone https://github.com/katopz/hello-rust-cloudrun
cd hello-rust-cloudrun
username=katopz 👈 your username

# Build the container on your local machine.
docker build -t $username/helloworld-rust .

# Push the container to docker registry.
docker push $username/helloworld-rust
```

## Deploy

> Deploy again, this time on our local.

```shell
# Deploy the app into your cluster.
kubectl apply --filename service.yaml

# Find the URL for your service.
kubectl get ksvc helloworld-rust  --output=custom-columns=NAME:.metadata.name,URL:.status.url
```

> Expected output:

```shell
NAME              URL
helloworld-rust   http://helloworld-rust.default.127.0.0.1.sslip.io
```

## Try local

```shell
curl -w '\n' http://helloworld-rust.default.127.0.0.1.sslip.io
```

> Expected output:

```
Hello Rust Sample v1
```

## Cleanup

```shell
# To remove the sample app from your cluster, delete the service record:
kubectl delete --filename service.yaml
```

## Other versions

> rust:latest = 1.68GB  
> rust:alpine = 1.23GB // with fixed error: linking with `cc` failed  
> rust:slim-buster = 1.04GB // recommended

```shell
# To build `rust:alpine` image.
docker build -t $username/helloworld-rust . -f alpine.Dockerfile

# To build `rust:slim-buster` image.
docker build -t $username/helloworld-rust . -f slim-buster.Dockerfile
```

## TODO
- Multi stage build: https://dev.to/sergeyzenchenko/actix-web-in-docker-how-to-build-small-and-secure-images-2mjd
- Reduce size to 6mb (no dev): https://github.com/kpcyrd/mini-docker-rust

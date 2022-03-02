# hello-rust-cloudrun

Hello Rust on CloudRun

> ref: https://github.com/knative/docs/tree/main/code-samples/community/serving/helloworld-rust

## Deploy

> This will deploy built image to CloudRun to prove that it works.

```
# Install gcloud cli and login
open https://cloud.google.com/sdk/docs/install

# Deploy samples image to CloudRun
gcloud run deploy --image=gcr.io/knative-samples/helloworld-rust:v0.1.0 --platform=managed
```

## Setup

> This will setup `homebrew`, `docker desktop` and `docker` for build image locally.

```
# Install Homebrew
/usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"

# Install `Docker Desktop`
open https://docs.docker.com/desktop/mac/install/

# Install Docker
brew install docker
```

> This will setup `kubectl` to run the image locally.

```
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

```
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

```
# Deploy the app into your cluster.
kubectl apply --filename service.yaml

# Find the URL for your service.
kubectl get ksvc helloworld-rust  --output=custom-columns=NAME:.metadata.name,URL:.status.url
```

> Expected output:

```
NAME              URL
helloworld-rust   http://helloworld-rust.default.127.0.0.1.sslip.io
```

## Cleanup

```
# To remove the sample app from your cluster, delete the service record:
kubectl delete --filename service.yaml
```

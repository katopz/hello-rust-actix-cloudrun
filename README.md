# hello-rust-cloudrun

Hello Rust on CloudRun

ref: https://github.com/knative/docs/tree/main/code-samples/community/serving/helloworld-rust

- run Docker

```
# Install the Knative CLI
brew install kn

# Install the Knative quickstart plugin
brew install knative-sandbox/kn-plugins/quickstart

# Install kind
go get sigs.k8s.io/kind

# Run the Knative quickstart plugin
kn quickstart kind

# Verify you have a cluster called knative
kind get clusters
```

```
# In your source code directory
gcloud run deploy --image=gcr.io/knative-samples/helloworld-rust:v0.1.0 --platform=managed
```

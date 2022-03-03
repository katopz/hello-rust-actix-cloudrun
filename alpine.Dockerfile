# Use the official Rust image.
# https://hub.docker.com/_/rust
FROM rust:alpine

# Copy local code to the container image.
WORKDIR /usr/src/app
COPY . .

# https://dev.to/nabbisen/rustup-on-alpine-linux-fix-error-linker-cc-not-found-309h
RUN apk add build-base

# Install production dependencies and build a release artifact.
RUN cargo install --path .

# Run the web service on container startup.
CMD ["hellorust"]
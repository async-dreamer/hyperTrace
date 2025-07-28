#!/bin/bash

set -e

# Detect container runtime
if command -v podman &> /dev/null; then
    CONTAINER_RUNTIME="podman"
elif command -v docker &> /dev/null; then
    CONTAINER_RUNTIME="docker"
else
    echo "Error: Neither podman nor docker is installed."
    exit 1
fi

# Absolute paths
PROJECT_DIR="$(realpath .)"
ARTIFACT_PATH="$PROJECT_DIR/artifacts/hyperTrace"

# Validate colon-free path
if [[ "$PROJECT_DIR" == *:* ]]; then
    echo "Error: Project directory path contains a colon (:), which is not supported in volume mounts."
    exit 1
fi

# Clean up
rm -rf "$ARTIFACT_PATH"

# Volume mount (split into -v and arg to avoid parsing bugs)
if [ "$CONTAINER_RUNTIME" = "podman" ]; then
    VOLUME_MOUNT="${PROJECT_DIR}:/app:Z"
else
    VOLUME_MOUNT="${PROJECT_DIR}:/app"
fi

# Build image
"$CONTAINER_RUNTIME" build --platform linux/amd64 -t rust-app-image .

# Build in container — notice: -v and its arg are separate
"$CONTAINER_RUNTIME" run -it --rm -v "$VOLUME_MOUNT" rust-app-image cargo build --release

# Move built binary
mkdir -p "$(dirname "$ARTIFACT_PATH")"
mv "$PROJECT_DIR/target/release/hyperTrace" "$ARTIFACT_PATH"
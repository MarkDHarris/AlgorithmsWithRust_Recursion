#!/bin/bash
start_time=$(date +%s)

USERNAME="maharris"
IMAGENAME="rustdev:latest"
ARCH="amd64"

docker build -t $IMAGENAME --progress=plain -f ./.devcontainer/ubuntu/Dockerfile --build-arg USERNAME="$USERNAME" --build-arg ARCH="$ARCH" ./.devcontainer

end_time=$(date +%s)
duration=$((end_time - start_time))

hours=$((duration / 3600))
minutes=$(( (duration % 3600) / 60 ))
seconds=$((duration % 60))
printf "%02d:%02d:%02d" $hours $minutes $seconds
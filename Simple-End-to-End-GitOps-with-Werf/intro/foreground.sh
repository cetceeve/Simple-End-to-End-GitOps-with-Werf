#!/bin/bash
echo "Pre-building demo app image"
docker build -t localhost:5000/demo-app:latest demo-app/
echo "Installing local image registry..."
docker run -d -p 5000:5000 --restart=always --name registry registry:2
echo DONE
#!/bin/bash
echo "Installing local image registry..."
docker run -d -p 5000:5000 --restart=always --name registry registry:2
export LOCAL_REGISTRY="localhost:5000"
echo "Setup git environment"
git config --global user.email "demo@example.com"
git config --global user.name "Killrcoder"
echo $LOCAL_REGISTRY
echo "All set! You are ready to go!"
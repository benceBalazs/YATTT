#!/bin/bash

docker build -t rust-service:latest .

# run local registry
docker run -d -p 5000:5000 --name registry registry:2

# push to local registry
docker tag rust-service:latest localhost:5000/rust-service:latest
docker push localhost:5000/rust-service:latest
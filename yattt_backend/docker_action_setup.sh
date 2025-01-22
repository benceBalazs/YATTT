#!/bin/bash

docker build -t yattt_backend_service .

docker save -o yattt_backend_service.tar.gz yattt_backend_service:latest
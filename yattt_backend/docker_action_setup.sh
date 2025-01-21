#!/bin/bash

docker build -t yatt_backend_service .

docker save -o yatt_backend_service.tar.gz yatt_backend_service:latest
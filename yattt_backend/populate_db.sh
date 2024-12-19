#!/bin/bash

# make sure SurrealDB is running on port 127.0.0.1:8000
cat setup.surql | surreal sql --user root --pass root
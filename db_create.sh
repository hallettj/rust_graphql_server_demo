#!/bin/bash

set -e

docker run \
    --name graphql_server_demo_db \
    -e POSTGRES_PASSWORD=password \
    -p 127.0.0.1:5432:44544/tcp \
    --detach \
    postgres

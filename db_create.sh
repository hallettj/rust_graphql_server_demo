#!/bin/bash

set -e

docker run \
    --name graphql_server_demo_db \
    -e POSTGRES_PASSWORD=password \
    -p 127.0.0.1:44544:5432/tcp \
    --detach \
    postgres

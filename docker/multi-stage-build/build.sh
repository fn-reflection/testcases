#!/bin/bash
set -eux
set -o pipefail

# cache-fromで指定したタグ付きイメージがあるならキャッシュを使いビルドする
docker build --file docker/Dockerfile -t testcases:latest --cache-from=testcases:base,testcases:build_client,testcases:build_server --build-arg BUILDKIT_INLINE_CACHE=1 .

# 中間キャッシュをタグ付でビルドする
docker build --file docker/Dockerfile -t testcases:base --target base --cache-from testcases:base --build-arg BUILDKIT_INLINE_CACHE=1 . &
docker build --file docker/Dockerfile -t testcases:build_client --target build_client --cache-from testcases:build_client --build-arg BUILDKIT_INLINE_CACHE=1 . &
docker build --file docker/Dockerfile -t testcases:build_server --target build_server --cache-from testcases:build_server --build-arg BUILDKIT_INLINE_CACHE=1 . &
wait

existing_db_container_id=$(docker container ls -f name=testcases_tcp_server --format "{{.ID}}")
if [ -z "$existing_db_container_id" ]; then
    docker run -d --name testcases_tcp_server testcases:latest ./tcp_server
fi

docker run --net=container:testcases_tcp_server --rm -it testcases:latest ./tcp_client

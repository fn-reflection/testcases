# cache-fromで指定したタグ付きイメージがあるならキャッシュを使いビルドする
docker build -t test:latest --cache-from=test:stage1,test:stage2 --build-arg BUILDKIT_INLINE_CACHE=1 .

# 中間キャッシュをタグ付でビルドする
docker build -t test:stage1 --target stage1 --cache-from test:stage1 --build-arg BUILDKIT_INLINE_CACHE=1 . &
docker build -t test:stage2 --target stage2 --cache-from test:stage2 --build-arg BUILDKIT_INLINE_CACHE=1 . &
wait

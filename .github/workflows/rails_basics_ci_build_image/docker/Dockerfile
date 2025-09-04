FROM ubuntu:24.04

RUN apt-get update && apt-get install -y \
    imagemagick \
    libmagickwand-dev \
    && rm -rf /var/lib/apt/lists/*

# ImageMagick動作確認
RUN magick --version

WORKDIR /workdir

FROM ubuntu:24.04

RUN apt-get update && apt-get install -y \
    build-essential \
    # for split-tests-by-timings action
    curl \
    imagemagick \
    libmagickwand-dev \
    # to install psych gem
    libyaml-dev \
    && rm -rf /var/lib/apt/lists/*

# ImageMagickコマンド動作確認
RUN convert --version

WORKDIR /workdir

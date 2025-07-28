FROM ubuntu:25.04

RUN apt-get update && apt-get install -y curl clang libicu-dev \
binutils git gnupg2 \
libc6-dev \
libcurl4-openssl-dev \
libedit2 \
libgcc-13-dev \
libncurses-dev \
libpython3-dev \
libsqlite3-0 \
libstdc++-13-dev \
libxml2-dev \
libz3-dev \
pkg-config \
tzdata \
unzip \
zlib1g-dev

# Set the working directory inside the container
WORKDIR /app

# Install Rust using rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Add Rust to the PATH
ENV PATH="/root/.cargo/bin:${PATH}"

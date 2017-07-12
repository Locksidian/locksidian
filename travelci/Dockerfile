FROM debian:jessie
MAINTAINER Valentin Fries <contact@fries.io>

# Install dependencies
RUN apt-get update &&\
    apt-get install -y\
        # Rust
        curl wget g++ git\
        # Locksidian
        libsqlite3-dev libssl-dev pkg-config\
        # kcov
        cmake python\
        libcurl4-openssl-dev libelf-dev libdw-dev\
        libbfd-dev binutils-dev libiberty-dev zlib1g-dev

# Configure Rust development environment
RUN curl -sSf https://sh.rustup.rs/ | sh -s -- --default-toolchain nightly -y

# Install kcov
RUN wget https://github.com/SimonKagstrom/kcov/archive/master.tar.gz &&\
    tar xzf master.tar.gz &&\
    cd kcov-master &&\
    mkdir build &&\
    cd build &&\
    cmake .. &&\
    make &&\
    make install

# Clean image
RUN apt-get clean &&\
    rm -rf /var/lib/apt/lists/* &&\
    rm -rf /tmp/*

# Configure image
ENV PATH=/root/.cargo/bin:$PATH\
    USER=root

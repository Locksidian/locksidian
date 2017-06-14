FROM registry.gitlab.com/locksidian/locksidian-ci:master
MAINTAINER Valentin Fries <contact@fries.io>

COPY . /src

RUN cd /src && \
    cargo build --release && \
    mkdir /app && \
    cp /src/target/release/locksidian /app/locksidian && \
    chmod +x /app/locksidian && \
    rm -rf /src

EXPOSE 8080
VOLUME /root/.locksidian

WORKDIR /app

ENTRYPOINT ["/app/locksidian"]
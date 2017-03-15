FROM registry.gitlab.com/locksidian/locksidian-ci:master
MAINTAINER Valentin Fries <contact@fries.io>

COPY . /src

RUN cd /src && \
    cargo build --release && \
    mkdir /app && \
    mkdir -p /opt/locksidian && \
    cp /src/target/release/locksidian /app/locksidian && \
    chmod +x /app/locksidian && \
    rm -rf /src

EXPOSE 8080
VOLUME /opt/locksidian

WORKDIR /app

ENTRYPOINT ["/app/locksidian", "-d", "0.0.0.0:8080"]
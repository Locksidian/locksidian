FROM registry.gitlab.com/locksidian/locksidian-ci:master

COPY src/ /src

RUN cd /src && \
    cargo build --release && \
    mkdir -p /opt/locksidian && \
    cp /src/target/release/locksidian /opt/locksidian/locksidian && \
    chmod +x /opt/locksidian/locksidian && \
    rm -rf /src

WORKDIR /opt/locksidian
ENTRYPOINT ["/opt/locksidian/locksidian"]
FROM alpine:latest

COPY target/release/locksidian /opt/locksidian/locksidian

ENTRYPOINT ["/opt/locksidian/locksidian"]
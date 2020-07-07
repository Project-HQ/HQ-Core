FROM scratch
ADD target/x86_64-unknown-linux-musl/release/hq-core /
EXPOSE 8080
CMD ["/hq-core"]

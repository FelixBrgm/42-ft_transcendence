FROM rust:latest

# Update package list and install netcat
RUN apt-get update && \
    apt-get install -y netcat && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

ENV CARGO_TARGET_DIR=/target

ENV CARGO_HOME=/usr/src/server/target/dep

WORKDIR /usr/src/server

VOLUME [ "/usr/src/server" ]

EXPOSE 7878

CMD ["cargo", "run"]

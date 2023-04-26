FROM rust:latest

EXPOSE 4242

# Update package list and install netcat
RUN apt-get update && \
    apt-get install -y netcat && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*


ENV CARGO_HOME=/usr/src/development/server/target/dependencies

WORKDIR /usr/src/development

VOLUME [ "/usr/src/development" ]

CMD ["tail", "-f", "/dev/null"]

# Base image
FROM ubuntu:22.04

# Install necessary dependencies for Node.js and Rust
RUN apt-get update && \
    apt-get install -y curl && \
    apt-get install -y nodejs npm && \
    apt-get install -y build-essential && \
    apt-get install -y libpq-dev && \
    apt-get install -y git && \
    apt-get install -y pkg-config && \
    apt-get install -y netcat && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Install Vue CLI
RUN npm install -g @vue/cli

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

USER root

# Update package list and install netcat

# Install diesel_cli for PostgreSQL
RUN cargo install diesel_cli --no-default-features --features "postgres"

# Set the PS1 environment variable in the container's bashrc
RUN echo 'export PS1="\W> "' >> ~/.bashrc

EXPOSE 4242
EXPOSE 8080
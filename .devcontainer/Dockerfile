# Base image
FROM ubuntu:22.04

# Install necessary dependencies
RUN apt-get update && apt-get install -y \
        curl \
        nodejs npm \
        build-essential \
        libpq-dev \
        git \
        pkg-config \
        netcat \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Remove conflicting package libnode-dev
RUN apt-get purge -y libnode-dev

# Install Node.js LTS version
RUN curl -sL https://deb.nodesource.com/setup_lts.x | bash - && apt-get install -y nodejs

# Install Vue CLI
RUN npm install -g @vue/cli

# Install Rust + cleanup
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && rm -rf /var/lib/apt/lists/*

ENV PATH="/root/.cargo/bin:${PATH}"

# Install diesel_cli for PostgreSQL
RUN cargo install diesel_cli --no-default-features --features "postgres"

# Set the PS1 environment variable in the container's bashrc
RUN echo 'export PS1="\W> "' >> ~/.bashrc

# Expose ports
EXPOSE 4242
EXPOSE 8080

# Default command to keep the container running
CMD ["tail", "-f", "/dev/null"]

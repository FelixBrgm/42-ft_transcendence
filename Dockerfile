# Base image
FROM ubuntu:22.04

# Install necessary dependencies for Node.js and Rust
RUN apt-get update && \
    apt-get install -y curl && \
    curl -sL https://deb.nodesource.com/setup_14.x | bash - && \
    apt-get install -y nodejs && \
    apt-get install -y build-essential && \
    apt-get install -y netcat && \
    rm -rf /var/lib/apt/lists/*
# Install Vue CLI
RUN npm install -g @vue/cli

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

USER root

# Update package list and install netcat
RUN apt-get update && \
	apt-get install -y netcat && \
	apt-get clean && \
	rm -rf /var/lib/apt/lists/*

# Set the PS1 environment variable in the container's bashrc
RUN echo 'export PS1="\W> "' >> ~/.bashrc

# So that libraries are persistant between container starts
ENV CARGO_HOME=/usr/src/development/server/target/dependencies

# Setting working directory
WORKDIR /usr/src/development

# Mounting the volume to the docker container
VOLUME [ "/usr/src/development" ]

CMD ["tail", "-f", "/dev/null"]
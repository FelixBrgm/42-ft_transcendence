# Base image
FROM node:14

# Install Vue CLI
RUN npm install -g @vue/cli

RUN cd client && npm i && cd ..

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

USER root

# Update package list and install netcat
RUN apt-get update && \
	apt-get install -y netcat && \
	apt-get clean && \
	rm -rf /var/lib/apt/lists/*


# So that libraries are persistant between container starts
ENV CARGO_HOME=/usr/src/development/server/target/dependencies

# Setting working directory
WORKDIR /usr/src/development

# Mounting the volume to the docker container
VOLUME [ "/usr/src/development" ]

CMD ["tail", "-f", "/dev/null"]

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
    nginx \
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
# Create a directory for your app
WORKDIR /app
# Copy your Rust backend and Vue frontend code into the container
COPY /client /app/client
COPY /server /app/server
# Build your Rust backend
RUN cd server && cargo build --release
# Build your Vue frontend
RUN cd client && npm install
RUN cd client && npm run build

COPY nginx.conf /etc/nginx/nginx.conf

# Start your Rust backend and Vue frontend
EXPOSE 8081
EXPOSE 80

CMD ["sh", "-c", "service nginx start & /app/server/target/release/server"]
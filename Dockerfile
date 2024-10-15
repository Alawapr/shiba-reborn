# Getting the alpine image
FROM debian:stable-slim

# Installing curl & updating the packages
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    curl \
    build-essential \
    ca-certificates \
    pkg-config \
    libssl-dev \
    git \
    && rm -rf /var/lib/apt/lists/*

# Create a working directory for Shiba
WORKDIR /home/compilation
COPY . .

# Installing rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Compiling Shiba
RUN cargo build --release -F prod 

# Moving the binary from release to production
WORKDIR /home/production
RUN cp /home/compilation/target/release/shiba_reborn /home/production/
RUN cp /home/compilation/.env /home/production/
RUN mv /home/compilation/.git /home/production/
RUN chmod +x shiba_reborn

# Running the binary
CMD ["./shiba_reborn"]
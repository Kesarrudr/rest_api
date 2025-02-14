# Use Ubuntu 23.10 for both stages
FROM ubuntu:23.10 AS builder

# Install dependencies
RUN apt-get update && apt-get install -y \
  curl build-essential libpq-dev pkg-config ca-certificates \
  && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:$PATH"

WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime Stage
FROM ubuntu:23.10

# Install required dependencies
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*

# Copy binary
COPY --from=builder /app/target/release/mining-pool-api /mining-pool-api

CMD ["/mining-pool-api"]

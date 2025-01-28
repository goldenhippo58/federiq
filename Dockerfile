# Use Rust official image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo manifest first to cache dependencies
COPY Cargo.toml Cargo.lock ./

# Create a dummy src directory to satisfy dependencies
RUN mkdir -p src/federated_query_engine

# Build dependencies separately to optimize Docker caching
RUN cargo build --release || echo "Initial build for dependency caching"

# Copy the entire project
COPY . .

# Build the actual application
RUN cargo build --release

# Ensure the binary is executable
RUN chmod +x /app/target/release/federated_query_engine

# Expose application port
EXPOSE 3000

# Run the application explicitly with full path
CMD ["/app/target/release/federated_query_engine"]

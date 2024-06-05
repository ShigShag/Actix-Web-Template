# Builder stage
FROM rust:bullseye as builder

ARG HOST="0.0.0.0"
ARG PORT="8000"

# Set the working directory
WORKDIR /usr/src/actix-web-template

# Copy your Cargo.toml into the image
COPY ./Cargo.toml ./Cargo.toml

# Create a dummy main.rs file to compile dependencies
RUN mkdir src \
    && echo "fn main() {println!(\"if you see this, the build broke\");}" > src/main.rs

# Build your application to cache the dependencies
RUN cargo build --release

# Remove the dummy source and target directory, then copy the actual source code
RUN rm -rf ./src ./target/release/deps/actix_web_template*
COPY ./src ./src
COPY ./static ./static

# Rebuild your application with the actual source code
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

# Update package list and install libpq
RUN apt-get update && \
    apt-get install -y libpq5 && \
    rm -rf /var/lib/apt/lists/*

# Create a non-root user to run your application
RUN useradd -m -s /bin/false dummy

# Copy the binary from the builder stage
COPY --from=builder /usr/src/actix-web-template/target/release/actix-web-template /usr/local/bin/actix-web-template
COPY --from=builder /usr/src/actix-web-template/static/ /usr/src/actix-web-template/static/

# Adjust ownership and permissions
USER root
RUN find / -perm /6000 -type f -exec chmod a-s {} \; || true && \
    chown -R dummy:dummy /usr/local/bin/actix-web-template /usr/src/actix-web-template && \
    chmod -R 750 /usr/local/bin/actix-web-template /usr/src/actix-web-template

# Switch to dummy user
USER dummy

# Expose port 9090
EXPOSE ${PORT}

# Set certificate path
ENV HOST=${HOST}
ENV PORT=${PORT}
ENV STATIC_PATH=/usr/src/actix-web-template/static

# Command to run the application
CMD ["/usr/local/bin/actix-web-template"]

# Use a base image with the latest version of Rust installed
FROM rust:latest

# Set the working directory in the container
WORKDIR /app

# Copy the local application code into the container
COPY . .

# Specify the command to run when the container starts
CMD ["./target/release/my-rust-app"]


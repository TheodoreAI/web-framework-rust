# Use a base image with Rust pre-installed
FROM rust:latest

# Set the working directory
WORKDIR /usr/src/app

# Copy your Rust project files to the image
COPY . .

# Build your Rust project
RUN cargo build

# Specify the command to run your Rust application
CMD ["cargo", "run"]

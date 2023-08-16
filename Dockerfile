# Use a base image with Rust pre-installed
FROM rust:latest AS build-deps

# Set the working directory
WORKDIR /usr/src/app

# Copy only the dependency-related files
COPY Cargo.toml Cargo.lock ./

# Build your Rust dependencies
RUN cargo build --release

# Create a new stage with a clean base image
FROM rust:latest

# Set the working directory
WORKDIR /usr/src/app

# Copy all the project files to the image
COPY . .

# Copy the built dependencies from the previous stage
COPY --from=build-deps /usr/src/app/target/release/ /usr/src/app/target/release/

# Specify the command to run your Rust application
CMD ["cargo", "run", "--release"]

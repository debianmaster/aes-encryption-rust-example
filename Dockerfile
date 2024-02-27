# Stage 1: Building the application
FROM rust:1.58-alpine3.15 AS builder

# Install build dependencies
RUN apk add --no-cache musl-dev

# Create a new empty shell project
RUN USER=root cargo new --bin enc-dec-service
WORKDIR /enc-dec-service

# Copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# This step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# Now that the dependencies are built, copy your source code
COPY ./src ./src

# Build your application
RUN rm ./target/release/deps/enc_dec_service*
RUN cargo build --release

# Stage 2: Preparing the final image
FROM alpine:latest

# Install needed libraries
RUN apk add --no-cache libgcc

# Copy the built executable from the builder stage
COPY --from=builder /enc-dec-service/target/release/enc-dec-service .

# Expose the port the app runs on
EXPOSE 8080

# Command to run the executable
CMD ["./enc-dec-service"]

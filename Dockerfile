# Build stage
FROM rust:1.81.0-bullseye as builder

WORKDIR /src

# Accept the build argument
ARG DATABASE_URL

# Make sure to use the ARG in ENV
ENV DATABASE_URL=$DATABASE_URL

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release


# Production stage
FROM debian:buster-slim

WORKDIR /usr/local/bin

COPY --from=builder /src/target/release/nodes-challenge-bipa .

RUN apt-get update && apt install -y openssl && apt-get install libpq-dev -y && apt install libc6 -y

CMD ["./nodes-challenge-bipa"]
FROM rust:1.70 as builder

WORKDIR /usr/src/xtrackr
COPY . .

# Install Diesel CLI
RUN cargo install diesel_cli --no-default-features --features postgres

# Build the application
RUN cargo build --release

# Create a minimal runtime image
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y libpq5 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/xtrackr/target/release/xtrackr /usr/local/bin/xtrackr
COPY --from=builder /usr/src/xtrackr/migrations /migrations

WORKDIR /app
CMD ["xtrackr"] 

> 🚧 Work in Progress
> XTrackr is currently under active development. While key features are in place, expect breaking changes, incomplete documentation, and the occasional existential bug. Contributions and feedback are welcome—just don’t expect a polished experience yet.

# XTrackr

A privacy-first, GDPR-compliant tracking solution built with Rust and PostgreSQL.

## Features

- Privacy-first design
- GDPR compliance
- IP anonymization
- Configurable data retention
- Consent management
- Fast and memory-safe implementation
- PostgreSQL backend

## Local Development Setup

### Prerequisites

- Rust 1.86 or later
- PostgreSQL 13 or later
- Docker and Docker Compose (optional)
- Diesel CLI

### Installing Diesel CLI

```bash
cargo install diesel_cli --no-default-features --features postgres
```

2. Run migrations:
```bash
# Run all pending migrations
diesel migration run
```

3. Build and run the application:
```bash
cargo run
```

### Docker Setup

1. Start the services:
```bash
docker-compose up -d
```

4. Run migrations in the container:
```bash
docker-compose exec app diesel migration run
```

## Testing the Application

### Health Check

```bash
curl http://localhost:3000/health
# Expected response: "OK"
```

### Tracking an Event

1. First, create a visitor and get consent:
```bash
curl -X POST http://localhost:3000/consent \
  -H "Content-Type: application/json" \
  -d '{
    "visitor_id": "550e8400-e29b-41d4-a716-446655440000",
    "consent_given": true
  }'
```

2. Create a session:
```bash
curl -X POST http://localhost:3000/session \
  -H "Content-Type: application/json" \
  -d '{
    "visitor_id": "550e8400-e29b-41d4-a716-446655440000",
    "user_agent": "Mozilla/5.0",
    "ip_address": "192.168.1.1"
  }'
```

3. Track an event:
```bash
curl -X POST http://localhost:3000/track \
  -H "Content-Type: application/json" \
  -d '{
    "session_id": "550e8400-e29b-41d4-a716-446655440000",
    "event_type": "page_view",
    "path": "/home",
    "user_agent": "Mozilla/5.0",
    "referrer": "https://example.com",
    "ip_address": "192.168.1.1",
    "metadata": {
      "screen_width": 1920,
      "screen_height": 1080
    }
  }'
```

### Verifying Data

You can check the data in PostgreSQL:

```bash
# Connect to the database
psql -U postgres -d xtrackr

# Check visitors
SELECT * FROM visitors;

# Check sessions
SELECT * FROM sessions;

# Check events
SELECT * FROM events;
```

## Development Workflow

1. Make changes to the code
2. Run tests:
```bash
cargo test
```

3. Run migrations (if schema changes):
```bash
diesel migration generate <migration_name>
# Edit the generated migration files
diesel migration run
```

4. Build and run:
```bash
cargo run
```

### Issues with `libpq` on macOS

Follow the link for the [solution](https://stackoverflow.com/a/70561227)

## GDPR Compliance

XTrackr is designed with GDPR compliance in mind:

- IP anonymization
- Configurable data retention
- Consent management
- Data minimization
- Right to be forgotten support

## License

MIT License

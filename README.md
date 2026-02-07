# Rusteze

A modular Rust web application built with Actix-web, Diesel ORM, and PostgreSQL.

## Project Structure

This is a Cargo workspace with the following structure:

- `crates/libs/lib-core` - Core library containing database models and schema definitions
- `crates/libs/lib-auth` - Authentication module
- `crates/services/web-server` - Main web server service using Actix-web

## Prerequisites

- Rust (edition 2024)
- Docker and Docker Compose
- Diesel CLI: `cargo install diesel_cli --no-default-features --features postgres`

## Setup

1. Create a `.env` file in the root directory with the following variables:

```env
POSTGRES_USER=your_user
POSTGRES_PASSWORD=your_password
POSTGRES_DB=rusteze
DB_PORT=5432
DATABASE_URL=postgresql://your_user:your_password@localhost:5432/rusteze
```

2. Start the PostgreSQL database:

```sh
docker-compose up -d
```

3. Run database migrations:

```sh
diesel migration run
```

## Building

Build the entire workspace:

```sh
cargo build
```

Build the web server specifically:

```sh
cargo build -p web-server
```

## Running

Start the web server:

```sh
cargo run -p web-server
```

The server will be available at `http://127.0.0.1:8080`

## Database Schema

The application uses two main tables:

- **users**: User accounts with authentication
  - `id` (UUID, Primary Key)
  - `username` (VARCHAR, Unique)
  - `password_hash` (VARCHAR)
  - `created_at` (TIMESTAMP)

- **devices**: IoT or hardware devices
  - `id` (UUID, Primary Key)
  - `serial_number` (VARCHAR)
  - `device_type` (VARCHAR)
  - `status` (VARCHAR)
  - `last_uplink` (TIMESTAMP)
  - `owner_id` (UUID, Foreign Key to users)

## Technologies

- **Actix-web** - Web framework
- **Diesel** - ORM and query builder
- **PostgreSQL** - Database
- **Serde** - Serialization/deserialization
- **UUID** - Unique identifier generation
- **Chrono** - Date and time handling

## Development

To add new migrations:

```sh
diesel migration generate migration_name
```

To regenerate the schema after migration changes:

```sh
diesel migration run
```

The schema will be automatically updated in [crates/libs/lib-core/src/schema.rs](crates/libs/lib-core/src/schema.rs).
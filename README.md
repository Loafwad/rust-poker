# Poker 🃏

A web-based poker hand dealer, evaluator, and comparator built with Rust. Deal random 5-card hands, evaluate their rank (high card through royal flush), compare two hands head-to-head, and browse your hand history, all through an interactive HTML UI powered by HTMX.

## Tech Stack

- **[Actix Web](https://actix.rs/)** - HTTP server
- **[Maud](https://maud.lambda.xyz/)** - Compile-time HTML templating
- **[HTMX](https://htmx.org/)** - Interactive frontend without JavaScript
- **[SQLx](https://github.com/launchbadge/sqlx)** + **PostgreSQL** - Persistent hand history
- **[Just](https://github.com/casey/just)** - Task runner

## Features

- **Deal** - Generate a random 5-card poker hand from a full 52-card deck
- **Evaluate** - Classify hands into one of 10 standard poker rankings:
  - High Card, One Pair, Two Pair, Three of a Kind, Straight, Flush, Full House, Four of a Kind, Straight Flush, Royal Flush
- **Compare** - Determine the winner between two hands with full tiebreaker logic (kickers, pair ranks, etc.)
- **History** - View all previously dealt hands stored in PostgreSQL

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)
- [Docker](https://docs.docker.com/get-docker/) (for the PostgreSQL database)
- [Just](https://github.com/casey/just#installation) (task runner, optional - you can run the commands manually)

## Getting Started

### 1. Start the database

```sh
just start-db
```

This runs a PostgreSQL container on port 5432 with the migrations in `migrations/` applied automatically on startup.

### 2. Run the application

```sh
just run
```

The server starts at http://localhost:8080.

### 3. Run the tests

```sh
just test
```

## API Endpoints

| Method | Path       | Description                              |
|--------|------------|------------------------------------------|
| GET    | `/`        | Main page with deal, compare & history UI |
| POST   | `/deal`    | Deal a random hand and persist it        |
| GET    | `/history` | Retrieve all previously dealt hands      |
| POST   | `/compare` | Compare two hands and return the winner  |

## Project Structure

```
├── Cargo.toml              # Dependencies & project metadata
├── Dockerfile              # Multi-stage production build
├── Justfile                # Task runner recipes
├── migrations/
│   ├── 00_init.sql         # (empty placeholder)
│   └── 01_hands.sql        # hands table & hand_kind enum
└── src/
    ├── main.rs             # Actix Web server setup & routing
    ├── game.rs             # Card, Hand, Rank, Suit types & hand evaluation
    ├── comparison.rs       # Hand comparison & tiebreaker logic
    └── index.rs            # Route handlers & Maud HTML templates
```

## Docker

Build and run the application in a container:

```sh
docker build -t poker .
docker run -p 8080:8080 poker
```

> **Note:** The containerized app expects a PostgreSQL instance accessible at the configured `DATABASE_URL`.

## License

This project is unlicensed - use it however you like.
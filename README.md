# themata-api

REST API for [Themata](https://themata.app) — a platform for sharing and discovering themes. Built with Rust ([Axum](https://github.com/tokio-rs/axum)), backed by PostgreSQL and Redis.

## Features

- Theme CRUD (create, read, update, delete, list, count)
- Likes (like, unlike, liked state)
- GitHub OAuth login with JWT authentication
- Anonymous view counting with a Redis-backed background aggregator
- OpenAPI-ish JSON schema endpoint (`/schema`)
- Prometheus metrics (`/metrics`)

## Requirements

- Rust (edition 2024)
- PostgreSQL
- Redis

## Configuration

Copy `.env.example` to `.env` and fill in the values:

| Variable                           | Description                                  |
| ---------------------------------- | -------------------------------------------- |
| `THEMATA_LISTEN_ADDR`              | Listen address (default `localhost:8081`)    |
| `DATABASE_URL`                     | PostgreSQL connection string                 |
| `THEMATA_REDIS_URL`                | Redis connection string                      |
| `THEMATA_JWT_SECRET`               | Secret used to sign JWTs                     |
| `THEMATA_GITHUB_APP_NAME`          | GitHub OAuth app name                        |
| `THEMATA_GITHUB_CLIENT_ID`         | GitHub OAuth client ID                       |
| `THEMATA_GITHUB_CLIENT_SECRET`     | GitHub OAuth client secret                   |
| `THEMATA_METRICS_SECRET_HASH`      | XxHash3_64 hash of the metrics access secret |
| `THEMATA_AGGREGATE_VIEWS_INTERVAL` | View aggregation loop interval in ms         |

### Nix

A dev shell is provided via `flake.nix`:

```sh
nix develop -c <optional-shell>
```

# Midnight Notify

Axum-based multi-tenant notification and reminder service (open-source core).

## Included Scaffold

- Full module structure (routes, handlers, services, providers, workers, db, utils)
- Health + ready endpoints
- Messages/templates/logs/webhook routes
- Postgres + Redis via Docker Compose
- Initial SQL migration
- Mock provider trait setup

## Quick Start

1. Copy env:
   - `cp .env.example .env`
2. Start infra:
   - `docker compose up -d`
3. Run app:
   - `cargo run`

## Endpoints (scaffold)

- `GET /health`
- `GET /ready`
- `POST /messages/send`
- `POST /messages/schedule`
- `GET /messages/:id/status`
- `POST /webhooks/provider`
- `POST /templates`
- `GET /templates`
- `GET /templates/:id`
- `PATCH /templates/:id`
- `DELETE /templates/:id`
- `GET /logs/messages`
- `GET /logs/messages/:id/events`

## Next Implementation Steps

1. API key middleware
2. SQLx inserts/queries for messages/templates
3. Redis enqueue + sender worker
4. Webhook signature validation + status updates

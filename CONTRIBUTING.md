# Contributing to Midnight Notify

Thanks for contributing to **Midnight Notify**.  
This project is an **Axum-based, multi-tenant notification + reminder service** with an open-source core. This guide explains how to set up the project locally, propose changes, and get PRs merged smoothly.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Ways to Contribute](#ways-to-contribute)
- [Development Setup](#development-setup)
- [Scaffolded Endpoints](#scaffolded-endpoints)
- [Project Overview](#project-overview)
- [Workspace / Module Layout](#workspace--module-layout)
- [Build, Test, and Lint](#build-test-and-lint)
- [Coding Standards](#coding-standards)
- [Adding or Changing Features](#adding-or-changing-features)
- [Database Migrations](#database-migrations)
- [Providers and Webhooks](#providers-and-webhooks)
- [Documentation Changes](#documentation-changes)
- [Commit Message Rules](#commit-message-rules)
- [Pull Request Checklist](#pull-request-checklist)
- [Security](#security)
- [Getting Help](#getting-help)

---

## Code of Conduct

Be respectful, constructive, and focused on technical outcomes.  
No harassment, no hate, no drama — just shipping good software together.

---

## Ways to Contribute

You can help by:

- Fixing bugs and edge cases
- Implementing missing core steps (see [Next Implementation Steps](#next-implementation-steps-core-roadmap))
- Improving request validation and error handling
- Adding tests (unit + integration)
- Improving docs, examples, and diagrams
- Adding providers (email/SMS/WhatsApp/etc.) behind the provider trait
- Hardening webhooks (signature validation, replay protection)

If you’re planning a bigger change (new architecture, major refactor, or new storage/queue design), open an issue first so we can align on approach.

---

## Development Setup

### Prerequisites

- Rust stable toolchain (Rust 2021 edition)
- `cargo`
- `git`
- Docker + Docker Compose

### Quick Start

```bash
# 1) Copy env
cp .env.example .env

# 2) Start infra (Postgres + Redis)
docker compose up -d

# 3) Run app
cargo run
```

---

## Scaffolded Endpoints

### Health and readiness

- `GET /health`
- `GET /ready`

### Messages

- `POST /messages/send`
- `POST /messages/schedule`
- `GET /messages/:id/status`

### Webhooks

- `POST /webhooks/provider`

### Templates

- `POST /templates`
- `GET /templates`
- `GET /templates/:id`
- `PATCH /templates/:id`
- `DELETE /templates/:id`

### Logs

- `GET /logs/messages`
- `GET /logs/messages/:id/events`

---

## Project Overview

Midnight Notify is designed for:

- **Multi-tenancy** (tenants isolated by API key + tenant-aware data model)
- **Provider abstraction** (mock provider trait scaffolded; more providers can plug in)
- **Queue + worker model** (Redis enqueue + sender worker planned)
- **Webhook-driven delivery updates** (provider status updates planned)
- **Templates** (create, update, delete, fetch templates)
- **Logs / events** (track message lifecycle)

---

## Workspace / Module Layout

The repository includes a full module structure for:

- `routes/` — route registration and path grouping
- `handlers/` — HTTP handlers, request/response shaping
- `services/` — business logic (tenant-aware)
- `providers/` — provider traits + provider implementations (mock scaffolded)
- `workers/` — background workers (sender/dispatcher)
- `db/` — SQLx pool, queries, models, migrations glue
- `utils/` — helpers, shared types, error handling, validation, etc.

**Rule of thumb:**

- Handlers should stay thin (request parsing + calling services).
- Services contain the “real logic”.
- Providers are replaceable implementations behind traits.
- Workers do async processing and should be idempotent.

---

## Build, Test, and Lint

Run these from the repository root.

### Compile check

```bash
cargo check
```

### Build (workspace)

```bash
cargo build --workspace
```

### Run tests

```bash
cargo test --workspace
```

### Format

```bash
cargo fmt --all
```

### Lint (CI-quality gate)

```bash
cargo clippy --workspace --all-targets -- -D warnings
```

Before opening a PR, at minimum run:

```bash
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

---

## Coding Standards

- Rust 2021
- Use `rustfmt` output (don’t fight formatting)

### Naming

- files/modules/functions: `snake_case`
- types/traits/enums: `PascalCase`
- constants/statics: `SCREAMING_SNAKE_CASE`

### General guidance

- Keep modules small and public APIs intentional
- Avoid unnecessary cloning; prefer clean ownership/borrowing
- Prefer structured errors (typed errors) and consistent HTTP error responses
- Validate inputs at the boundary (request DTOs), not deep in the DB layer

---

## Adding or Changing Features

Keep changes scoped and avoid unrelated refactors in the same PR.

### Next Implementation Steps (Core Roadmap)

These are the top priority missing pieces:

#### 1) API key middleware

- Authenticate requests via API key
- Resolve tenant identity (`tenant_id`) from key
- Enforce tenant-aware access across handlers/services

#### 2) SQLx inserts/queries for messages/templates

- Replace any placeholders with real DB ops
- Ensure all queries are tenant-scoped
- Add indexes where needed (`tenant_id`, `message_id`, `status`, `created_at`, `scheduled_at`)

#### 3) Redis enqueue + sender worker

- Enqueue scheduled/ready-to-send messages
- Worker pops jobs and dispatches through provider
- Ensure idempotency (avoid double-sends)
- Record delivery attempts + status transitions

#### 4) Webhook signature validation + status updates

- Verify provider signatures
- Prevent replay (timestamp + nonce if available)
- Update message status + append log events

### Feature PR Expectations

- Add tests for new behavior
- Update docs if endpoints or env/config changes
- Include error cases (invalid inputs, missing tenant, provider failures)
- Prefer incremental PRs: one feature per PR

---

## Database Migrations

- Keep migrations additive and reversible when possible.
- Avoid destructive changes without a migration plan.

Prefer:

- `tenant_id` columns for isolation
- `created_at`, `updated_at`
- lifecycle fields for messages (`status`, `provider_message_id`, `scheduled_at`, `sent_at`, `delivered_at`, `failed_at`)

If you change schema:

- include a migration file
- update any models / SQLx queries
- add tests (at least one integration test if practical)

---

## Providers and Webhooks

### Providers

Providers should implement the provider trait and be:

- deterministic and testable
- safe by default (timeouts, retries policy in worker/service layer)
- isolated from HTTP handlers (handlers should never speak provider directly)

If adding a provider:

- keep credentials in env
- add minimal docs for required env vars
- add mock/test coverage where possible

### Webhooks

Webhook handlers should:

- validate signature (and reject if invalid)
- parse payload safely
- map provider statuses into internal statuses
- append message events to logs (auditable history)
- be idempotent (same webhook delivered twice shouldn’t corrupt state)

---

## Documentation Changes

If behavior changes, update:

- `README.md` for user-facing usage changes
- any `docs/` pages (if present)
- endpoint examples when request/response shapes change
- env docs when adding new config vars

Docs-only PRs are welcome.

---

## Commit Message Rules

Use **Conventional Commits**:

- `feat: ...`
- `fix: ...`
- `docs: ...`
- `refactor: ...`
- `test: ...`
- `chore: ...`

Scopes are encouraged:

- `feat(api): add api key middleware`
- `fix(worker): prevent double send on retry`
- `docs(db): document migrations workflow`

Keep commits focused.

---

## Pull Request Checklist

Before requesting review:

- [ ] PR has a clear summary and motivation
- [ ] Changes are scoped (no drive-by refactors)
- [ ] `cargo test --workspace` passes locally
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes locally
- [ ] Added/updated tests for new behavior
- [ ] Docs updated (README/docs/examples) if user-facing behavior changed
- [ ] Tenant isolation is preserved in queries and services
- [ ] Error handling is consistent and safe (no leaking secrets)

### PR Description Template (recommended)

```md
## Summary

What changed and why.

## Changes
- Item 1
- Item 2

## Validation
- [x] cargo test --workspace
- [x] cargo clippy --workspace --all-targets -- -D warnings

## Docs
- [x] Updated docs/README (or N/A with reason)
```

---

## Security

If you find a security issue:

- Do **not** post full exploit details publicly.
- Open a security-focused issue with minimal details, or contact the maintainers privately (if a channel exists).
- Never commit secrets (API keys, tokens, private URLs) to the repo.

---

## Getting Help

- Open a GitHub issue for bugs or feature requests
- Use issues for design discussions (especially for queueing, multi-tenancy, provider standards)
- If something is unclear in the scaffold, ask — we’ll align and keep it moving


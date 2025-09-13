# Selfie Backend

![Selfie Logo](assets/logo.png) <!-- Placeholder for logo; add via scaffold -->

Selfie Backend is a high-performance, scalable microservices architecture built in Rust, designed to power a social media platform rivaling Instagram and Facebook. This monorepo uses Cargo workspaces for shared libraries and independent service binaries, emphasizing safety, concurrency (via Tokio), and modularity. Services communicate via gRPC (Tonic) and REST (Axum), with event-driven patterns via Kafka for real-time features like feeds, notifications, and stories.

Powered by Rust 1.90.0 (stable release with enhanced async and safety features), Axum 0.8 (ergonomic HTTP framework with Tower integration), Tonic 0.14 (production gRPC with streaming support), and Sea-ORM 1.1 (async ORM for Postgres), the backend handles massive scale: personalized feeds, E2E-encrypted chats, ML-powered recommendations, and monetization via payments/ads.

## 🚀 Quick Start

### Prerequisites
- **Rust 1.90.0+** (via rustup)
- **Docker & Docker Compose** (for local services: Postgres, Redis, Kafka)
- **Kubernetes 1.33+** (EKS or minikube for deployment)
- **Helm 3.18+** (for charts)
- **Terraform 1.14+** (for IaC)
- **PostgreSQL 16+, Redis 7+, Kafka 3.8+** (via Docker or cloud)

### Setup
1. Clone and enter the repo:
   ```
   git clone https://github.com/qiss22/selfie
   cd selfie-backend
   ```

2. Install dependencies and build:
   ```
   rustup toolchain install 1.90.0
   cargo check  # Workspace check
   ```

3. Run local dev stack:
   ```
   docker-compose up -d  # Starts DBs, Kafka, Jaeger
   ```

4. Build and run a service (e.g., auth-service):
   ```
   cd services/auth-service
   cargo run  # Binds to localhost:3000
   ```

5. Full workspace build/test:
   ```
   cargo build --workspace
   cargo test --workspace
   ```

For production deploy, see Deployment section. CI/CD via GitHub Actions in `.github/workflows/ci.yml`.

## 🏗️ Project Structure

```
selfie-backend/
├── Cargo.toml              # Workspace: shared libs + services
├── shared/                 # Library crates (reuse across services)
│   ├── auth-utils/         # JWT/OAuth, middleware
│   ├── db-models/          # Sea-ORM models/migrations
│   ├── proto/              # gRPC .proto + generated code
│   ├── utils/              # Tracing, errors, cache
│   ├── events/             # Kafka schemas (Avro/Serde)
│   └── ml-models/          # Candle/ONNX inference
├── services/               # Binary crates (microservices)
│   ├── auth-service/       # Login/2FA, sessions
│   ├── user-service/       # Profiles, privacy
│   ├── social-graph-service/ # Neo4j connections
│   ├── post-service/       # Posts, comments (GraphQL)
│   ├── feed-service/       # Personalized algo feeds
│   ├── stories-service/    # Ephemeral content (Redis TTL)
│   ├── chat-service/       # Real-time messaging (WebSockets)
│   ├── media-service/      # Uploads/processing (S3/FFmpeg)
│   ├── notification-service/ # Pushes (FCM/APNs)
│   ├── search-service/     # Meilisearch indexing
│   ├── recommendation-service/ # ML suggestions
│   ├── moderation-service/ # AI/human content flags
│   ├── admin-service/      # Ops dashboard (RBAC)
│   ├── payments-service/   # Stripe billing
│   ├── ads-service/        # RTB auctions
│   ├── cdn-edge-service/   # Proxy/caching
│   └── analytics-service/  # ClickHouse metrics
├── docker/                 # Dockerfiles, compose for local
├── helm/                   # K8s charts (umbrella + per-service)
├── infrastructure/         # Terraform IaC (EKS, RDS, MSK)
├── monitoring/             # Prometheus/Grafana/Jaeger/OTel
├── security/               # Policies, audits (cargo-deny)
├── testing/                # Mocks, fixtures, benches
├── scripts/                # Migrations, benchmarks (Rust CLI)
└── .github/workflows/      # CI/CD (fmt, clippy, test, audit)
```

- **Shared Crates**: Centralize deps (e.g., `tracing` for observability, `sea-orm` for DB models).
- **Services**: Independent deploys; each with `src/main.rs` (Tokio runtime), handlers, repos, events.
- **Async Everywhere**: Tokio for concurrency, Tower for middleware (rate-limit, tracing).
- **Testing**: Unit (cargo test), integration (testcontainers), benches (Criterion).

## 🛠️ Tech Stack

| Layer | Tech | Why? |
|-------|------|------|
| **Language** | Rust 1.90.0 | Memory safety, zero-cost abstractions, async excellence for high-throughput services. |
| **HTTP/gRPC** | Axum 0.8 / Tonic 0.14 | Ergonomic routing (Axum), streaming RPC (Tonic) with HTTP/2. |
| **ORM/DB** | Sea-ORM 1.1 / Postgres 16 | Async queries, relations; sharded for scale. |
| **Events** | rdkafka / Kafka 3.8 | Reliable pub/sub (Avro schemas). |
| **Cache/Queue** | Redis 7 / bb8 | TTL stories, rate-limiting. |
| **ML** | Candle 0.6 / ONNX | Lightweight inference (recommendations, moderation). |
| **Deploy** | Helm 3.18 / K8s 1.33 (EKS) | Autoscaling (HPA), service mesh ready. |
| **IaC** | Terraform 1.14 | Cloud provisioning (VPC, RDS, MSK). |
| **Observability** | OpenTelemetry / Prometheus/Grafana/Jaeger | Unified traces/metrics/logs; SLOs. |

## 🌟 Key Features

- **Microservices Modularity**: Independent scaling (e.g., feed-service on GPU for ML).
- **Real-Time**: WebSockets (chat/stories), Kafka fanout (notifications on posts).
- **Security**: mTLS (Tonic), JWT (auth-utils), OPA policies; cargo-deny audits.
- **Scalability**: Horizontal pods (HPA), sharded DB (Postgres/Citurs), edge caching (CDN).
- **Monetization**: Stripe webhooks (payments), RTB ads integration.
- **Observability**: OTEL traces across services, Grafana dashboards for latencies/SLOs.
- **Offline/Edge**: Signed S3 URLs (media), local ML (moderation).

## 🚀 Development Workflow

1. **Shared Libs**: `cd shared/auth-utils && cargo test` (e.g., JWT unit tests).
2. **Service Dev**: `cd services/auth-service && cargo run` (hot-reload with cargo-watch).
3. **Migrations**: `./scripts/migrate.rs up` (Sea-ORM CLI).
4. **Test**: `cargo test --workspace` (80%+ coverage); integration with testcontainers.
5. **Benchmark**: `cargo bench` (Criterion for throughput).
6. **Local Stack**: `docker-compose up` + service runs.
7. **Deploy**:
   - Build images: `docker build -t selfie/auth:latest services/auth-service`
   - Helm: `helm upgrade --install selfie ./helm -f values.yaml`
   - Terraform: `terraform apply` (provisions EKS/RDS).

Use `./scripts/benchmark.rs` for load tests. Lint: `cargo fmt && cargo clippy`.

## 📦 Deployment

- **Local**: Docker Compose for DBs/Kafka.
- **Prod**: EKS 1.33 (Terraform), Helm charts (umbrella for all services).
- **CI/CD**: GitHub Actions: Build/push images (multi-arch), deploy to staging/prod.
- **Scaling**: HPA on CPU/Mem, Istio for traffic (mTLS, canary).

Secrets via SealedSecrets; monitoring with Prometheus federation.

## 🔍 Contributing

1. Fork & PR to `main`.
2. Follow [Rust Style Guide](https://doc.rust-lang.org/nightly/style-guide/) + 2025 idioms (async traits, anyhow errors).
3. Run checks: `cargo fmt --check && cargo clippy -- -D warnings && cargo test`.
4. Add benches; aim for 80% coverage.

See `CONTRIBUTING.md` for details. Issues? Open a ticket!

## 📄 License

MIT License. See [LICENSE](LICENSE).

## 🔗 Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Axum Docs](https://docs.rs/axum/latest/axum/)
- [Tonic gRPC Guide](https://github.com/hyperium/tonic)
- Community: [r/rust](https://www.reddit.com/r/rust/) (microservices discussions)

---

*Last updated: September 13, 2025.*

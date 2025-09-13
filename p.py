#!/usr/bin/env python3
"""
Robust Python script to scaffold the enhanced Rust microservices backend monorepo for Selfie app.

This script generates the full monorepo structure as outlined, including:
- Root Cargo.toml workspace, .env.example, rust-toolchain.toml.
- Shared library crates (auth-utils, db-models, proto, utils, events, ml-models).
- Services as binary crates with src subdirs, migrations, tests, benches, Dockerfile.
- Docker, Helm, Terraform stubs.
- Monitoring, security, infrastructure, testing, scripts, CI configs.

Requires Python 3.8+. Run with: python scaffold_selfie_backend.py --root selfie-backend

Options:
--root: Root directory name (default: selfie-backend)
--version: Project version (default: 0.1.0)

Uses templates based on Rust 1.81.0 (stable as of Sep 2025), Axum 0.7, Tonic 0.12, Sea-ORM 1.1.
Inspired by Cargo workspace best practices for monorepos.
"""

import os
import argparse
import shutil
import subprocess
from pathlib import Path

# Versions (Sep 2025 stable)
RUST_VERSION = "1.81.0"
AXUM_VERSION = "0.7"
TONIC_VERSION = "0.12"
SEA_ORM_VERSION = "1.1"

# Services list
SERVICES = [
    "auth-service", "user-service", "social-graph-service", "post-service",
    "feed-service", "stories-service", "chat-service", "media-service",
    "notification-service", "search-service", "recommendation-service",
    "moderation-service", "admin-service", "payments-service",
    "ads-service", "cdn-edge-service", "analytics-service"
]

# Shared libs
SHARED_LIBS = [
    "auth-utils", "db-models", "proto", "utils", "events", "ml-models"
]

# Templates
ROOT_CARGO_TOML = """[workspace]
members = [
    "shared/{shared_libs}",
    "services/*"
]
resolver = "2"

[workspace.dependencies]
tokio = {{ version = "1.40", features = ["full"] }}
axum = {{ version = "{axum_version}", features = ["macros"] }}
tonic = {{ version = "{tonic_version}", features = ["tls"] }}
sea-orm = {{ version = "{sea_orm_version}", features = ["sqlx-postgres", "runtime-tokio-rustls", "macros"] }}
tracing = "0.1"
serde = {{ version = "1.0", features = ["derive"] }}
"""

RUST_TOOLCHAIN_TOML = f"""[toolchain]
channel = "{RUST_VERSION}"
components = ["rustfmt", "clippy"]
"""

ENV_EXAMPLE = """# Example env vars
DB_URL=postgres://user:pass@localhost/selfie
REDIS_URL=redis://localhost:6379
KAFKA_BOOTSTRAP=localhost:9092
JWT_SECRET=your_secret_key_here
OTEL_EXPORTER=jaeger
"""

RUSTFMT_TOML = """edition = "2021"
max_width = 100
"""

DOCKER_BASE = """FROM rust:{rust_version}-slim as chef
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust:{rust_version}-slim as cacher
WORKDIR /app
COPY recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:{rust_version}-slim as builder
WORKDIR /app
COPY . .
COPY --from=cacher /app/target target
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/auth-service /usr/local/bin/
CMD ["auth-service"]
""".format(rust_version=RUST_VERSION)

SERVICE_CARGO_TOML_TEMPLATE = """[package]
name = "{service_name}"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio.workspace = true
axum.workspace = true
sea-orm.workspace = true
tracing.workspace = true
serde.workspace = true
# Add more as needed

[[bin]]
name = "{service_name}"
path = "src/main.rs"
"""

LIB_CARGO_TOML_TEMPLATE = """[package]
name = "{lib_name}"
version = "0.1.0"
edition = "2021"

[dependencies]
serde.workspace = true
# Specific deps
"""

MAIN_RS_TEMPLATE = """use axum::{{routing::get, Router}};
use std::net::SocketAddr;
use tracing_subscriber;

#[tokio::main]
async fn main() {{
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(root));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running on {{}}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}}

async fn root() -> &'static str {{
    "Hello, Selfie Backend!"
}}
"""

LIB_RS_TEMPLATE = """pub mod models;
pub mod errors;

pub use models::*;
pub use errors::*;
"""

def mkdir_p(path: Path):
    path.mkdir(parents=True, exist_ok=True)

def write_file(path: Path, content: str):
    path.write_text(content)
    print(f"Created {path}")

def generate_shared_lib(base_dir: Path, lib_name: str):
    lib_dir = base_dir / "shared" / lib_name
    mkdir_p(lib_dir / "src")
    write_file(lib_dir / "Cargo.toml", LIB_CARGO_TOML_TEMPLATE.format(lib_name=lib_name))
    write_file(lib_dir / "src" / "lib.rs", LIB_RS_TEMPLATE)
    # Add specific subdirs if needed, e.g., for proto
    if lib_name == "proto":
        mkdir_p(lib_dir / "proto")
        (lib_dir / "build.rs").write_text(
            """fn main() {{
    tonic_build::compile_protos("proto/selfie.proto").unwrap();
}}"""
        )

def generate_service(base_dir: Path, service_name: str):
    svc_dir = base_dir / "services" / service_name
    mkdir_p(svc_dir / "src" / "api")
    mkdir_p(svc_dir / "src" / "handlers")
    mkdir_p(svc_dir / "src" / "repository")
    mkdir_p(svc_dir / "src" / "service")
    mkdir_p(svc_dir / "src" / "middleware")
    mkdir_p(svc_dir / "migrations")
    mkdir_p(svc_dir / "tests")
    mkdir_p(svc_dir / "benches")

    write_file(svc_dir / "Cargo.toml", SERVICE_CARGO_TOML_TEMPLATE.format(service_name=service_name))
    write_file(svc_dir / "src" / "main.rs", MAIN_RS_TEMPLATE)
    write_file(svc_dir / "Dockerfile", DOCKER_BASE.format(service_name=service_name))

def generate_docker(base_dir: Path):
    docker_dir = base_dir / "docker"
    mkdir_p(docker_dir)
    write_file(docker_dir / "Dockerfile.base", DOCKER_BASE.format(rust_version=RUST_VERSION, service_name="base"))
    # docker-compose.yml stub
    write_file(docker_dir / "docker-compose.yml", 
        """version: '3.8'
services:
  postgres:
    image: postgres:16
    environment:
      POSTGRES_DB: selfie
  redis:
    image: redis:7-alpine
  kafka:
    image: confluentinc/cp-kafka:7.5.0
""")

def generate_helm(base_dir: Path):
    helm_dir = base_dir / "helm"
    mkdir_p(helm_dir / "templates")
    write_file(helm_dir / "Chart.yaml", 
        """apiVersion: v2
name: selfie
description: Selfie Backend Helm Chart
version: 0.1.0
""")
    write_file(helm_dir / "values.yaml", 
        """replicaCount: 1
resources: {{}}
""")
    write_file(helm_dir / "templates" / "deployment.yaml", 
        """apiVersion: apps/v1
kind: Deployment
metadata:
  name: {{ .Release.Name }}-deployment
spec:
  replicas: {{ .Values.replicaCount }}
  template:
    spec:
      containers:
      - name: {{ .Release.Name }}
        image: "{{ .Values.image.repository }}:{{ .Values.image.tag }}"
""")

def generate_infra(base_dir: Path):
    infra_dir = base_dir / "infrastructure" / "terraform"
    mkdir_p(infra_dir)
    write_file(infra_dir / "main.tf", 
        """terraform {{
  required_providers {{
    aws = {{
      source = "hashicorp/aws"
      version = "~> 5.0"
    }}
  }}
}}

provider "aws" {{
  region = var.region
}}

resource "aws_vpc" "selfie" {{
  cidr_block = "10.0.0.0/16"
}}
""")
    write_file(infra_dir / "variables.tf", 
        """variable "region" {{
  description = "AWS region"
  type = string
  default = "us-east-1"
}}
""")

def generate_monitoring(base_dir: Path):
    mon_dir = base_dir / "monitoring" / "prometheus"
    mkdir_p(mon_dir)
    write_file(mon_dir / "prometheus.yml", 
        """global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'selfie-services'
    static_configs:
      - targets: ['localhost:9090']
""")

def generate_security(base_dir: Path):
    sec_dir = base_dir / "security"
    mkdir_p(sec_dir)
    write_file(sec_dir / "cargo-deny.toml", 
        """[policies]
ban-dev = ["unsafe"]

[advisories]
ignore = []
""")

def generate_testing(base_dir: Path):
    test_dir = base_dir / "testing"
    mkdir_p(test_dir)
    write_file(test_dir / "README.md", "# Testing fixtures and mocks")

def generate_scripts(base_dir: Path):
    scripts_dir = base_dir / "scripts"
    mkdir_p(scripts_dir)
    (scripts_dir / "migrate.rs").write_text(
        """fn main() {{
    // Sea-ORM migration runner
    println!("Run migrations...");
}}"""
    )

def generate_ci(base_dir: Path):
    ci_dir = base_dir / ".github" / "workflows"
    mkdir_p(ci_dir)
    write_file(ci_dir / "ci.yml", 
        """name: CI
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@stable
    - run: cargo fmt --check
    - run: cargo clippy -- -D warnings
    - run: cargo test
    - run: cargo audit
""")

def main():
    parser = argparse.ArgumentParser(description="Scaffold Selfie Rust Backend")
    parser.add_argument("--root", default="selfie-backend", help="Root dir name")
    parser.add_argument("--version", default="0.1.0", help="Project version")
    args = parser.parse_args()
    
    root_dir = Path(args.root)
    if root_dir.exists():
        print(f"Error: {root_dir} already exists. Remove or choose another name.")
        return
    
    print(f"Scaffolding Selfie Backend in {root_dir}...")
    
    # Root files
    mkdir_p(root_dir)
    shared_libs_str = ', '.join([f'"{lib}"' for lib in SHARED_LIBS])
    write_file(root_dir / "Cargo.toml", ROOT_CARGO_TOML.format(shared_libs=shared_libs_str, axum_version=AXUM_VERSION, tonic_version=TONIC_VERSION, sea_orm_version=SEA_ORM_VERSION))
    write_file(root_dir / "rust-toolchain.toml", RUST_TOOLCHAIN_TOML)
    write_file(root_dir / ".env.example", ENV_EXAMPLE)
    write_file(root_dir / ".rustfmt.toml", RUSTFMT_TOML)
    write_file(root_dir / "README.md", "# Selfie Backend\nRust microservices monorepo.")
    
    # Init Cargo workspace
    try:
        subprocess.run(["cargo", "init", "--lib"], cwd=root_dir, check=True)
        if (root_dir / "src").exists():
            shutil.rmtree(root_dir / "src")  # Remove dummy lib
    except subprocess.CalledProcessError:
        print("Cargo workspace already initialized, continuing...")
    
    # Shared libs
    shared_dir = root_dir / "shared"
    mkdir_p(shared_dir)
    for lib in SHARED_LIBS:
        generate_shared_lib(shared_dir, lib)
    
    # Services
    services_dir = root_dir / "services"
    mkdir_p(services_dir)
    for svc in SERVICES:
        generate_service(services_dir, svc)
    
    # Other dirs
    generate_docker(root_dir)
    generate_helm(root_dir)
    generate_infra(root_dir)
    generate_monitoring(root_dir)
    generate_security(root_dir)
    generate_testing(root_dir)
    generate_scripts(root_dir)
    generate_ci(root_dir)
    
    # .gitignore
    gitignore_content = """# Generated by Cargo
# will have compiled files and executables
debug/
target/

# Remove Cargo.lock from gitignore if creating an executable, leave it for libraries
# More information here https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html
Cargo.lock

# These are backup files generated by rustfmt
*.rs.bk

# MSVC Windows gunk
*.pdb

# Test binaries
/target

# Env files
.env

# Helm/Terraform
.terrafrom/
charts/

# Docker
docker-compose.override.yml
"""
    write_file(root_dir / ".gitignore", gitignore_content)
    
    print("Scaffolding complete! Run `cd {root_dir} && cargo check` to verify.".format(root_dir=root_dir))
    print("For full setup: Install deps, run `cargo build` for shared, then per-service.")

if __name__ == "__main__":
    main()

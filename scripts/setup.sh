#!/usr/bin/env bash
# Inicializa el proyecto: .env, PostgreSQL y esquema nooks_cookbook.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

if [[ ! -f .env ]]; then
  cp .env.example .env
  echo "→ Creado .env desde .env.example"
fi

# shellcheck disable=SC1091
set -a
source .env
set +a

echo "→ Levantando PostgreSQL (Docker)..."
docker compose up -d --wait

schema_exists() {
  docker compose exec -T db psql -U nooks -d nooks_cookbook -tAc \
    "SELECT 1 FROM information_schema.schemata WHERE schema_name = 'nooks_cookbook'" \
    | grep -q 1
}

if schema_exists; then
  echo "→ Esquema nooks_cookbook ya está aplicado."
else
  echo "→ Aplicando esquema desde db/nooks_cookbook.sql..."
  docker compose exec -T db psql -U nooks -d nooks_cookbook < db/nooks_cookbook.sql
  echo "→ Esquema aplicado."
fi

catalog_seeded() {
  docker compose exec -T db psql -U nooks -d nooks_cookbook -tAc \
    "SELECT 1 FROM nooks_cookbook.ingrediente LIMIT 1" \
    | grep -q 1
}

if catalog_seeded; then
  echo "→ Catálogo (ingredientes/utensilios) ya tiene datos."
else
  echo "→ Cargando catálogo inicial (db/seed_data.sql)..."
  docker compose exec -T db psql -U nooks -d nooks_cookbook < db/seed_data.sql
fi

logros_seeded() {
  docker compose exec -T db psql -U nooks -d nooks_cookbook -tAc \
    "SELECT 1 FROM nooks_cookbook.logro LIMIT 1" \
    | grep -q 1
}

if logros_seeded; then
  echo "→ Logros ya están cargados."
else
  echo "→ Cargando logros iniciales (db/seed_logros.sql)..."
  docker compose exec -T db psql -U nooks -d nooks_cookbook < db/seed_logros.sql
fi

echo ""
echo "Listo. Puedes ejecutar: cargo run"
echo "DATABASE_URL=${DATABASE_URL}"

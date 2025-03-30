dev: install-cargo-watch dev-up
  cargo watch -x run

dev-up: copy-env-dev install-sqlx-cli
  docker compose up -d
  # This command ensures postgres is ready to accept connections
  docker compose exec postgres /bin/bash -c "until pg_isready; do sleep 1; done"
  sqlx db create
  sqlx migrate run

# Remove just the containers
dev-down:
  docker compose down

# Remove all containers & volumes
# Run only if you want a fresh db instance
dev-clean:
  docker compose down --volumes --remove-orphans 

install-sqlx-cli:
  cargo install --locked sqlx-cli

install-cargo-watch:
  cargo install --locked cargo-watch

copy-env-dev:
  cp .env.dev .env

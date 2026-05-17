.PHONY: setup reset-db run check db-logs

setup:
	@./scripts/setup.sh

reset-db:
	@./scripts/reset-db.sh

run:
	@cargo run

check:
	@cargo check

db-logs:
	@docker compose logs -f db

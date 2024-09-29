install-sqlx-cli:
	cargo install sqlx-cli --no-default-features --features postgres

run-db-migrations:
	export $$(cat ./../secrets/db_events.env | xargs) && sqlx migrate run --source src/db/migrations

revert-db-migrations:
	export $$(cat ./../secrets/db_events.env | xargs) && sqlx migrate revert --source src/db/migrations

run:
	export $$(cat ./../secrets/db_events.env | xargs) && RUST_BACKTRACE=1 cargo run
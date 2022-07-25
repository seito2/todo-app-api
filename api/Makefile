migrate:
	diesel migration run --database-url postgres://postgres:postgres@db/postgres
	@make sync-schema

sync-schema:
	diesel print-schema > src/schema.rs --database-url postgres://postgres:postgres@db/postgres

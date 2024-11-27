.PHONY: diesel-setup
diesel-setup: ## Setups database with the diesel CLI
	@echo Setting up database with diesel
	diesel setup
	@echo 'Setup complete'

.PHONY: create-migration
create-migration: ## Generates a migration script
	@echo 'Running migration on $(migration-name)'
	diesel migration generate $(migration-name)
	@echo 'Migration on $(migration-name) complete'

.PHONY: run-migrations
run-migrations: ## Runs migrations
	@echo 'Running migrations...'
	diesel migration run
	@echo 'Done running migrations'

.PHONY: redo-migrations
redo-migrations: ## Redoes migrations
	diesel migration redo
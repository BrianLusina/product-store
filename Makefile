.PHONY: diesel-setup
diesel-setup: ## Setups database with the diesel CLI
    @echo 'Setting up database with diesel'
    diesel setup
    @echo 'Setup complete'

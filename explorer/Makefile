.PHONY: help deps run create_env

help:
	@echo "deps - Installing Elixir code dependencies"
	@echo "run - Starting the Elixir backend server"
	@echo "create_env - Create environment variables files from .env.example to a new .env file"
	@echo "help - Show this help"

deps:
	@echo "Installing Elixir code dependencies"
	@mix setup

dashboard_for_devnet: 
	@mix setup
	@cp .env.dev .env
	./start.sh

run: 
	@echo "Starting the Elixir backend server"
	./start.sh

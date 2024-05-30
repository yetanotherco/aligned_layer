.PHONY: help deps run create_env

help:
	@echo "deps - Installing Elixir code dependencies"
	@echo "run - Starting the Elixir backend server"
	@echo "create_env - Create environment variables files from .env.example to a new .env file"
	@echo "help - Show this help"

deps:
	@echo "Installing Elixir code dependencies"
	@mix setup

run:
	@echo "Starting the Elixir backend server"
	./start.sh

build_db:
	@docker build -t explorer-postgres-image .

# docker run creates a new container
run_db: clean_db
	@docker run --name explorer-postgres-container -p 5432:5432 explorer-postgres-image

# docker start starts (or continues execution of) an existing container
start_db:
	@docker start explorer-postgres-container

clean_db:
	@docker stop explorer-postgres-container || true
	@docker rm explorer-postgres-container || true

create_env:
	@echo "Creating environment variables files .env",
	@cp .env.example .env
	@echo "Environment variables files created successfully!"

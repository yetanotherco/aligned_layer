# Use the official PostgreSQL image from the Docker Hub
FROM postgres:latest

# Environment variables
ENV POSTGRES_USER=explorer_user
ENV POSTGRES_PASSWORD=explorer_pass
ENV POSTGRES_DB=explorer_db

# Copy the initialization script into the container
COPY init.sql /docker-entrypoint-initdb.d/

# Expose the default PostgreSQL port
EXPOSE 5432

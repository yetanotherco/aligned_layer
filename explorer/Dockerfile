# https://hub.docker.com/_/postgres
FROM postgres:16.3

# Environment variables
ENV POSTGRES_USER=explorer_user
ENV POSTGRES_PASSWORD=explorer_pass
ENV POSTGRES_DB=explorer_db

# Copy the initialization script into the container
COPY init.sql /docker-entrypoint-initdb.d/

# Expose the default PostgreSQL port
EXPOSE 5432

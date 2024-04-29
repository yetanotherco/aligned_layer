# Use the official Go image as a builder stage.
FROM golang:1.22.2 as builder
# Set the working directory inside the container.
WORKDIR /app
# Copy the entire project from the current directory to the working directory inside the container.
COPY . .
# Build the application binary.
# You can specify the main package with the directory containing the 'main.go' file.
RUN CGO_ENABLED=0 GOOS=linux go build -o operator-app ./operator/cmd
# Start from a new stage to create a smaller image.
FROM scratch
# Copy the binary from the builder stage to the current stage.
COPY --from=builder /app/operator-app /operator-app
# Set the binary as the entrypoint of the container.
ENTRYPOINT ["/operator-app"]
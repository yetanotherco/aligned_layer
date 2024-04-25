# Use the official Go base image with a specific version
FROM golang:1.22.2

# Install any necessary dependencies
# For instance, if you require git, curl, or other tools, you would include them here
# If all dependencies are managed by Go modules and you don't need additional tools, you can skip this step

# Set the working directory inside the container
WORKDIR /app

# Copy the go.mod and go.sum files first to leverage Docker cache layers
COPY go.mod go.sum ./

# Download the Go modules
RUN go mod download

# Copy the rest of the code from the host machine to the container
COPY . .

# Since the Makefile assumes the current working directory is at the root where the Makefile is,
# you need to ensure that's where you are running the command from in the container
# So if the Makefile isn't in the `aggregator` directory, you'll need to COPY it to the container's current directory

# Copy the Makefile into the current directory (/app)
COPY ../Makefile .

# Copy other necessary directories and files that the application requires from outside the aggregator directory
# This is assuming that the Docker build context is the root of your project,
# so all paths in these COPY commands are relative to that root
COPY contracts/ ../contracts/
COPY operator/ ../operator/

# Set any environment variables needed by your application
# ENV KEY=VALUE

# Run the Go application via the Makefile target
CMD ["make", "aggregator-start"]

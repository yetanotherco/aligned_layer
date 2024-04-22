package config

import (
	"github.com/Layr-Labs/eigensdk-go/logging"
)

// Config is a struct that holds the configuration for the application
type Config struct {
	Logger logging.Logger
	// Aggregator address, format: host:port
	AggregatorAddress string
}

// New creates a new Config instance
func New() *Config {
	logger, err := logging.NewZapLogger(logging.Development)
	if err != nil {
		panic(err)
	}

	return &Config{
		Logger:            logger,
		AggregatorAddress: "localhost:1234", // TODO: Read from env or config file
	}
}

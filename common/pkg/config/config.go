package config

import (
	"github.com/Layr-Labs/eigensdk-go/logging"
	"os"
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

	aggregatorAddress := os.Getenv("AGGREGATOR_ADDRESS")
	if aggregatorAddress == "" {
		logger.Fatal("AGGREGATOR_ADDRESS environment variable not set")
	}

	return &Config{
		Logger:            logger,
		AggregatorAddress: aggregatorAddress, // TODO: Read from env or config file
	}
}

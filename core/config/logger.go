package config

import (
	"fmt"
	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
	"strings"
)

func NewLogger(loggingLevel string) (sdklogging.Logger, error) {
	logger, err := sdklogging.NewZapLogger(getLoggerLevel(loggingLevel))

	if err != nil {
		fmt.Println("Could not initialize logger")
		return nil, err
	}

	return logger, nil
}

func getLoggerLevel(level string) sdklogging.LogLevel {
	switch strings.ToUpper(level) {
	case "DEVELOPMENT":
		return sdklogging.Development
	case "PRODUCTION":
		return sdklogging.Production
	default:
		return sdklogging.Development
	}
}

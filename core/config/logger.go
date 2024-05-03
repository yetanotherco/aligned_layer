package config

import (
	"fmt"

	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
)

func NewLogger(loggingLevel sdklogging.LogLevel) (sdklogging.Logger, error) {
	logger, err := sdklogging.NewZapLogger(loggingLevel)
	if err != nil {
		fmt.Println("Could not initialize logger")
		return nil, err
	}
	return logger, nil
}

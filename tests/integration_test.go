package tests

import (
	"context"
	"github.com/docker/docker/api/types/container"
	"github.com/docker/go-connections/nat"
	"github.com/stretchr/testify/assert"
	"github.com/testcontainers/testcontainers-go"
	"github.com/testcontainers/testcontainers-go/wait"
	"github.com/yetanotherco/aligned_layer/aggregator/pkg"
	"github.com/yetanotherco/aligned_layer/core/config"
	operator "github.com/yetanotherco/aligned_layer/operator/pkg"
	"os"
	"testing"
)

func TestIntegration(t *testing.T) {
	err := os.Chdir("../")
	assert.Nil(t, err, "Could not change directory to project root")

	// start anvil
	_ = startAnvilTestContainer(t)

	configFilePath := "config-files/config-test.yaml"
	// start aggregator
	aggregator := buildAggregator(t, configFilePath)
	go func() {
		err := aggregator.ServeOperators()
		assert.Nil(t, err, "Could not start aggregator")
	}()

	// start operator
	op := buildOperator(t, configFilePath)
	go func() {
		err := op.Start(context.Background())
		assert.Nil(t, err, "Could not start operator")
	}()

}

func buildAggregator(t *testing.T, configFile string) *pkg.Aggregator {
	aggregatorConfig := config.NewAggregatorConfig(configFile)

	aggregator, err := pkg.NewAggregator(*aggregatorConfig)
	assert.Nil(t, err)

	return aggregator

}

func buildOperator(t *testing.T, configFile string) *operator.Operator {
	operatorConfig := config.NewOperatorConfig(configFile)

	opereator, err := operator.NewOperatorFromConfig(*operatorConfig)
	assert.Nil(t, err)

	return opereator
}

func startAnvilTestContainer(t *testing.T) testcontainers.Container {
	rootDir, err := os.Getwd()
	assert.Nil(t, err, "Could not get current working directory")

	hostPath := rootDir + "/contracts/scripts/anvil/state/alignedlayer-deployed-anvil-state.json"

	ctx := context.Background()
	req := testcontainers.ContainerRequest{
		Image:      "ghcr.io/foundry-rs/foundry:latest",
		Entrypoint: []string{"anvil"},
		Cmd:        []string{"--load-state", "/root/.anvil/state.json", "--port", "8546"},
		HostConfigModifier: func(hostConfig *container.HostConfig) {
			hostConfig.NetworkMode = "host"
			hostConfig.AutoRemove = true

			// map state file to container
			hostConfig.Binds = []string{hostPath + ":/root/.anvil/state.json"}

			// map ports to container
			hostConfig.PortBindings = nat.PortMap{
				"8545/tcp": []nat.PortBinding{
					{
						HostIP:   "127.0.0.1",
						HostPort: "8546",
					},
				},
			}
		},
		WaitingFor: wait.ForLog("Listening on"),
	}
	anvilC, err := testcontainers.GenericContainer(ctx, testcontainers.GenericContainerRequest{
		ContainerRequest: req,
		Started:          true,
	})

	assert.Nil(t, err, "Could not start anvil container")
	return anvilC
}

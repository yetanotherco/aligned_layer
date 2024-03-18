package task_generator

import (
	"context"
	"math/rand"
	"os"
	"time"

	"github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/Layr-Labs/eigensdk-go/signer"
	"github.com/yetanotherco/aligned_layer/aggregator/types"
	"github.com/yetanotherco/aligned_layer/common"
	"github.com/yetanotherco/aligned_layer/core/chainio"
	"github.com/yetanotherco/aligned_layer/core/config"
)

type TaskGenerator struct {
	logger    logging.Logger
	avsWriter chainio.AvsWriterer
}

func NewTaskGenerator(c *config.Config) (*TaskGenerator, error) {
	chainId, err := c.EthHttpClient.ChainID(context.Background())
	if err != nil {
		c.Logger.Error("Cannot get chainId", "err", err)
		return nil, err
	}

	privateKeySigner, err := signer.NewPrivateKeySigner(c.EcdsaPrivateKey, chainId)
	if err != nil {
		c.Logger.Error("Cannot create signer", "err", err)
		return nil, err
	}
	c.Signer = privateKeySigner

	avsWriter, err := chainio.NewAvsWriterFromConfig(c)
	if err != nil {
		c.Logger.Errorf("Cannot create AVS writer", "err", err)
		return nil, err
	}

	return &TaskGenerator{
		logger:    c.Logger,
		avsWriter: avsWriter,
	}, nil
}

func (tg *TaskGenerator) Start(ctx context.Context) error {
	tg.logger.Infof("Starting task generator.")

	ticker := time.NewTicker(10 * time.Second)
	tg.logger.Infof("Task generator set to send new task every 10 seconds...")
	defer ticker.Stop()

	taskNum := int64(0)
	// ticker doesn't tick immediately, so we send the first task here
	// see https://github.com/golang/go/issues/17601

	// We are randomizing bytes for bad proofs, all should fail
	r := rand.New(rand.NewSource(time.Now().UnixNano()))
	var proof []byte
	var pubInput []byte
	badProof := make([]byte, 32)
	r.Read(badProof)
	proof = badProof

	_ = tg.SendNewTask(proof, pubInput, common.LambdaworksCairo)
	taskNum++

	for {
		select {
		case <-ctx.Done():
			return nil
		case <-ticker.C:
			taskNum++

			// Randomly creates tasks to verify correct and incorrect proofs
			// These proofs can be either Cairo, Plonk, Sp1, Kimchi or a randomly generated one
			switch r.Intn(5) {
			case 0:
				proof, pubInput = generateCairoProof()
				err := tg.SendNewTask(proof, pubInput, common.LambdaworksCairo)
				if err != nil {
					continue
				}
			case 1:
				proof, pubInput = generateSp1Proof()
				err := tg.SendNewTask(proof, pubInput, common.Sp1BabyBearBlake3)
				if err != nil {
					continue
				}
			case 2:
				proof, pubInput = generatePlonkProof()
				err := tg.SendNewTask(proof, pubInput, common.GnarkPlonkBls12_381)
				if err != nil {
					continue
				}
			case 3:
				proof, pubInput = generateKimchiProof()
				err := tg.SendNewTask(proof, pubInput, common.Kimchi)
				if err != nil {
					continue
				}
			case 4:
				proof, pubInput = generateRandomProof(r)
				verifierId := r.Intn(3)
				err := tg.SendNewTask(proof, pubInput, common.VerifierId(verifierId))
				if err != nil {
					continue
				}
			}
		}

	}

}

// sendNewTask sends a new task to the task manager contract
func (tg *TaskGenerator) SendNewTask(proof []byte, pubInput []byte, verifierId common.VerifierId) error {
	_, taskIndex, err := tg.avsWriter.SendNewTaskVerifyProof(context.Background(), proof, pubInput, verifierId, types.QUORUM_THRESHOLD_NUMERATOR, types.QUORUM_NUMBERS)
	if err != nil {
		tg.logger.Error("Task generator failed to send proof", "err", err)
		return err
	}

	tg.logger.Infof("Generated new task with index %d \n", taskIndex)

	return nil
}

func generateCairoProof() ([]byte, []byte) {
	proofBytes, err := os.ReadFile("tests/testing_data/fibo_5.proof")
	if err != nil {
		panic("Could not read CAIRO proof file")
	}

	var pubInputBytes []byte

	return proofBytes, pubInputBytes
}

func generatePlonkProof() ([]byte, []byte) {
	proofBytes, err := os.ReadFile("tests/testing_data/plonk_cubic_circuit.proof")
	if err != nil {
		panic("Could not read PLONK proof file")
	}
	pubInputBytes, err := os.ReadFile("tests/testing_data/witness.pub")
	if err != nil {
		panic("Could not read PLONK public input file")
	}

	return proofBytes, pubInputBytes
}

func generateSp1Proof() ([]byte, []byte) {
	proofBytes, err := os.ReadFile("tests/testing_data/sp1_fibonacci.proof")
	if err != nil {
		panic("Could not read SP1 proof file")
	}

	var pubInputBytes []byte

	return proofBytes, pubInputBytes
}

func generateKimchiProof() ([]byte, []byte) {
	proofBytes, err := os.ReadFile("tests/testing_data/kimchi/kimchi_ec_add.proof")
	if err != nil {
		panic("Could not read Kimchi proof file")
	}

	pubInputBytes, err := os.ReadFile("tests/testing_data/kimchi/kimchi_verifier_index.bin")
	if err != nil {
		panic("Could not read Kimchi public input file")
	}

	return proofBytes, pubInputBytes
}

func generateRandomProof(r *rand.Rand) ([]byte, []byte) {
	badProof := make([]byte, 32)
	r.Read(badProof)

	var pubInputBytes []byte

	return badProof, pubInputBytes
}

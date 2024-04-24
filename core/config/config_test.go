package config_test

import (
	"errors"
	"testing"

	"github.com/yetanotherco/aligned_layer/core/tests/mocks"
)

func TestConfigValidate(t *testing.T) {
	testCases := []struct {
		name                                   string
		ecdsaPrivateKey                        string
		alignedLayerOperatorStateRetrieverAddr string
		alignedLayerServiceManagerAddr         string
		expectedError                          error
	}{
		{
			name:                                   "Missing EcdsaPrivateKey",
			ecdsaPrivateKey:                        "",
			alignedLayerOperatorStateRetrieverAddr: "0x9d4454b023096f34b160d6b654540c56a1f81688",
			alignedLayerServiceManagerAddr:         "0xc5a5c42992decbae36851359345fe25997f5c42d",
			expectedError:                          errors.New("Config: EcdsaPrivateKey is required"),
		},
		{
			name:                                   "Missing AlignedLayerOperatorStateRetrieverAddr",
			ecdsaPrivateKey:                        "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
			alignedLayerOperatorStateRetrieverAddr: "",
			alignedLayerServiceManagerAddr:         "0xc5a5c42992decbae36851359345fe25997f5c42d",
			expectedError:                          errors.New("Config: AlignedLayerOperatorStateRetrieverAddr is required"),
		},
		{
			name:                                   "Missing AlignedLayerServiceManagerAddr",
			ecdsaPrivateKey:                        "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
			alignedLayerOperatorStateRetrieverAddr: "0x9d4454b023096f34b160d6b654540c56a1f81688",
			alignedLayerServiceManagerAddr:         "",
			expectedError:                          errors.New("Config: AlignedLayerServiceManagerAddr is required"),
		},
		{
			name:                                   "All Fields Present",
			ecdsaPrivateKey:                        "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
			alignedLayerOperatorStateRetrieverAddr: "0x9d4454b023096f34b160d6b654540c56a1f81688",
			alignedLayerServiceManagerAddr:         "0xc5a5c42992decbae36851359345fe25997f5c42d",
			expectedError:                          nil,
		},
	}

	for _, tc := range testCases {
		t.Run(tc.name, func(t *testing.T) {
			mockConfig := mocks.NewMockConfig(tc.ecdsaPrivateKey, tc.alignedLayerOperatorStateRetrieverAddr, tc.alignedLayerServiceManagerAddr)
			err := mockConfig.Validate()
			if err == nil && tc.expectedError != nil {
				t.Errorf("Expected error %v, but got nil", tc.expectedError)
			} else if err != nil && tc.expectedError == nil {
				t.Errorf("Expected no error, but got %v", err)
			} else if err != nil && tc.expectedError != nil && err.Error() != tc.expectedError.Error() {
				t.Errorf("Expected error %v, but got %v", tc.expectedError, err)
			}
		})
	}
}

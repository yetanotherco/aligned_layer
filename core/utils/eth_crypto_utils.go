package utils

import (
	"github.com/ethereum/go-ethereum/accounts/abi"
	servicemanager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
	"golang.org/x/crypto/sha3"
)

func TaskResponseDigest(h *servicemanager.AlignedLayerServiceManagerTaskResponse) ([32]byte, error) {
	encodeTaskResponseByte, err := AbiEncodeTaskResponse(*h)
	if err != nil {
		return [32]byte{}, err
	}

	var taskResponseDigest [32]byte
	hasher := sha3.NewLegacyKeccak256()
	hasher.Write(encodeTaskResponseByte)
	copy(taskResponseDigest[:], hasher.Sum(nil)[:32])

	return taskResponseDigest, nil
}

func AbiEncodeTaskResponse(taskResponse servicemanager.AlignedLayerServiceManagerTaskResponse) ([]byte, error) {
	// The order here has to match the field ordering of servicemanager.AlignedLayerServiceManagerTaskResponse

	/* TODO: Solve this in a more generic way so it's less prone for errors. Name and types can be obtained with reflection
	for i := 0; i < reflectedType.NumField(); i++ {
		name := reflectedType.Field(i).Name
		thisType := reflectedType.Field(i).Type
	}
	*/

	/*
		This matches:

		struct TaskResponse {
			uint64 taskIndex;
			bool proofIsCorrect;
		}
	*/
	taskResponseType, err := abi.NewType("tuple", "", []abi.ArgumentMarshaling{
		{
			Name: "taskIndex",
			Type: "uint64",
		},
		{
			Name: "proofIsCorrect",
			Type: "bool",
		},
	})
	if err != nil {
		return nil, err
	}
	arguments := abi.Arguments{
		{
			Type: taskResponseType,
		},
	}

	bytes, err := arguments.Pack(taskResponse)
	if err != nil {
		return nil, err
	}

	return bytes, nil
}

# Multiple OOB in the verify function of the Operator

**Author(s):** Nabih Benazzouz [@Fuzzinglabs](https://github.com/FuzzingLabs/)

**Date:** 09/08/2024

## **Executive Summary**

During an audit and fuzzing of several operator functions, we identified multiple out-of-bounds (OOB) vulnerabilities within the `verify` function. This function manipulates arrays without performing adequate size checks. For example, the code snippet `copy(csLenBuffer, paramsBytes[:4])` is used without verifying the size of the `paramsBytes` array, leading to potential OOB errors.

## Vulnerability Details

- **Severity:** Critical

- **Affected Component:** operator/pkg

- **Permalink:** [GitHub Source](https://github.com/yetanotherco/aligned_layer/blob/81e1ae4ff6cdddca28808ac071d4b1ae72793346/operator/pkg/operator.go#L251)

## Environment

- **Distro Version:** 6.9.9-1-MANJARO
- **Additional Environment Details:** rustc 1.79.0-nightly (1cec373f6 2024-04-16)

## Steps to Reproduce

You can trigger the OOB issues in this function using the following fuzzing harness. Once one OOB is fixed, others may be revealed.

1. Start the batcher:

```go
// operator/pkg/operatorfuzz_test.go
package operator

import (
	"encoding/binary"
	"errors"
	"fmt"
	"testing"

	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"
	"github.com/google/gofuzz"
	"github.com/yetanotherco/aligned_layer/common"
)

func NewLogger(loggingLevel sdklogging.LogLevel) (sdklogging.Logger, error) {
	logger, err := sdklogging.NewZapLogger(loggingLevel)
	if err != nil {
		fmt.Println("Could not initialize logger")
		return nil, err
	}
	return logger, nil
}

func bytesToInt64(b []byte) (int64, error) {
	if len(b) < 8 {
		return 0, errors.New("byte slice too short to convert to int64")
	}
	return int64(binary.BigEndian.Uint64(b[:8])), nil
}

func FuzzVerifyOperator(f *testing.F) {

	f.Fuzz(func(t *testing.T, data []byte) {
		// Verify operator
		seed, err := bytesToInt64(data)
		if err != nil {
			return
		}
		f := fuzz.NewWithSeed(seed)
		logger, _ := NewLogger(sdklogging.LogLevel("development"))
		var op Operator = Operator{Logger: logger}
		var verifData VerificationData
		var verifBool chan bool = make(chan bool)
		f.Fuzz(&verifData)
		verifData.ProvingSystemId = common.Halo2IPA
		op.verify(verifData, verifBool)
	})
}
```
```sh
go test -fuzz=FuzzVerifyOperator
```


## Root Cause Analysis

The vulnerability in the `verify` function arises due to the lack of length checks for various arrays or slices before they are manipulated. This oversight can lead to out-of-bounds errors.

Example:

```go
...
		csLenBuffer := make([]byte, 4)
		copy(csLenBuffer, paramsBytes[:4])
		csLen := (uint32)(binary.LittleEndian.Uint32(csLenBuffer))

		// Deserialize vkLen
		vkLenBuffer := make([]byte, 4)
		copy(vkLenBuffer, paramsBytes[4:8])
		vkLen := (uint32)(binary.LittleEndian.Uint32(vkLenBuffer))

		// Deserialize ipaParamLen
		IpaParamsLenBuffer := make([]byte, 4)
		copy(IpaParamsLenBuffer, paramsBytes[8:12])
		IpaParamsLen := (uint32)(binary.LittleEndian.Uint32(IpaParamsLenBuffer))
...
and more

```

Since the `verificationData` utilized in this function is retrieved from an S3 bucket, there is a possibility that a user could send malformed data to the S3, triggering these vulnerabilities.

```go
func (o *Operator) ProcessNewBatchLog(newBatchLog *servicemanager.ContractAlignedLayerServiceManagerNewBatch) error {
    ...
	verificationDataBatch, err := o.getBatchFromS3(newBatchLog.BatchDataPointer, newBatchLog.BatchMerkleRoot)
    ...
	for _, verificationData := range verificationDataBatch {
		go func(data VerificationData) {
			defer wg.Done()
			o.verify(data, results)

```
Additionally, a user could exploit this vulnerability by providing their own URL in the smart contract via `AlignedServiceManager.createNewTask`.

```solidity
...
contract AlignedLayerServiceManager is
    ...
{
    ...
    // ! Everyone can call this function
    function createNewTask(
        bytes32 batchMerkleRoot,
        string calldata batchDataPointer
    ) external payable {
        ...
    }
}
```


## Detailed Behavior

```sh
--- FAIL: FuzzVerifyOperator (0.04s)
    --- FAIL: FuzzVerifyOperator (0.00s)
        testing.go:1590: panic: runtime error: slice bounds out of range [:4] with capacity 2
            goroutine 32 [running]:
            runtime/debug.Stack()
                /usr/lib/go/src/runtime/debug/stack.go:24 +0x9b
            testing.tRunner.func1()
                /usr/lib/go/src/testing/testing.go:1590 +0x1c8
            panic({0x190ac40?, 0xc0001425e8?})
                /usr/lib/go/src/runtime/panic.go:770 +0x132
            github.com/yetanotherco/aligned_layer/operator/pkg.(*Operator).verify(0xc0005195e0, {0x5, {0x0, 0x0, 0x0}, {0xc00027fef0, 0x3, 0x3}, {0xc00027ff00, 0x2, ...}, ...}, ...)
                /home/raefko/AlignedLayer_audit/operator/pkg/operator.go:251 +0x14e9
            github.com/yetanotherco/aligned_layer/operator/pkg.FuzzVerifyOperator.func1(0x0?, {0xc00027fe80, 0x8, 0x4a9613?})
                /home/raefko/AlignedLayer_audit/operator/pkg/operatorfuzz_test.go:45 +0x1e8
            reflect.Value.call({0x18463a0?, 0x1a382e0?, 0x13?}, {0x195a57a, 0x4}, {0xc000299860, 0x2, 0x2?})
                /usr/lib/go/src/reflect/value.go:596 +0xca6
            reflect.Value.Call({0x18463a0?, 0x1a382e0?, 0x5e16ed?}, {0xc000299860?, 0x1958fa0?, 0xf?})
                /usr/lib/go/src/reflect/value.go:380 +0xb9
            testing.(*F).Fuzz.func1.1(0xc00015fa00?)
                /usr/lib/go/src/testing/fuzz.go:335 +0x325
            testing.tRunner(0xc00015fa00, 0xc000166900)
                /usr/lib/go/src/testing/testing.go:1689 +0xfb
            created by testing.(*F).Fuzz.func1 in goroutine 34
                /usr/lib/go/src/testing/fuzz.go:322 +0x574
            
    
FAIL
exit status 1
FAIL    github.com/yetanotherco/aligned_layer/operator/pkg      0.056s

```

## Recommendations

1.	Implement strict input validation to ensure that all input fields are of the expected size before processing.
2.	Add a modifier to the `createNewTask` function to restrict access to only authorized users.
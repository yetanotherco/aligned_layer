# A dangerous ffi pattern that lead to panic in `operator`

**Author(s):** Mohammed Benhelli [@Fuzzinglabs](https://github.com/FuzzingLabs/)

**Date:** 01/08/2024

### **Executive summary**

During the process of auditing the code and developing fuzzing harnesses, we found a pattern that can lead to a panic in
the `getBatchFromS3` method of `Operator` for example.

### Vulnerability Details

- **Severity:** Medium
- **Affected Component:**
    - `operator/sp1/sp1.go`
    - `operator/risc_zero/risc_zero.go`
    - `operator/merkle_tree/merkle_tree.go`

## Environment

- **Distro Version:** Ubuntu 22.04.4 LTS
- **Additional Environment Details:** go version go1.22.5 linux/amd64

## Semgrep rule
To track this pattern, we can use the following [semgrep](https://semgrep.dev/) rule:
```yaml
rules:
  - id: unsafe-pointer-slice-access
    patterns:
      - pattern: |
          func $FUNC(..., $PARAM []$TYPE, ...) $RET {
            ...
            $VAR := (*C.uchar)(unsafe.Pointer(&$PARAM[0]))
            ...
          }
    message: |
      Hardcoded access to the first element of a slice detected.
      - For unsafe.Pointer(&$PARAM[0]): Consider checking slice length before access.
    languages: [go]
    severity: WARNING
    metadata:
      category: safety
      technology:
        - go
      references:
        - https://golang.org/pkg/unsafe/
        - https://blog.golang.org/slices
    captures:
      - name: SLICE
        type: slice
```

And run it like this in the root of the project:
```sh
semgrep --config unsafe-pointer-slice-access.yaml .
```

## Semgrep matches
```txt
┌─────────────────┐
│ 6 Code Findings │
└─────────────────┘

    operator/merkle_tree/merkle_tree.go
    ❯❱ custom-rules.unsafe-pointer-slice-access
          Hardcoded access to the first element of a slice detected. - For unsafe.Pointer(&batchBuffer[0]):
          Consider checking slice length before access.

           13┆ func VerifyMerkleTreeBatch(batchBuffer []byte, batchLen uint, merkleRootBuffer [32]byte)
               bool {
           14┆   batchPtr := (*C.uchar)(unsafe.Pointer(&batchBuffer[0]))
           15┆   merkleRootPtr := (*C.uchar)(unsafe.Pointer(&merkleRootBuffer[0]))
           16┆   return (bool)(C.verify_merkle_tree_batch_ffi(batchPtr, (C.uint)(batchLen),
               merkleRootPtr))
           17┆ }

    operator/risc_zero/risc_zero.go
    ❯❱ custom-rules.unsafe-pointer-slice-access
          Hardcoded access to the first element of a slice detected. - For unsafe.Pointer(&imageIdBuffer[0]):
          Consider checking slice length before access.

           14┆ func VerifyRiscZeroReceipt(receiptBuffer []byte, receiptLen uint32, imageIdBuffer []byte,
               imageIdLen uint32, publicInput []byte, publicInputLen uint32) bool {
           15┆   receiptPtr := (*C.uchar)(unsafe.Pointer(&receiptBuffer[0]))
           16┆   imageIdPtr := (*C.uchar)(unsafe.Pointer(&imageIdBuffer[0]))
           17┆   publicInputPtr := (*C.uchar)(unsafe.Pointer(&publicInput[0]))
           18┆   return (bool)(C.verify_risc_zero_receipt_ffi(receiptPtr, (C.uint32_t)(receiptLen),
               imageIdPtr, (C.uint32_t)(imageIdLen), publicInputPtr, (C.uint32_t)(publicInputLen)))
           19┆ }
            ⋮┆----------------------------------------
    ❯❱ custom-rules.unsafe-pointer-slice-access
          Hardcoded access to the first element of a slice detected. - For unsafe.Pointer(&publicInput[0]):
          Consider checking slice length before access.

           14┆ func VerifyRiscZeroReceipt(receiptBuffer []byte, receiptLen uint32, imageIdBuffer []byte,
               imageIdLen uint32, publicInput []byte, publicInputLen uint32) bool {
           15┆   receiptPtr := (*C.uchar)(unsafe.Pointer(&receiptBuffer[0]))
           16┆   imageIdPtr := (*C.uchar)(unsafe.Pointer(&imageIdBuffer[0]))
           17┆   publicInputPtr := (*C.uchar)(unsafe.Pointer(&publicInput[0]))
           18┆   return (bool)(C.verify_risc_zero_receipt_ffi(receiptPtr, (C.uint32_t)(receiptLen),
               imageIdPtr, (C.uint32_t)(imageIdLen), publicInputPtr, (C.uint32_t)(publicInputLen)))
           19┆ }
            ⋮┆----------------------------------------
    ❯❱ custom-rules.unsafe-pointer-slice-access
          Hardcoded access to the first element of a slice detected. - For unsafe.Pointer(&receiptBuffer[0]):
          Consider checking slice length before access.

           14┆ func VerifyRiscZeroReceipt(receiptBuffer []byte, receiptLen uint32, imageIdBuffer []byte,
               imageIdLen uint32, publicInput []byte, publicInputLen uint32) bool {
           15┆   receiptPtr := (*C.uchar)(unsafe.Pointer(&receiptBuffer[0]))
           16┆   imageIdPtr := (*C.uchar)(unsafe.Pointer(&imageIdBuffer[0]))
           17┆   publicInputPtr := (*C.uchar)(unsafe.Pointer(&publicInput[0]))
           18┆   return (bool)(C.verify_risc_zero_receipt_ffi(receiptPtr, (C.uint32_t)(receiptLen),
               imageIdPtr, (C.uint32_t)(imageIdLen), publicInputPtr, (C.uint32_t)(publicInputLen)))
           19┆ }

    operator/sp1/sp1.go
    ❯❱ custom-rules.unsafe-pointer-slice-access
          Hardcoded access to the first element of a slice detected. - For unsafe.Pointer(&elfBuffer[0]):
          Consider checking slice length before access.

           12┆ func VerifySp1Proof(proofBuffer []byte, proofLen uint32, elfBuffer []byte, elfLen uint32)
               bool {
           13┆   proofPtr := (*C.uchar)(unsafe.Pointer(&proofBuffer[0]))
           14┆   elfPtr := (*C.uchar)(unsafe.Pointer(&elfBuffer[0]))
           15┆
           16┆   return (bool)(C.verify_sp1_proof_ffi(proofPtr, (C.uint32_t)(proofLen), elfPtr,
               (C.uint32_t)(elfLen)))
           17┆ }
            ⋮┆----------------------------------------
    ❯❱ custom-rules.unsafe-pointer-slice-access
          Hardcoded access to the first element of a slice detected. - For unsafe.Pointer(&proofBuffer[0]):
          Consider checking slice length before access.

           12┆ func VerifySp1Proof(proofBuffer []byte, proofLen uint32, elfBuffer []byte, elfLen uint32)
               bool {
           13┆   proofPtr := (*C.uchar)(unsafe.Pointer(&proofBuffer[0]))
           14┆   elfPtr := (*C.uchar)(unsafe.Pointer(&elfBuffer[0]))
           15┆
           16┆   return (bool)(C.verify_sp1_proof_ffi(proofPtr, (C.uint32_t)(proofLen), elfPtr,
               (C.uint32_t)(elfLen)))
           17┆ }

```

## Steps to Reproduce

To demonstrate the issue, we provide a Proof of Concept (POC) to show that the bug can be triggered in the `operator` package.

1. Create a file with the following content:
    ```go
    package operator_test
    
    import (
        "github.com/Layr-Labs/eigensdk-go/logging"
        servicemanager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
        "github.com/yetanotherco/aligned_layer/core/config"
        operator "github.com/yetanotherco/aligned_layer/operator/pkg"
        "testing"
    )
    
    func TestGetBatchS3Panic(t *testing.T) {
        logger, err := config.NewLogger(logging.Production)
        if err != nil {
            t.Fatalf("Error creating logger: %v", err)
        }
        op := operator.Operator{
            Logger: logger,
        }
        newBatchLog := &servicemanager.ContractAlignedLayerServiceManagerNewBatch{
            BatchMerkleRoot:  [32]byte([]byte("[\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t]")),
            BatchDataPointer: "https://proof-of-null.s3.eu-west-3.amazonaws.com/rieng",
        }
        if err := op.ProcessNewBatchLog(newBatchLog); err != nil {
            t.Fatalf("Error processing new batch log: %v", err)
        }
    }
    ```
2. Run the test:
   ```sh
   go test github.com/yetanotherco/aligned_layer/fuzzinglabs/operator
   ```

## Root Cause Analysis

The root cause is located in `operator/merkle_tree/merkle_tree.go`.

```go
package merkle_tree
...


func VerifyMerkleTreeBatch(batchBuffer []byte, batchLen uint, merkleRootBuffer [32]byte) bool {
	batchPtr := (*C.uchar)(unsafe.Pointer(&batchBuffer[0]))
	...
}
```

`batchBuffer` can be empty, which will cause the panic accessing the first element of the slice.

## Detailed Behavior

```sh
{"level":"info","ts":1722517205.441977,"caller":"pkg/operator.go:177","msg":"Received new batch with proofs to verify","batch merkle root":[91,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9]}
{"level":"info","ts":1722517205.4420319,"caller":"pkg/s3.go:13","msg":"Getting batch from S3..., batchURL: https://proof-of-null.s3.eu-west-3.amazonaws.com/rieng"}
{"level":"info","ts":1722517205.5218606,"caller":"pkg/s3.go:46","msg":"Verifying batch merkle tree..."}
--- FAIL: TestGetBatchS3Panic (0.08s)
panic: runtime error: index out of range [0] with length 0 [recovered]
        panic: runtime error: index out of range [0] with length 0

goroutine 6 [running]:
...
github.com/yetanotherco/aligned_layer/operator/merkle_tree.VerifyMerkleTreeBatch(...)
        /aligned_layer/operator/merkle_tree/merkle_tree.go:14
github.com/yetanotherco/aligned_layer/operator/pkg.(*Operator).getBatchFromS3(0xc000002180, {0x1552b63, 0x36}, {0x5b, 0x9, 0x9, 0x9, 0x9, 0x9, 0x9, ...})
        /aligned_layer/operator/pkg/s3.go:47 +0x630
github.com/yetanotherco/aligned_layer/operator/pkg.(*Operator).ProcessNewBatchLog(0xc000002180, 0xc000159e70)
        /aligned_layer/operator/pkg/operator.go:181 +0x10e
...
```

## Recommendations

- Input Validation: Ensure that the len of `batchBuffer` is greater than 0 before processing it.
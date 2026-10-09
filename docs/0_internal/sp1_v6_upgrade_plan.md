# Plan: Move Hoodi to Succinct's SP1 gateway and upgrade Aggregation Mode to SP1 v6 (Hoodi first, then mainnet)

## Context
On 2026-10-09 at 06:13 UTC, Succinct froze the SP1 **v5.0.0** Groth16 route (`0xa4594c59`) and the v6.0.0 route (`0x0e78f4db`) on its gateways. Our aggregator uses `sp1-sdk 5.2.3` (circuit v5.0.0), so every `verifyAggregationSP1` on mainnet now reverts with `RouteIsFrozen`. On Hoodi we use our **own** gateway (`0xe1D19B93…e9ef`), which still routes v5. That's worse: it keeps accepting a version Succinct froze. Succinct now runs a Hoodi gateway, so we should use theirs.

Facts verified on-chain or upstream:
| | Hoodi | Mainnet |
|---|---|---|
| `AlignedProofAggregationService` (UUPS) | `0x6B34AAaE780A5EAB4c91AB8F54f2a421E9c2FB59` | `0xD0696d3eEebffcAB2D1b358805efAA005A9A8BC0` |
| Owner | EOA `0x8F2485805f174048855878593D323Bcfcf813149` | Safe `0xfC135A861efF65b7644Fb03d9f851251546C0835` (**5 of 8**) |
| `sp1VerifierAddress` today | our gateway `0xe1D19B93…e9ef` | Succinct `0x397A5f7f3dBd538f23DE225B51f532c34448dA9B` ✅ |
| Succinct Groth16 gateway | `0x7DA83eC4af493081500Ecd36d1a72c23F8fc2abd` | `0x397A5f7f…dA9B` |
| v6.1.0 route `0x4388a21c` → `0xb69f2584…f4e2` | active | active |
| Allowlisted SP1 chunk VK (SP1_ID=1) | `0x00d6e32a…77af` | `0x00d6e32a…77af` |

- Target is **sp1-sdk / sp1-zkvm / sp1-build 6.8.1**, the latest release. Its `SP1_CIRCUIT_VERSION` is **v6.1.0**, so its proofs route to the active `0x4388a21c`. Never pin 6.0.x: GHSA-63x8-x938-vx33 applies, and that route is frozen.
- **No contract upgrade is needed.** `ISP1Verifier` and the gateway interfaces are byte-identical between v5 and v6.1.x. Mainnet only needs the VK allowlist changed. Hoodi needs that plus `setSP1VerifierAddress`.
- **The upgrade breaks users.** A v6 guest cannot verify v5 proofs, so users must re-prove with sp1-sdk ≥ 6.1.0. v5 tasks still queued in the DB can never be aggregated.
- Decisions so far:
  - Scope is Aggregation Mode only. The deprecated Verification Layer stays on v5.
  - Queued v5 tasks are marked with a new `failed` status.
  - Mainnet runs as-is until the cutover (SP1 runs keep failing). Each failure happens at the gas-estimation step in `provider.fill()` at `proof_aggregator/src/backend/mod.rs:362`, so no gas is spent.

---

## Part A: Code changes (one PR, deployed by the normal release flow; new minor version, since it breaks the SDK)

### A1. Dependencies (SP1 6.8.1)
- `aggregation_mode/Cargo.toml:16`: `sp1-sdk = { version = "6.8.1" }` (default features are now `[]`).
- `aggregation_mode/cli/Cargo.toml:14`: switch to `sp1-sdk = { workspace = true }`. It is currently pinned on its own.
- `aggregation_mode/proof_aggregator/Cargo.toml`:
  - Set `sp1-build = "6.8.1"` (line 34).
  - Change the `gpu` feature (lines 42-44) to `["risc0-zkvm/cuda", "sp1-sdk/cuda"]`. Without that, `SP1_PROVER=cuda` panics in v6.
- `aggregation_mode/proof_aggregator/aggregation_programs/sp1/Cargo.toml`:
  - `sp1-zkvm = { version = "6.8.1", features = ["verify"] }`.
  - Replace the direct sha2/sha3 git deps with the v6 patches, applied through `[patch.crates-io]` in `aggregation_programs/Cargo.toml`:
    - `sha2`: tag `patch-sha2-0.10.8-sp1-6.0.0`
    - `sha3`: tag `patch-sha3-0.10.8-sp1-6.0.0`
    - The old tags are riscv32-gated. They would quietly fall back to software hashing, which costs far more cycles.
- Update locks with targeted `cargo update -p …`. Then confirm the **RISC0 image IDs in `programs_ids.json` are unchanged**: workspace B (`aggregation_programs/Cargo.lock`) is shared with the risc0 guests.
- Delete the stale tracked lockfiles `aggregation_mode/proof_aggregator/Cargo.lock` and `aggregation_programs/sp1/Cargo.lock`. Both crates are workspace members, so these files are unused.
- `examples/l2` uses `agg_mode_sdk` by path, so bump it in lockstep:
  - sp1-sdk, sp1-build and sp1-zkvm 6.8.1, plus the sha3 patch tag
  - its `Cargo.lock`
  - its program IDs (`zkvm_programs/bin/write_program_ids.rs`)

### A2. Code: SDK v6 API, which is async by default
- `aggregation_mode/proof_aggregator/src/aggregators/sp1_aggregator.rs`:
  - `ProverClient::from_env().await` and `builder().cpu().build().await`.
  - `setup(elf).await?`, then take the vk with `pk.verifying_key().clone()` (lines 19-26, 117, 210-214).
  - `prove(&pk, stdin)` takes stdin by value and is `.await`ed directly (`.compressed()` at 119-123, `.groth16()` at 180-185).
  - `verify(&proof, &vk, None)` (lines 41-59).
  - Keep `stdin.write_proof(*compressed, vk.vk.clone())`.
  - Mock mode: `.mock()` now returns `MockProver`, and `.deferred_proof_verification(false)` still exists.
  - ELFs become `Elf::Static(include_bytes!(…))`.
- `aggregation_mode/gateway/src/verifiers.rs`: replace the `LazyLock` CPU prover with the verify-only `ProverClient::builder().light()` client (async). Keep accepting **`SP1Proof::Compressed` only**. v5 bincode will fail to deserialize at `gateway/src/http.rs:283-291`: return a clear 400, e.g. "SP1 v5 proofs are no longer supported; re-prove with sp1-sdk >= 6.1.0".
- `aggregation_mode/sdk` and `cli`: should only need recompiling. Update the doc snippets.
- `aggregation_mode/proof_aggregator/build.rs:30-42`: pin `BuildArgs { tag: "v6.8.1".into(), … }` so the VK hashes don't depend on whichever sp1-build version happens to be installed.

### A3. Regenerate program IDs and VKs
- Run `make proof_aggregator_write_program_ids` (Makefile:297) → `proof_aggregator/scripts/build_programs.sh`. This makes two passes:
  1. It writes the new user-proofs VK words into `chunk_aggregator_main.rs:9-11`.
  2. It writes the final `programs_ids.json`.
- The script uses BSD `sed -i ''`, so either run it on macOS or make it portable.
- Record the new **`sp1_chunk_aggregator_vk_hash`** (call it `NEW_VK`). This is the value that goes on-chain.
- Replace `0x00d6e32a…77af` with `NEW_VK` in:
  - `contracts/script/deploy/config/{hoodi,mainnet,devnet,devnet.mock}/proof-aggregator-service.*.config.json`
  - `config-files/config-proof-aggregator*.yaml`, including the stale `00ba19ee…` copies
  - `infra/aggregation_mode/config-proof-aggregator-sp1.template.yaml:15`

### A4. Contracts and devnet
- Bump the `contracts/lib/sp1-contracts` submodule from `512b5e0` (v5.0.0) to tag **v6.1.1** (`d3629729`).
- `contracts/script/deploy/SP1VerifierGroth16Deployer.s.sol`: import `v6.1.0/SP1VerifierGroth16.sol` and use key `V6_1_0_SP1_VERIFIER_GROTH16`. Only the anvil devnet uses this.
- Regenerate the devnet state with the v6.1.0 verifier and `NEW_VK`:
  - `contracts/deployments/31337.json`
  - `contracts/scripts/anvil/state/{sp1,alignedlayer}-deployed-anvil-state.json`
  - `network_params.yaml` genesis allocations
- Hoodi config: in `contracts/script/deploy/config/hoodi/proof-aggregator-service.hoodi.config.json`, set `sp1VerifierAddress` to `0x7DA83eC4af493081500Ecd36d1a72c23F8fc2abd`. Keep `contracts/deployments/560048.json` as the record of the deprecated gateway.
- **New owner-ops tooling.** There is none today; the only reference is a README paragraph. Add `contracts/scripts/proof_aggregator_owner_call.sh`, modeled on `contracts/scripts/upgrade_proof_aggregator.sh:57-69`:
  - It builds calldata with `cast calldata`.
  - If `MULTISIG=false`, it sends with `cast send --private-key`. Otherwise it prints the target, the calldata and the selector for the Safe.
  - Makefile targets:
    - `proof_aggregator_set_sp1_verifier NETWORK= ADDRESS=`
    - `proof_aggregator_allow_program NETWORK= COMMITMENT= PROVING_SYSTEM=`
    - `proof_aggregator_disallow_program NETWORK= COMMITMENT= PROVING_SYSTEM=`
  - The proxy address is read with `jq '.addresses.alignedProofAggregationService' $PROOF_AGGREGATOR_OUTPUT_PATH`.
- Fix `contracts/scripts/.env.mainnet`: set `MULTISIG=true`. It is `false` today even though the owner is a Safe.

### A5. DB: terminal `failed` status
- Add `aggregation_mode/db/migrations/003_task_status_failed.sql` containing `ALTER TYPE task_status ADD VALUE IF NOT EXISTS 'failed';`.
- Add `Failed` to `TaskStatus` in `aggregation_mode/db/src/types.rs:12-16`. sqlx decoding fails on unknown enum values, and the gateway status endpoint serializes this enum (`gateway/src/types.rs:59,67`).
- Add the one-off ops script `aggregation_mode/db/scripts/mark_sp1_v5_tasks_failed.sql`. It runs in a transaction and takes a `:cutover_ts` parameter:
  ```sql
  UPDATE tasks SET status = 'failed', status_updated_at = now()
  WHERE proving_system_id = 1 AND status IN ('pending','processing') AND inserted_at < :cutover_ts;
  ```
  The fetcher only selects `pending` and stale `processing` rows (`proof_aggregator/src/backend/db.rs:76-93`), so `failed` rows are never picked up again.

### A6. Fixtures, task sender and docs
- Bump `scripts/test_files/sp1/fibonacci_proof_generator/{script,program}` to v6.8.1 and set `SP1_VERSION="6_1_0"` in `script/src/main.rs:8`.
  - Generate `sp1_fibonacci_6_1_0.{proof,pub,vk,elf}` and `_vk.bin` with `make generate_sp1_fibonacci_proof`.
  - **Keep the `*_5_0_0.*` files**: the Verification Layer still uses them.
- Point these at the 6_1_0 fixture:
  - `Makefile:291-292,334-335`
  - `scripts/.agg_mode.task_sender.env:2-3`
  - `infra/aggregation_mode/ansible/playbooks/ini/config-{hoodi,mainnet}.ini:105-106`
- CI (`.github/workflows/build-and-test-rust.yml:33-37`):
  - Pin `sp1up -v 6.8.1`. MSRV is 1.88, which matches the current pin.
  - Confirm no CI job builds a v5 *guest* with the local toolchain. Aggregation guests build in Docker, and the Verification Layer crates only verify.
- Infra: in `infra/aggregation_mode/aggregation_mode.sh:91-94`, pin `sp1up -v 6.8.1`. Moongate is gone in v6. The GPU server binary `sp1-gpu-server v6.8.1` is downloaded into `~/.sp1/bin`, and Groth16 wrapping still uses the `sp1-gnark:v6.1.0` Docker image.
- Docs:
  - `docs/2_proof_aggregation_layer/architecture/4_supported_verifiers.md:5`: SP1 v6.1.x, compressed only.
  - `guides/4_sdk.md:15,31-45`, `guides/2_generating_proofs.md:21-48` (async v6 API) and `guides/1_quickstart.md:57-58,84-85` (fixture paths).
  - Fix the wrong `program_ids.json` path in `proof_aggregator/README.md:34,69-84`.
  - Add an internal runbook `docs/0_internal/8_b_1_propose_allow_verifying_program_multisig.md`, modeled on `7_b_1_propose_setting_aggregator_address_multisig.md`.

---

## Part B: Hoodi rollout (owner is an EOA, so direct `cast send`)
1. Merge the PR and deploy to Hoodi with the usual flow (`git_branch=staging` in `config-hoodi.ini`).
2. Run the DB migration: `make postgres_migrations ENV=hoodi`.
3. Redeploy the gateway with the v6 build (ansible `gateway.yaml`), then the task sender with the v6 fixture (`task_sender.yaml`). Note `CUTOVER_TS` = when the gateway deploy finished.
4. Mark v5 tasks failed: run `mark_sp1_v5_tasks_failed.sql` with `CUTOVER_TS`.
5. **On-chain** (`NETWORK=hoodi`, owner key):
   - `make proof_aggregator_set_sp1_verifier ADDRESS=0x7DA83eC4af493081500Ecd36d1a72c23F8fc2abd` (selector `0x9eade0f1`)
   - `make proof_aggregator_allow_program COMMITMENT=$NEW_VK PROVING_SYSTEM=1` (`0xd5d8d0a8`)
   - `make proof_aggregator_disallow_program COMMITMENT=0x00d6e32a34f68ea643362b96615591c94ee0bf99ee871740ab2337966a4f77af PROVING_SYSTEM=1` (`0x55a9a7bf`)
   - Check with `cast call`: `sp1VerifierAddress()` should return `0x7DA8…`, `allowedVerifiersProvingSystem(NEW_VK)` should return `1`, and the old VK should return `0`.
6. Optional hygiene: freeze v5 on our own gateway (owner `0xF138…E927`) with `SP1VerifierGroth16Deployer.s.sol::freeze()`.
7. Update the Hoodi aggregator host:
   - Run `sp1up -v 6.8.1`.
   - Reinstall with `cargo install --path aggregation_mode/proof_aggregator --features prove,gpu --bin proof_aggregator_gpu --locked` from the release commit.
   - Set `sp1_chunk_aggregator_vk_hash: NEW_VK` in its SP1 config.
   - Confirm the GPU meets the v6 requirements: compute capability ≥ 8.0, ≥ 24 GB VRAM, CUDA 12.
8. Run an aggregation with a v6 proof from the task sender or `make agg_mode_gateway_send_sp1_proof`.
   - Expect an `AggregatedProofVerified` event.
   - Expect `make verify_aggregated_proof_sp1` to return Verified.
   - **Save the tx input** (`sp1PublicValues`, `sp1ProofBytes`) for the mainnet fork rehearsal.
9. Let it run for a few daily cycles and watch for gateway 4xx from old-SDK users.

## Part C: Mainnet rollout (Safe 5/8)

**Prepare the Safe transaction (can overlap with the Hoodi soak):**
1. **Independent VK check:** a second engineer checks out the release tag, runs `make proof_aggregator_write_program_ids`, and confirms `programs_ids.json` gives the same `NEW_VK`. The build is reproducible because the Docker tag is pinned.
2. **Fork rehearsal**, to prove the mainnet gateway accepts our v6 proof with `NEW_VK` before anyone signs:
   ```
   anvil --fork-url $MAINNET_RPC
   cast rpc anvil_impersonateAccount 0xfC135A861efF65b7644Fb03d9f851251546C0835 ; fund it with anvil_setBalance
   cast send --unlocked --from <Safe> 0xD0696d3e… "allowVerifyingProgram(bytes32,uint8)" $NEW_VK 1
   cast call --from 0x57628fe929016C30463473349DcDd592ea27C8b3 0xD0696d3e… \
     "verifyAggregationSP1(bytes32,bytes,bytes,bytes32)" 0x00…00 $PUB $PROOF $NEW_VK   # must not revert
   ```
   The contract does not check `blobhash`, so the Hoodi proof can be reused as-is.
3. **Generate calldata** with `make proof_aggregator_allow_program NETWORK=mainnet COMMITMENT=$NEW_VK PROVING_SYSTEM=1` and the matching `disallow` target. With `MULTISIG=true` they only print calldata.
4. **Propose ONE Safe batch** in the Transaction Builder (follow `docs/0_internal/7_b_1_…md`; "Use Implementation ABI" on proxy `0xD0696d3e…8BC0`, value 0):
   - `allowVerifyingProgram(NEW_VK, 1)`, selector `0xd5d8d0a8`
   - `disallowVerifyingProgram(0x00d6e32a…77af, 1)`, selector `0x55a9a7bf`. Safe to include now because the v5 route is frozen, so the old VK is already unusable. Already-verified merkle roots are unaffected.
   - **No `setSP1VerifierAddress` and no `upgradeTo` on mainnet.**
5. Simulate in the Safe UI, then collect 5 of 8 signatures. Signers check the selectors, the `NEW_VK` against `programs_ids.json` at the release tag, the target proxy, and value 0. Execute when signatures are in. This is harmless before the cutover, since nothing can produce a `NEW_VK` proof until v6 is deployed.

**Cutover (after the Safe tx has executed and Hoodi is healthy):**
6. Run `make postgres_migrations ENV=mainnet`, redeploy the gateway and task sender with the v6 release, record `CUTOVER_TS`, and run `mark_sp1_v5_tasks_failed.sql`.
7. On the Paperspace GPU host:
   - Run `sp1up -v 6.8.1` and reinstall `proof_aggregator_gpu` from the release tag.
   - Update `~/config/config-proof-aggregator-sp1.yaml` (`sp1_chunk_aggregator_vk_hash: NEW_VK`).
   - Pre-fetch `sp1-gpu-server` and the `~/.sp1/circuits/groth16` v6.1.0 artifacts.
   - Confirm the VRAM requirement.
8. Trigger `.github/workflows/aggregation_mode.yml` with `workflow_dispatch` instead of waiting for the 15:00 UTC cron. Watch for `AggregatedProofVerified` on `0xD0696d3e…`.
9. Tell users: SP1 Aggregation Mode now requires sp1-sdk ≥ 6.1.0 (we're on 6.8.1) and the new `agg_mode_sdk` release. v5 submissions made before `CUTOVER_TS` were marked `failed` and need to be re-proved and resubmitted.

## Verification
- **Local:**
  - `cargo build/clippy/test` in `aggregation_mode/` and `examples/l2`.
  - `programs_ids.json`: SP1 VKs changed, RISC0 IDs unchanged.
- **Devnet E2E:**
  1. Start the anvil devnet with the regenerated state (gateway plus v6.1.0 verifier, `NEW_VK` allowlisted).
  2. Run the migrations.
  3. Start the gateway and submit `sp1_fibonacci_6_1_0` with `make agg_mode_gateway_send_sp1_proof`.
  4. Submitting a v5 proof must return a 4xx.
  5. Run the aggregator with `--features prove` (real Groth16, CPU) and check `make verify_aggregated_proof_sp1`.
- **Migration:** apply 003 on a DB copy and run the failed-marking SQL. Restart the aggregator and confirm it doesn't pick up `failed` rows, and that the gateway status endpoint returns `failed`.
- **On-chain:** check with `cast call` after each step (Part B.5). Run the mainnet fork rehearsal (Part C.2) before signing. Watch the first live mainnet `AggregatedProofVerified` event.

## Out of scope / noted
- Verification Layer (`crates/batcher`, `operator/sp1`, `crates/sdk`, examples other than `l2`) stays on SP1 v5. Its off-chain verification relies on a version Succinct froze on-chain. Track that separately.
- Noticed but not fixed:
  - In `aggregation_mode/sdk/src/blockchain/types.rs:71-81`, `TryFrom<u16>` maps 0→SP1 and 1→RISC0, while the enum uses 1 and 2.
  - The infra config templates lack `db_connection_urls` and `monthly_budget_eth`.

# Unlocked Funds Test Script

## Overview

The `test_unlocked_funds.sh` script tests the unlocked funds flow in the Aligned protocol by continuously cycling between locking and unlocking funds while submitting proofs.

## Usage

**Important: This script must be run from the root of the repository.**

```bash
./scripts/unlocked_funds/test_unlocked_funds.sh <env_file_path>
```

## Required Environment Variables

The script requires an environment file with the following variables:

- `RPC_URL` - RPC endpoint URL for the blockchain network
- `NETWORK` - Network identifier (e.g., holesky, mainnet)
- `BATCHER_PAYMENT_SERVICE_ADDRESS` - Contract address of the batcher payment service
- `PRIVATE_KEY` - Private key for transaction signing
- `SLEEP_SECONDS` - Sleep duration between cycles (in seconds)

## How It Works

1. **Initial Setup**: Locks funds using the batcher payment service contract
2. **Continuous Loop**:
   - Submits a proof to Aligned (runs in background)
   - Waits 5 minutes
   - Unlocks funds from the contract
   - Sleeps for the configured duration
   - Locks funds again
   - Repeats the cycle

## Test Files

The script uses test files located at:
- `./scripts/test_files/circom_groth16_bn256_script/proof.json`
- `./scripts/test_files/circom_groth16_bn256_script/public.json`
- `./scripts/test_files/circom_groth16_bn256_script/verification_key.json`

## Success Criteria

The test is considered successful when the `UserFundsUnlocked` event is detected in the proof submission output.

## Dependencies

- `aligned` CLI tool
- `cast` (from Foundry)
- Bash shell environment

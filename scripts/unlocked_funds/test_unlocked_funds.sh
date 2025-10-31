#!/bin/bash

# ENV VARIABLES:
# - RPC_URL
# - NETWORK
# - BATCHER_PAYMENT_SERVICE_ADDRESS
# - PRIVATE_KEY
# - SLEEP_SECONDS

# Load env file from $1 path
source "$1"

### FUNCTIONS ###

# Function to log with timestamp and severity
function log() {
  local severity=$1
  local message=$2
  local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
  
  echo "[$timestamp] [$severity] $message"
}

# Function to send a proof to Aligned
function send_proof() {
  log "INFO" "⏳ Sending proof to Aligned..."

  submit=$(aligned submit \
    --rpc_url $RPC_URL \
    --network $NETWORK \
    --private_key $PRIVATE_KEY \
    --proving_system CircomGroth16Bn256 \
    --proof ./scripts/test_files/circom_groth16_bn256_script/proof.json \
    --public_input ./scripts/test_files/circom_groth16_bn256_script/public.json \
    --vk ./scripts/test_files/circom_groth16_bn256_script/verification_key.json \
    --custom_fee_estimate 64 \
    2>&1)
  
  # Check if UserFundsUnlocked appears in the output
  if echo "$submit" | grep -q "UserFundsUnlocked"; then
    log "INFO" "✅ Test successful - UserFundsUnlocked event detected"
  else
    log "ERROR" "❌ Test failed - UserFundsUnlocked event not detected"
    log "ERROR" "Submit output: $submit"
  fi
}


# Function to unlock funds
function unlock_funds() {
  log "INFO" "⏳ Unlocking funds..."
  output=$(cast send $BATCHER_PAYMENT_SERVICE_ADDRESS "unlock()" \
    --rpc-url $RPC_URL \
    --private-key $PRIVATE_KEY 2>&1)
  
  if echo "$output" | grep -q "status.*1 (success)"; then
    log "INFO" "✅ Unlock transaction successful"
  else
    log "ERROR" "❌ Unlock transaction failed"
  fi
}

# Function to lock funds
function lock_funds() {
  log "INFO" "⏳ Locking funds..."
  output=$(cast send $BATCHER_PAYMENT_SERVICE_ADDRESS "lock()" \
    --rpc-url $RPC_URL \
    --private-key $PRIVATE_KEY 2>&1)
  
  if echo "$output" | grep -q "status.*1 (success)"; then
    log "INFO" "✅ Lock transaction successful"
  else
    log "ERROR" "❌ Lock transaction failed"
  fi
}

### MAIN LOOP ###
log "INFO" "🚀 Running..."
lock_funds
while true
do
  send_proof &
  sleep 300
  unlock_funds
  sleep $SLEEP_SECONDS
  lock_funds
done

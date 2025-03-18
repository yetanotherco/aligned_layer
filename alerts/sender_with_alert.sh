#!/bin/bash

# ENV VARIABLES:
# - REPETITIONS
# - EXPLORER_URL
# - SENDER_ADDRESS
# - RPC_URL
# - EXPLORER_URL
# - NETWORK
# - PRIVATE_KEY
# - VERIFICATION_WAIT_TIME
# - LOGS_BLOCK_RANGE
# - SLEEP_TIME
# - PAGER_DUTY_KEY
# - PAGER_DUTY_EMAIL
# - PAGER_DUTY_SERVICE_ID
# - SLACK_WEBHOOK_URL

# TODO (Improvement):
# 1. This script waits VERIFICATION_WAIT_TIME seconds before fetching the explorer for the response tx hash.
#    We should instead poll in a loop until the batch is marked as verified

# ACKNOWLEDGMENTS
#
# Special thanks to AniV for their contribution on StackExchange regarding AWK formatting with floating point operations: 
# https://unix.stackexchange.com/questions/292087/how-do-i-get-bc-to-start-decimal-fractions-with-a-leading-zero


# Load env file from $1 path
source "$1"

# Just for debugging
#set -ex

### FUNCTIONS ###

# Function to get the tx cost from the tx hash
# @param tx_hash
function fetch_tx_cost() {
  if [[ -z "$1" ]]; then
    echo 0
  else
    # Get the tx receipt from the blockchain
    receipt=$(cast receipt --rpc-url $RPC_URL $1)
    # Parse the gas used and gas price
    gas_price=$(echo "$receipt" | grep "effectiveGasPrice" | awk '{ print $2 }')
    gas_used=$(echo "$receipt" | grep "gasUsed" | awk '{ print $2 }')
    # Calculate fee in wei
    fee_in_wei=$(($gas_price * $gas_used))

    echo $fee_in_wei
  fi
}

# Function to get the tx cost from the tx hash
# @param tx_hash
function get_number_proofs_in_batch_from_create_task_tx() {
  if [[ -z "$1" ]]; then
    echo 0
  else
    # Get the tx receipt from the blockchain
    calldata=$(cast tx $1 --rpc-url $RPC_URL input)
    decoded_calldata=$(cast --calldata-decode --json "createNewTask(bytes32 batchMerkleRoot, string batchDataPointer, address[] proofSubmitters, uint256 feeForAggregator, uint256 feePerProof, uint256 respondToTaskFeeLimit)" $calldata)
    # We count the number of proofSubmitters within the tx which corresponds to the number of proofs sent in the last batch
    number_proofs_in_batch=$(echo $decoded_calldata | jq '.[2] | [ match(","; "g")] | length + 1')

    echo $number_proofs_in_batch
  fi
}

# Function to send PagerDuty alert
# @param message
function send_pagerduty_alert() {
  . alerts/pagerduty.sh "$1"
}

# Function so send Slack message
# @param message
function send_slack_message() {
  . alerts/slack.sh "$1"
}

#################
while true
do

  ## Remove Proof Data
  rm -rf ./scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/*
  rm -rf ./aligned_verification_data/*

  mkdir -p ./scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs

  ## Generate Proof
  nonce=$(aligned get-user-nonce --network $NETWORK --user_addr $SENDER_ADDRESS 2>&1 | awk '{print $9}')
  echo $nonce
  if ! [[ "$nonce" =~ ^[0-9]+$ ]]; then
    echo "Failed getting user nonce, retrying in 10 seconds"
    sleep 10
    continue
  fi

  x=$((nonce + 1)) # So we don't have any issues with nonce = 0
  echo "Generating proof $x != 0, nonce: $nonce"
  go run ./scripts/test_files/gnark_groth16_bn254_infinite_script/cmd/main.go $x

  ## Send Proof
  echo "Submitting $REPETITIONS proofs $x != 0"
  submit=$(aligned submit \
    --proving_system Groth16Bn254 \
    --repetitions $REPETITIONS \
    --proof "./scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_${x}_groth16.proof" \
    --public_input "./scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_${x}_groth16.pub" \
    --vk "./scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/ineq_${x}_groth16.vk" \
    --proof_generator_addr $SENDER_ADDRESS \
    --private_key $PRIVATE_KEY \
    --rpc_url $RPC_URL \
    --network $NETWORK \
    --max_fee 0.004ether \
    2>&1)

  echo "$submit"

  submit_errors=$(echo "$submit" | grep -oE 'ERROR[^]]*]([^[]*)' | sed 's/^[^]]*]//;s/[[:space:]]*$//')

  # Loop through each error found and print with the custom message
  is_error=0
  while IFS= read -r error; do
      if [[ -n "$error" ]]; then
          slack_error_message="Error submitting proof to $NETWORK: $error"
          send_slack_message "$slack_error_message"
          is_error=1
      fi
  done <<< "$submit_errors"

  if [ $is_error -eq 1 ]; then
    echo "Error submitting proofs to $NETWORK, retrying in 60 seconds"
    send_slack_message "Error submitting proofs to $NETWORK, retrying in 60 seconds"
    sleep 60
    continue
  fi

  echo "Waiting $VERIFICATION_WAIT_TIME seconds for verification"
  sleep $VERIFICATION_WAIT_TIME

  # Get all the batches merkle roots
  batch_merkle_roots=$(echo "$submit" | grep "Batch merkle root: " | grep -oE "0x[[:alnum:]]{64}" | uniq)

  # Fetch the logs of both submission and response
  current_block_number=$(cast block-number --rpc-url $RPC_URL)
  from_block_number=$(($current_block_number - LOGS_BLOCK_RANGE))
  if [ $from_block_number -lt 0 ]; then
    from_block_number=0
  fi

  total_fee_in_wei=0
  total_number_proofs=0
  batch_explorer_urls=()
  for batch_merkle_root in $batch_merkle_roots 
  do
    # Construct the batcher explorer url
    batch_explorer_url="$EXPLORER_URL/batches/$batch_merkle_root"
    batch_explorer_urls+=($batch_explorer_url)

    log=$(cast logs --rpc-url $RPC_URL --from-block $from_block_number --to-block latest 'NewBatchV3 (bytes32 indexed batchMerkleRoot, address senderAddress, uint32 taskCreatedBlock, string batchDataPointer, uint256 respondToTaskFeeLimit)' $batch_merkle_root)
    submission_tx_hash=$(echo "$log" | grep -oE "transactionHash: 0x[[:alnum:]]{64}" | awk '{ print $2 }')

    log=$(cast logs --rpc-url $RPC_URL --from-block $from_block_number --to-block latest 'BatchVerified (bytes32 indexed batchMerkleRoot, address senderAddress)' $batch_merkle_root)
    response_tx_hash=$(echo "$log" | grep -oE "transactionHash: 0x[[:alnum:]]{64}" | awk '{ print $2 }')

    # Calculate fees for transactions
    number_proofs_in_batch=$(get_number_proofs_in_batch_from_create_task_tx $submission_tx_hash)
    submission_fee_in_wei=$(fetch_tx_cost $submission_tx_hash)
    response_fee_in_wei=$(fetch_tx_cost $response_tx_hash)
    batch_fee_in_wei=$((submission_fee_in_wei + response_fee_in_wei))

    # Accumulate the fee
    total_fee_in_wei=$(($total_fee_in_wei + $batch_fee_in_wei))

    # Accumulate proofs in batch
    total_number_proofs=$(($total_number_proofs + $number_proofs_in_batch))
  done

  # Calculate the spent amount by converting the fee to ETH
  wei_to_eth_division_factor=$((10**18))
  spent_amount=$(echo "scale=30; $total_fee_in_wei / (10^18)" | bc -l | awk '{printf "%.15f", $0}')

  eth_usd=$(curl -s https://cryptoprices.cc/ETH/)
  spent_amount_usd=$(echo "$spent_amount * $eth_usd" | bc | awk '{printf "%.2f", $1}')

  slack_messsage=""
  verified=1

  ## Verify Proofs
  echo "Verifying $REPETITIONS proofs $x != 0"
  for proof in ./aligned_verification_data/*.cbor; do
    ## Validate Proof on Chain
    verification=$(aligned verify-proof-onchain \
      --aligned-verification-data $proof \
      --rpc_url $RPC_URL \
      --network $NETWORK \
      2>&1)

    ## Send Alert if Verification Fails
    if echo "$verification" | grep -q not; then
      message="Proof verification failed for $proof [ ${batch_explorer_urls[@]} ]"
      echo "$message"
      send_pagerduty_alert "$message"
      verified=0 # Some proofs failed, so we should not send the success message
      break
    elif echo "$verification" | grep -q verified; then
      echo "Proof verification succeeded for $proof"
    fi
  done

  if [ $verified -eq 1 ]; then
    slack_message="$total_number_proofs proofs submitted and verified. We sent $REPETITIONS proofs. Spent amount: $spent_amount ETH ($ $spent_amount_usd) [ ${batch_explorer_urls[@]} ]"
  else
    slack_message="$total_number_proofs proofs submitted but not verified. We sent $REPETITIONS proofs. Spent amount: $spent_amount ETH ($ $spent_amount_usd) [ ${batch_explorer_urls[@]} ]"
  fi

  ## Send Update to Slack
  echo "$slack_message"
  send_slack_message "$slack_message"

  ## Remove Proof Data
  rm -rf ./scripts/test_files/gnark_groth16_bn254_infinite_script/infinite_proofs/*
  rm -rf ./aligned_verification_data/*

  echo "Sleeping $SLEEP_TIME seconds"
  sleep $SLEEP_TIME
done

# Troubleshooting

### How to resolve the error "Error in new task subscription"

This error is caused by the operator not being able to subscribe to the task.

Make sure you have configured the RPC correctly and verify that the node is running.

The following RPC providers are known to have issues:

- [dRPC](https://drpc.org/)

### My operator is not showing up on the Aligned Explorer

The [explorer](https://explorer.alignedlayer.com/) does not update the operator status in real time. 
The explorer updates the operators' list every 1 hour.

If your operator is not showing up after 1 hour, please check the following:

- The operator is **whitelisted** on the Aligned AVS, you can run the following command:

    ```bash
    cast call \
    --rpc-url https://ethereum-holesky-rpc.publicnode.com \
    0x3aD77134c986193c9ef98e55e800B71e72835b62 \
    "isWhitelisted(address _address)(bool)" <operator_address>
    ```
  
    If the operator is whitelisted, it will return `true`.

- The operator is **registered** on the Aligned AVS:
    
    ```bash
    cast call \
    --rpc-url https://ethereum-holesky-rpc.publicnode.com \
    0xD0A725d82649f9e4155D7A60B638Fe33b3F25e3b \
    "getOperatorId(address operator)(bytes32)" <operator_address>
    ```
  
    If the operator is not registered, it will return `0x0` otherwise it will return the operator ID.

### How to resolve the error "Eth ws url or fallback is empty" or "Eth rpc url or fallback is empty"

This error is caused by the operator not being able to get the RPC urls.

Make sure you have configured the RPC correctly in the [config file](0_running_an_operator.md#step-3---update-the-configuration-for-your-specific-operator).

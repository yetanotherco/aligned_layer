# Setting Aggregator Address
This doc contains a guide on how to call `setAggregator(address)` to set the `alignedAggregator` value of the AlignedServiceManager.sol contract. 

### NOTE:
- This guide assumes the Aligned layer contracts have been sucessfully deployed and the deployment outputs are within `./contracts/script/output/<DEPLOYMENT_FOLDER>`

1. Locate the deployed Aligned Aggregator Address

The address of Aligned Aggregator can be found in `./contracts/script/output/<DEPLOYMENT_FOLDER>/aligned_deployment_output.json` within:
```
    "permissions": {
        ...
        "alignedLayerAggregator": "<AGGREGATOR_ADDRESS",
        ...
    }
```

2. Locate the Aligned Service Manager Address

The address of Aligned Service Manager can be found in `./contracts/script/output/<DEPLOYMENT_FOLDER>/aligned_deployment_output.json` within:
```
    "addresses": {
        ...
        "alignedLayerServiceManager": "<ALIGNED_SERVICE_MANAGER_ADDRESS>",
        ...
    }
```

3. Set Environment Variables

To run the make targets specified in this guide, you must first set the following env vars within `./contracts/scripts/.env.<NETWORK>`:
```
RPC_URL=<rpc_url>
PRIVATE_KEY=<aligned_service_manager_owner_private_key>
ALIGNED_SERVICE_MANAGER_ADDRESS=<aligned_service_manager_address>
```

4. Check the current value of `alignedAggregator` within AlignedServiceManager.sol

```
cast call --rpc-url <RPC_URL> $ALIGNED_SERVICE_MANAGER_ADDRESS "alignedAggregator()(address)"
```

You should see that the returned address matches the address from `./contracts/script/output/<DEPLOYMENT_FOLDER>/aligned_deployment_output.json` 

5. Change the value of `alignedAggregator` within AlignedServiceManager.sol

Set the environment variable `AGGREGATOR_ADDRESS` to the new address of the aggregator.
```
export AGGREGATOR_ADDRESS=<aggregator_address>
make set_aggregator_address
```

6. Verify the Aligned Aggreagtor Address has changed
```
cast call --rpc-url <RPC_URL> $ALIGNED_SERVICE_MANAGER_ADDRESS "alignedAggregator()(address)" 
```

You should observe that the printed address matches the address in `AGGREGATOR_ADDRESS`.
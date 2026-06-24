# Aligned Token Claim Contracts

## How to run locally

> [!IMPORTANT]
> Foundry must be installed and running.

1. Run `anvil` in one terminal.
2. Get your local **foundation** and **token distributor** wallet addresses and write them down in the `claim_contracts/script-config/config.example.json` file. **Only the `foundation` and `tokenDistributor` fields should be modified in this step**.
3. Run `make deploy-token-local DEPLOYER_PRIVATE_KEY=<deployer_private_key>` in another terminal.
4. Write down the forge script's output (the addresses of the token proxy and its admin preferably).
5. From the output of the previous step, complete the `claim_contracts/script-config/config.example.json` file with the token proxy address. **Only the `tokenProxy` field should be modified in this step**.
6. Run `make deploy-claimable DEPLOYER_PRIVATE_KEY=<deployer_private_key>`.
7. Write down the forge script's output (the addresses of the claimable proxy and its admin preferably).
8. From the output of the previous step, complete the `claim_contracts/script-config/config.example.json` file with the claimable proxy address. **Only the `claimableProxy` field should be modified in this step**.
9. Run `make claimable-update-root RPC_URL=http://localhost:8545 OWNER_PRIVATE_KEY=<foundation_private_key> AIRDROP=<claimable_proxy_address> MERKLE_ROOT=<merkle_root>`.
10. Run `make claimable-update-timestamp RPC_URL=http://localhost:8545 OWNER_PRIVATE_KEY=<foundation_private_key> AIRDROP=<claimable_proxy_address> TIMESTAMP=2733427549`.
11. Run `make approve-claimable RPC_URL=http://localhost:8545 DISTRIBUTOR_PRIVATE_KEY=<token_distributor_private_key> TOKEN=<token_proxy_address> AIRDROP=<claimable_proxy_address>`.
12. Run `make claimable-unpause RPC_URL=http://localhost:8545 OWNER_PRIVATE_KEY=<foundation_private_key> AIRDROP=<claimable_proxy_address>`.

> [!NOTE]
> You can do the previous steps in one run with `make deploy-example MERKLE_ROOT=<merkle_root> TIMESTAMP=2733427549`.
> Remember to write down the addresses of the proxies and their admins.

## How to run in Sepolia testnet

> [!IMPORTANT]
> Foundry must be installed and running.
> You must have an Etherscan API key.

1. Get your **foundation** and **token distributor** wallet addresses and write them down in the `claim_contracts/script-config/config.sepolia.json` file (as it is done in the `claim_contracts/script-config/config.example.json` file). **Only the `foundation` and `tokenDistributor` fields should be modified in this step**.
2. Run `make deploy-token-testnet DEPLOYER_PRIVATE_KEY=<deployer_private_key> RPC_URL=<sepolia_rpc_url> ETHERSCAN_API_KEY=<your_etherscan_api_key>`.
3. Write down the forge script's output (the addresses of the token proxy and its admin preferably).
4. From the output of the previous step, complete the `claim_contracts/script-config/config.sepolia.json` file with the token proxy address. **Only the `tokenProxy` field should be modified in this step**.
5. Run `make deploy-claimable-testnet DEPLOYER_PRIVATE_KEY=<deployer_private_key> RPC_URL=<sepolia_rpc_url> ETHERSCAN_API_KEY=<your_etherscan_api_key>`.
6. Write down the forge script's output (the addresses of the claimable proxy and its admin preferably).
7. From the output of the previous step, complete the `claim_contracts/script-config/config.sepolia.json` file with the claimable proxy address. **Only the `claimableProxy` field should be modified in this step**.
8. Run `make claimable-update-root RPC_URL=<sepolia_rpc_url> OWNER_PRIVATE_KEY=<foundation_private_key> AIRDROP=<claimable_proxy_address> MERKLE_ROOT=<merkle_root>`.
9. Run `make claimable-update-timestamp RPC_URL=<sepolia_rpc_url> OWNER_PRIVATE_KEY=<foundation_private_key> AIRDROP=<claimable_proxy_address> TIMESTAMP=2733427549`.
10. Run `make approve-claimable RPC_URL=<sepolia_rpc_url> DISTRIBUTOR_PRIVATE_KEY=<token_distributor_private_key> TOKEN=<token_proxy_address> AIRDROP=<claimable_proxy_address>`.
11. Run `make claimable-unpause RPC_URL=<sepolia_rpc_url> OWNER_PRIVATE_KEY=<foundation_private_key> AIRDROP=<claimable_proxy_address>`.

## Enabling Claimability

### By Calldata

> [!IMPORTANT]
>
> - This step-by-step **assumes** that the claimable proxy contract **is already deployed** and that **is already paused**. If it is not paused, the first transaction should be to pause it using this calldata `cast calldata "pause()"`.
> - This method **only** generates the necessary calldata to call the methods through transactions. It does **not** actually call the methods. This method is useful for copy-pasting the calldata into a multisig wallet.
> - Steps 1, 2, and 4 can be batched into a single transaction in a multisig wallet. This multisig must be the `ClaimableAirdrop` contract owner.
> - Step 3 must be done by the token distributor multisig as it is the one that has the tokens to be claimed.

> [!WARNING]
>
> - Double-check the data you passing into the commands, any mistake can lead to undesired behavior.

1. Update the merkle root
   ```
   // Example merkle_root = 0x97619aea42a289b94acc9fb98f5030576fa7449f1dd6701275815a6e99441927
   cast calldata "updateMerkleRoot(bytes32)" <merkle_root>
   ```
2. Update the claim time limit
   ```
   // Example timestamp = 2733427549
   cast calldata "extendClaimPeriod(uint256)" <timestamp>
   ```
3. Approve the claimable proxy contract to spend the token from the distributor (_2.6B, taking into account the 18 decimals_)
   ```
   // Example claim_proxy_address = 0x0234947ce63d1a5E731e5700b911FB32ec54C3c6
   cast calldata "approve(address,uint256)" <claim_proxy_address> 2600000000000000000000000000
   ```
4. Unpause the claimable contract (it is paused by default)
   ```
   cast calldata "unpause()"
   ```

### Local

1. Deploy the claimable contract as explained above.
2. Set the correct merkle root
   ```
   make claimable-update-root MERKLE_ROOT=<claims-merkle-root>
   ```
3. Set the correct claim time limit
   ```
   make claimable-update-timestamp TIMESTAMP=2733427549
   ```
4. Approve the claimable contract to spend the token from the distributor
   ```
   make approve-claimable
   ```
5. Unpause the claimable contract
   ```
   make claimable-unpause
   ```

or

```
make deploy-example MERKLE_ROOT=<claims-merkle-root> TIMESTAMP=2733427549
```

# Contract upgrade instructions

To upgrade a contract, first make sure you pause the contract if it's not paused already (NOTE: the erc20 cannot be paused, the claim contract can though). Once that's done, clone the `aligned_layer` repo and go into the `claim_contracts` directory:

> [!NOTE]
> The ERC20 cannot be paused. Only the claimable airdrop proxy can be paused.

```
git clone git@github.com:yetanotherco/aligned_layer.git && cd aligned_layer/claim_contracts
```

## Write the new contract implementation

This implementation will most likely be a copy paste of the old implementation, only with one or few changes. In addition to that, there is one thing that MUST be done on this new contract:

- Add a public `reinitalize function` with a `reinitializer()` modifier that takes in the next version number of the contract (the first version is `1`). As an example, if this is the first upgrade being done, you should add this function to the contract:

> [!WARNING]
> DO NOT UPDATE STORAGE VARIABLES IN THIS AND FOLLOWING UPGRADES, ONLY ADD NEW ONES.

```solidity
function reinitialize() public reinitializer(2) {
        if (!paused()) {
            _pause();
        }
    }
```

Put the new implementation in a file inside the `src` directory with an appropriate name.

## Write the deployment script

Under the `script` directory, create a new forge script (with the `.s.sol` extension) with a name like `UpgradeContract.s.sol`, with this code in it:

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import <path_to_upgrade_contract>;
import "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {ERC1967Utils} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Utils.sol";
import "forge-std/Script.sol";
import {Vm} from "forge-std/Vm.sol";
import {Utils} from "./Utils.sol";

/// @notice Upgrade contract template
contract UpgradeContract is Script {
    function run(string memory config) public {
        string memory root = vm.projectRoot();
        string memory path = string.concat(
            root,
            "/script-config/config.",
            config,
            ".json"
        );
        string memory config_json = vm.readFile(path);

        address _currentContractProxy = stdJson.readAddress(
            config_json,
            ".contractProxy"
        );

        vm.broadcast();
        <NameOfUpgradeContract> _newContract = new <NameOfUpgradeContract>();

        bytes memory _upgradeCalldata = abi.encodeCall(
            ProxyAdmin.upgradeAndCall,
            (
                ITransparentUpgradeableProxy(_currentContractProxy),
                address(_newContract),
                abi.encodeCall(<NameOfUpgradeContract>.reinitialize, ())
            )
        );

        console.log(
            "Proxy Admin to call:",
            getAdminAddress(_currentContractProxy)
        );
        console.log("Calldata of the transaction: ");
        console.logBytes(_upgradeCalldata);
    }

    function getAdminAddress(address proxy) internal view returns (address) {
        address CHEATCODE_ADDRESS = 0x7109709ECfa91a80626fF3989D68f67F5b1DD12D;
        Vm vm = Vm(CHEATCODE_ADDRESS);

        bytes32 adminSlot = vm.load(proxy, ERC1967Utils.ADMIN_SLOT);
        return address(uint160(uint256(adminSlot)));
    }
}

```

then fill in the missing parts (between `<>` brackets), putting the path to the new contract code and the name of it.

> [!IMPORTANT]
> Remember to fill the missing parts (between `<>` brackets) in the script, putting the path to the new contract code and the name of it where needed.

Go into the `config.mainnet.json` file inside the `script-config` directory and fill in the following values:

```
{
    "foundation": "",
    "contractProxy": ""
 }

```

- `foundation` is the address of the foundation safe.
- `contractProxy` is the address of the contract proxy to upgrade.

## Run the deployment script

Run the script with

```
cd script && \
	forge script <name_of_the_script.s.sol> \
	--sig "run(string)" \
	mainnet \
	--private-key <private_key> \
	--rpc-url <mainnet_rpc_url> \
	--broadcast \
	--verify \
	--etherscan-api-key <etherscan_api_key>
```

After running this script, it will show a message like this:

```
Proxy Admin to call: 0xf447FD34D97317759777E242fF64cEAe9C58Bf9A
Calldata of the transaction:
0x9623609d0000000000000000000000000234947ce63d1a5e731e5700b911fb32ec54c3c3000000000000000000000000f7ac74dbc77e1afda093598c912a6b082dabc31a000000000000000000000000000000000000000000000000000000000000006000000000000000000000000000000000000000000000000000000000000000046c2eb35000000000000000000000000000000000000000000000000000000000
```

Go into the foundation safe, create a new transaction calling the proxy admin address shown in the message with the message's calldata. Done.

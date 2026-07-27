# ALIGN Claim Contracts

This repo deploys the **AlignedToken** (ALIGN) and the **ClaimableAirdrop** contract (both behind
Transparent proxies). The airdrop is split across **Ethereum and Base** — each network has its own
claim contract and its own merkle root (the `ethereum` root and the `base` root produced by the
merkle generator in `aligned_airdrop_web`).

Each environment below is a single flow: **deploy → enable claiming**.

## Prerequisites

- [Foundry](https://book.getfoundry.sh/getting-started/installation).
- A funded deployer account private key. The mainnet targets sign with `--interactive`, so they
  prompt for the key rather than reading a variable or a keystore path.
- An Etherscan API key to verify the deployed contract (the same key works for Base via the
  Etherscan v2 API).

### Who signs what on mainnet

On mainnet every privileged action goes through a Safe, and **each chain has its own pair of safes**.
The Ethereum foundation safe may not be the Base foundation safe; the same holds for the
distributors. Don't assume they match — fill each chain's config from that chain's safes rather than
copying addresses across.

| | Ethereum mainnet | Base mainnet |
| --- | --- | --- |
| **Foundation** safe — claim contract owner + proxy admin owner | `foundation` in `config.mainnet.json` | `foundation` in `config.base-mainnet.json` |
| **Distributor** safe — holds the tokens, approves the claim contract | `tokenDistributor` in `config.mainnet.json` | `tokenDistributor` in `config.base-mainnet.json` |

Deployment is the one exception: `forge script` signs with an ordinary **hot EOA**, which needs no
privileges at all — ownership is assigned from the config's `foundation` at initialization, not from
the deployer. Don't try to run a deploy from a Safe.

## Local (anvil)

A single anvil chain that deploys the token + claim contract and turns claiming on.

One-shot (deploy everything and enable claiming):

```
make deploy-example MERKLE_ROOT=<claims-merkle-root> TIMESTAMP=<claim-deadline>
```

This runs `deploy-token` → `deploy-claimable-local` → set root → set deadline → approve → unpause.

Or step by step, if you need the deployed addresses along the way:

1. Start anvil in another terminal: `anvil`
2. Deploy the token: `make deploy-token`
3. Copy the printed token proxy address into `script-config/config.example.json`, under `tokenProxy`.
4. Deploy the claim contract: `make deploy-claimable-local`
5. Enable claiming: `make enable-claimability MERKLE_ROOT=<root> TIMESTAMP=<deadline>`

## Sepolia (testnet)

> [!NOTE]
> The ALIGN token is already deployed on both testnets (Ethereum Sepolia
> `0xd2Fd114f098b355321cB3424400f3CC6a0d75C9A`, Base Sepolia
> `0x4AAcFbc2C31598a560b285dB20966E00B73F9F81`) and the configs already point `tokenProxy` at it,
> so you only deploy and enable `ClaimableAirdrop` — on **both** chains.
>
> Need to (re)deploy the token? On Ethereum Sepolia run
> `make deploy-token-sepolia DEPLOYER_PRIVATE_KEY=<key> ETHERSCAN_API_KEY=<key>`; on Base Sepolia the
> L2 token is created through the OP bridge factory, not forge (see [`base/`](base/README.md)). Then
> put the new address under `tokenProxy` in the config.

### 1. Deploy

**Ethereum Sepolia** — fill `foundation` (contract owner) and `tokenDistributor` (the account
holding ALIGN to distribute) in `script-config/config.sepolia.json`, then:

```
make deploy-claimable-sepolia DEPLOYER_PRIVATE_KEY=<key> ETHERSCAN_API_KEY=<key>
```

**Base Sepolia** — bridge ALIGN to your `tokenDistributor` first (see [`base/`](base/README.md)),
fill `foundation` and `tokenDistributor` in `script-config/config.base-sepolia.json`, then:

```
make deploy-claimable-base-sepolia DEPLOYER_PRIVATE_KEY=<key> ETHERSCAN_API_KEY=<key>
```

Note the claimable proxy address printed for each.

### 2. Enable claiming

Run once per network (the owner/distributor are plain accounts on testnet):

```
make enable-claimability \
  AIRDROP=<claimable-proxy> TOKEN=<token-proxy> \
  MERKLE_ROOT=<network-root> TIMESTAMP=<claim-deadline> \
  OWNER_PRIVATE_KEY=<owner-key> DISTRIBUTOR_PRIVATE_KEY=<distributor-key> \
  RPC_URL=<network-rpc>
```

> [!IMPORTANT]
> Use the **ethereum** root on the Sepolia contract and the **base** root on the Base Sepolia
> contract.

This runs, in order: `updateMerkleRoot` → `extendClaimPeriod` → `approve` (2.6B by default,
override with `APPROVE_AMOUNT=`) → `unpause`. The contract must be paused for the first two steps,
which it is right after deployment. Each step is also available as its own target
(`claimable-update-root`, `claimable-update-timestamp`, `approve-claimable`, `claimable-unpause`).

## Mainnet

Both chains: **Ethereum mainnet** and **Base mainnet**. Each needs its own claim contract, its own
merkle root, and its own pair of safes (see [Who signs what on mainnet](#who-signs-what-on-mainnet)).
The ALIGN token already exists on both — you only deploy and enable `ClaimableAirdrop`.

Run Ethereum first, then Base; the two are independent apart from sharing the same
`START_TIMESTAMP` and claim deadline.

### Ethereum mainnet

#### 1. Deploy

Fill `script-config/config.mainnet.json` with the Ethereum `foundation` and `tokenDistributor` safes.
`tokenProxy` is already the mainnet ALIGN token.

```
make deploy-claimable-mainnet ETHERSCAN_API_KEY=<key>
```

Prompts for the deployer key. Note the claimable **proxy** address printed in the output (not the
implementation).

> [!NOTE]
> To deploy the token from scratch on Ethereum: `make deploy-token-mainnet ETHERSCAN_API_KEY=<key>`.
> You should not need this — the ALIGN token is already deployed.

#### 2. Enable claiming

Follow [Enable claiming from the safes](#enable-claiming-from-the-safes) with:

- `AIRDROP` = the claim proxy you just deployed, `TOKEN` = the mainnet ALIGN token
- `MERKLE_ROOT` = the **ethereum** root
- Owner steps from the **Ethereum foundation** safe, approve from the **Ethereum distributor** safe

### Base mainnet

#### 1. Token — already deployed, but its supply has to be bridged

The Base ALIGN token is an `OptimismMintableERC20` created through the OP factory, not with forge —
see [`base/`](base/README.md). It is pre-filled as `tokenProxy` in
`script-config/config.base-mainnet.json`.

Its balance on Base only exists to the extent it has been **bridged from Ethereum**. Claims call
`safeTransferFrom` on the distributor, so before enabling claiming the **Base distributor safe** must
hold at least the total claimable on Base.

#### 2. Fund the Base distributor safe

The **Ethereum distributor** safe bridges to the **Base distributor** safe. Since the two may be
different addresses, the deposit has to name the destination explicitly — otherwise the tokens land
at whatever address matches the sender on Base, which may be one nobody controls. Full walkthrough in
[`base/`](base/README.md#bridge-l1---base).

Takes ~20 minutes to land. Verify before continuing:

```
cast call <base-align-token> 'balanceOf(address)(uint256)' <base-distributor-safe> \
  --rpc-url https://mainnet.base.org
```

#### 3. Deploy

Fill `script-config/config.base-mainnet.json` with the **Base** `foundation` and `tokenDistributor`
safes, then:

```
make deploy-claimable-base-mainnet ETHERSCAN_API_KEY=<key>
```

Note the claimable proxy address printed in the output.

#### 4. Enable claiming

Follow [Enable claiming from the safes](#enable-claiming-from-the-safes) with:

- `AIRDROP` = the Base claim proxy, `TOKEN` = the Base ALIGN token
- `MERKLE_ROOT` = the **base** root
- Owner steps from the **Base foundation** safe, approve from the **Base distributor** safe
- `APPROVE_AMOUNT` = the amount actually bridged, not the 2.6B default

### Enable claiming from the safes

The owner is a Safe, so you generate the calldata for each step and execute it from the multisig
rather than sending transactions directly. This section applies to **either** chain — use that
chain's safes, root, token and claim proxy throughout.

> [!IMPORTANT]
>
> - This assumes the claim proxy is **already deployed** and **paused** (it is right after deploy).
>   If it is not paused, pause it first with `make calldata-pause`.
> - These targets only **generate calldata** to copy into a multisig transaction; they do not send
>   anything.
> - Steps 1, 2 and 4 are owner actions and can be batched in one multisig transaction. Step 3 must
>   be done by the token-distributor safe (it holds the tokens).
> - `unpause` goes **last**: `updateMerkleRoot` and `extendClaimPeriod` are `whenPaused`.

> [!WARNING]
> Double-check the data you pass into these commands — any mistake can lead to undesired behavior.
> In particular, use the **ethereum** root on the Ethereum contract and the **base** root on the
> Base contract. Crossed roots deploy and enable without complaint and fail only when a user claims,
> with `Invalid Merkle proof`.

1. Merkle root, from the foundation safe:
   `make calldata-update-merkle-root MERKLE_ROOT=<that-chain's-root>`
2. Claim deadline, from the foundation safe:
   `make calldata-update-limit-timestamp LIMIT_TIMESTAMP=<timestamp>`
3. Approve spending, from the token-distributor safe, **sent to that chain's token contract**:
   `make calldata-approve-spending CLAIM_PROXY_ADDRESS=<claimable-proxy> APPROVE_AMOUNT=<amount>`
   (`APPROVE_AMOUNT` defaults to 2.6B; on Base pass the bridged amount instead)
4. Unpause, from the foundation safe: `make calldata-unpause`

Submit each piece of calldata as a transaction from the appropriate safe.

### Verify (either chain, read-only)

```
make claimable-get-root      AIRDROP=<claim-proxy> RPC_URL=<rpc>   # that chain's root
make claimable-get-timestamp AIRDROP=<claim-proxy> RPC_URL=<rpc>   # the claim deadline
make test-airdrop            AIRDROP=<claim-proxy> RPC_URL=<rpc>   # paused()=false, owner()=safe

cast call <token> 'allowance(address,address)(uint256)' \
  <distributor-safe> <claim-proxy> --rpc-url <rpc>
```

`RPC_URL` has no mainnet default — pass `https://ethereum-rpc.publicnode.com` or
`https://mainnet.base.org` explicitly, or the read hits `http://localhost:8545`.

## Upgrades

To upgrade a contract, first make sure you pause the contract if it's not paused already. Once that's done, clone the `aligned_layer` repo and go into the `claim_contracts` directory:

> [!NOTE]
> The ERC20 cannot be paused. Only the claimable airdrop proxy can be paused.

> [!IMPORTANT]
> Upgrades are **per chain**. Ethereum mainnet and Base mainnet have separate proxies, separate proxy
> admins and separate foundation safes, so an upgrade has to be repeated on each chain with that
> chain's config (`config.mainnet.json` / `config.base-mainnet.json`), and the final transaction must
> be submitted from that chain's foundation safe. Note the upgrade script below reads a different
> shape from the deploy scripts — `foundation` plus `contractProxy` — so don't overwrite the
> `tokenDistributor`/`tokenProxy` values a redeploy would need.

```
git clone git@github.com:yetanotherco/aligned_layer.git && cd aligned_layer/claim_contracts
```

### Write the new contract implementation

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

### Write the deployment script

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

### Run the deployment script

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

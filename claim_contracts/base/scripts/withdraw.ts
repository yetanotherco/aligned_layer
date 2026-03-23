import {
  createPublicClient,
  createWalletClient,
  http,
  type Hash,
  type Chain,
} from "viem";
import { privateKeyToAccount } from "viem/accounts";
import { sepolia, baseSepolia, mainnet, base } from "viem/chains";
import {
  publicActionsL1,
  walletActionsL1,
  publicActionsL2,
  getWithdrawals,
} from "viem/op-stack";

// --- Config ---

type Network = "sepolia" | "mainnet";

function getChains(network: Network): { l1: Chain; l2: Chain } {
  return network === "sepolia"
    ? { l1: sepolia, l2: baseSepolia }
    : { l1: mainnet, l2: base };
}

function getRpcUrls(network: Network) {
  if (network === "sepolia") {
    return {
      l1: process.env.L1_SEPOLIA_RPC_URL || "https://ethereum-sepolia-rpc.publicnode.com",
      l2: process.env.BASE_SEPOLIA_RPC_URL || "https://sepolia.base.org",
    };
  }
  return {
    l1: process.env.L1_MAINNET_RPC_URL || "https://ethereum-rpc.publicnode.com",
    l2: process.env.BASE_MAINNET_RPC_URL || "https://mainnet.base.org",
  };
}

function getPrivateKey(): Hash {
  const key = process.env.USER_PRIVATE_KEY;
  if (!key) {
    console.error("Error: USER_PRIVATE_KEY not set");
    process.exit(1);
  }
  return key as Hash;
}

function createPublicClients(network: Network) {
  const chains = getChains(network);
  const rpc = getRpcUrls(network);

  const publicClientL1 = createPublicClient({
    chain: chains.l1,
    transport: http(rpc.l1),
  }).extend(publicActionsL1());

  const publicClientL2 = createPublicClient({
    chain: chains.l2,
    transport: http(rpc.l2),
  }).extend(publicActionsL2());

  return { publicClientL1, publicClientL2, chains };
}

function createClients(network: Network) {
  const { publicClientL1, publicClientL2, chains } = createPublicClients(network);
  const rpc = getRpcUrls(network);
  const account = privateKeyToAccount(getPrivateKey());

  const walletClientL1 = createWalletClient({
    account,
    chain: chains.l1,
    transport: http(rpc.l1),
  }).extend(walletActionsL1());

  return { publicClientL1, walletClientL1, publicClientL2, chains };
}

// --- Commands ---

async function prove(txHash: Hash, network: Network) {
  const { publicClientL1, walletClientL1, publicClientL2, chains } =
    createClients(network);

  console.log(`Getting withdrawal receipt for ${txHash}...`);
  const receipt = await publicClientL2.getTransactionReceipt({ hash: txHash });

  console.log("Waiting for withdrawal to be provable (~1 hour)...");
  const { output, withdrawal } = await publicClientL1.waitToProve({
    receipt,
    targetChain: chains.l2,
  });

  console.log("Building prove withdrawal args...");
  const proveArgs = await publicClientL2.buildProveWithdrawal({
    output,
    withdrawal,
  });

  console.log("Proving withdrawal on L1...");
  const proveHash = await walletClientL1.proveWithdrawal(proveArgs);

  console.log("Waiting for prove tx confirmation...");
  await publicClientL1.waitForTransactionReceipt({ hash: proveHash });

  console.log(`Withdrawal proved: ${proveHash}`);
  return proveHash;
}

async function finalize(txHash: Hash, network: Network) {
  const { publicClientL1, walletClientL1, publicClientL2, chains } =
    createClients(network);

  console.log(`Getting withdrawal receipt for ${txHash}...`);
  const receipt = await publicClientL2.getTransactionReceipt({ hash: txHash });

  console.log("Waiting for finalization period (7 days on mainnet)...");
  // Use getWithdrawalStatus instead of waitToFinalize due to a viem bug
  // with Portal v3+ (Fault Proof System) where waitToFinalize incorrectly
  // throws "Withdrawal has not been proven on L1".
  while (true) {
    const withdrawalStatus = await publicClientL1.getWithdrawalStatus({
      receipt,
      targetChain: chains.l2,
    });
    console.log(`  Status: ${withdrawalStatus}`);
    if (withdrawalStatus === "ready-to-finalize") break;
    if (withdrawalStatus === "finalized") {
      console.log("Withdrawal already finalized.");
      return;
    }
    await new Promise((resolve) => setTimeout(resolve, 10_000));
  }

  console.log("Finalizing withdrawal on L1...");
  const [withdrawal] = getWithdrawals({ logs: receipt.logs });
  const finalizeHash = await walletClientL1.finalizeWithdrawal({
    withdrawal,
    targetChain: chains.l2,
  });

  console.log("Waiting for finalize tx confirmation...");
  await publicClientL1.waitForTransactionReceipt({ hash: finalizeHash });

  console.log(`Withdrawal finalized: ${finalizeHash}`);
  return finalizeHash;
}

async function status(txHash: Hash, network: Network) {
  const { publicClientL1, publicClientL2, chains } = createPublicClients(network);

  console.log(`Getting withdrawal receipt for ${txHash}...`);
  const receipt = await publicClientL2.getTransactionReceipt({ hash: txHash });

  const withdrawalStatus = await publicClientL1.getWithdrawalStatus({
    receipt,
    targetChain: chains.l2,
  });

  console.log(`Withdrawal status: ${withdrawalStatus}`);
}

async function full(txHash: Hash, network: Network) {
  await prove(txHash, network);
  await finalize(txHash, network);
  console.log("Withdrawal complete.");
}

// --- CLI ---

function usage() {
  console.log(`Usage: npx tsx scripts/withdraw.ts <command> --tx-hash <hash> --network <sepolia|mainnet>

Commands:
  prove      Prove a withdrawal on L1 (wait ~1 hour after initiation)
  finalize   Finalize a withdrawal on L1 (wait 7 days after proving)
  full       Run prove + finalize sequentially
  status     Check withdrawal status`);
  process.exit(1);
}

function parseArgs() {
  const args = process.argv.slice(2);
  const command = args[0];

  if (!command || !["prove", "finalize", "full", "status"].includes(command)) {
    usage();
  }

  let txHash: string | undefined;
  let network: Network = "sepolia";

  for (let i = 1; i < args.length; i++) {
    if (args[i] === "--tx-hash" && args[i + 1]) {
      txHash = args[++i];
    } else if (args[i] === "--network" && args[i + 1]) {
      network = args[++i] as Network;
    }
  }

  if (!txHash) {
    console.error("Error: --tx-hash is required");
    usage();
  }

  if (!["sepolia", "mainnet"].includes(network)) {
    console.error("Error: --network must be sepolia or mainnet");
    usage();
  }

  return { command: command!, txHash: txHash as Hash, network };
}

async function main() {
  const { command, txHash, network } = parseArgs();

  console.log(`Network: ${network}`);
  console.log(`Command: ${command}`);
  console.log(`TX Hash: ${txHash}`);
  console.log("---");

  switch (command) {
    case "prove":
      await prove(txHash, network);
      break;
    case "finalize":
      await finalize(txHash, network);
      break;
    case "full":
      await full(txHash, network);
      break;
    case "status":
      await status(txHash, network);
      break;
  }
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});

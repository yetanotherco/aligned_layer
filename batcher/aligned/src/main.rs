use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

use clap::Parser;
use clap::Subcommand;
use clap::ValueEnum;
use env_logger::Env;
use ethers::prelude::*;
use ethers::utils::format_ether;
use ethers::utils::hex;
use ethers::utils::parse_ether;
use log::warn;
use log::{error, info};
use transaction::eip2718::TypedTransaction;

use aligned_sdk::errors::{AlignedError, SubmitError};
use aligned_sdk::sdk::{get_verification_key_commitment, submit_multiple, verify_proof_onchain};
use aligned_sdk::types::AlignedVerificationData;
use aligned_sdk::types::Chain;
use aligned_sdk::types::ProvingSystemId;
use aligned_sdk::types::VerificationData;

use crate::AlignedCommands::DepositToBatcher;
use crate::AlignedCommands::GetUserBalance;
use crate::AlignedCommands::GetVerificationKeyCommitment;
use crate::AlignedCommands::Submit;
use crate::AlignedCommands::VerifyProofOnchain;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct AlignedArgs {
    #[clap(subcommand)]
    pub command: AlignedCommands,
}

#[derive(Subcommand, Debug)]
pub enum AlignedCommands {
    #[clap(about = "Submit proof to the batcher")]
    Submit(SubmitArgs),
    #[clap(about = "Verify the proof was included in a verified batch on Ethereum")]
    VerifyProofOnchain(VerifyProofOnchainArgs),

    // GetVerificationKey, command name is get-vk-commitment
    #[clap(
        about = "Create verification key for proving system",
        name = "get-vk-commitment"
    )]
    GetVerificationKeyCommitment(GetVerificationKeyCommitmentArgs),
    // GetVericiationKey, command name is get-vk-commitment
    #[clap(
        about = "Deposits Ethereum in the batcher to pay for proofs",
        name = "deposit-to-batcher"
    )]
    DepositToBatcher(DepositToBatcherArgs),
    #[clap(about = "Get user balance from the batcher", name = "get-user-balance")]
    GetUserBalance(GetUserBalanceArgs),
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct SubmitArgs {
    #[arg(
        name = "Batcher address",
        long = "conn",
        default_value = "ws://localhost:8080"
    )]
    connect_addr: String,
    #[arg(name = "Proving system", long = "proving_system")]
    proving_system_flag: ProvingSystemArg,
    #[arg(name = "Proof file path", long = "proof")]
    proof_file_name: PathBuf,
    #[arg(name = "Public input file name", long = "public_input")]
    pub_input_file_name: Option<PathBuf>,
    #[arg(name = "Verification key file name", long = "vk")]
    verification_key_file_name: Option<PathBuf>,
    #[arg(name = "VM prgram code file name", long = "vm_program")]
    vm_program_code_file_name: Option<PathBuf>,
    #[arg(
        name = "Number of repetitions",
        long = "repetitions",
        default_value = "1"
    )]
    repetitions: usize,
    #[arg(
        name = "Proof generator address",
        long = "proof_generator_addr",
        default_value = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
    )] // defaults to anvil address 1
    proof_generator_addr: String,
    #[arg(
        name = "Aligned verification data directory Path",
        long = "aligned_verification_data_path",
        default_value = "./aligned_verification_data/"
    )]
    batch_inclusion_data_directory_path: String,
    #[arg(name = "Path to local keystore", long = "keystore_path")]
    keystore_path: Option<PathBuf>,
    #[arg(name = "Private key", long = "private_key")]
    private_key: Option<String>,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct DepositToBatcherArgs {
    #[arg(
        name = "Batcher Eth Address",
        long = "batcher_addr",
        default_value = "0x7969c5eD335650692Bc04293B07F5BF2e7A673C0"
    )]
    batcher_eth_address: String,
    #[arg(
        name = "Path to local keystore",
        long = "keystore_path",
        required = true
    )]
    keystore_path: Option<PathBuf>,
    #[arg(
        name = "Ethereum RPC provider address",
        long = "rpc",
        default_value = "http://localhost:8545"
    )]
    eth_rpc_url: String,
    #[arg(
        name = "The Ethereum network's name",
        long = "chain",
        default_value = "devnet"
    )]
    chain: ChainArg,
    #[arg(name = "Amount to deposit", long = "amount", required = true)]
    amount: String,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct VerifyProofOnchainArgs {
    #[arg(name = "Aligned verification data", long = "aligned-verification-data")]
    batch_inclusion_data: PathBuf,
    #[arg(
        name = "Ethereum RPC provider address",
        long = "rpc",
        default_value = "http://localhost:8545"
    )]
    eth_rpc_url: String,
    #[arg(
        name = "The Ethereum network's name",
        long = "chain",
        default_value = "devnet"
    )]
    chain: ChainArg,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct GetVerificationKeyCommitmentArgs {
    #[arg(name = "File name", long = "input")]
    input_file: PathBuf,
    #[arg(name = "Output file", long = "output")]
    output_file: Option<PathBuf>,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct GetUserBalanceArgs {
    #[arg(
        name = "Batcher Eth Address",
        long = "batcher_addr",
        default_value = "0x7969c5eD335650692Bc04293B07F5BF2e7A673C0"
    )]
    batcher_eth_address: String,
    #[arg(
        name = "Ethereum RPC provider address",
        long = "rpc",
        default_value = "http://localhost:8545"
    )]
    eth_rpc_url: String,
    #[arg(
        name = "The user's Ethereum address",
        long = "user_addr",
        required = true
    )]
    user_address: String,
}

#[derive(Debug, Clone, ValueEnum)]
enum ChainArg {
    Devnet,
    Holesky,
}

impl From<ChainArg> for aligned_sdk::types::Chain {
    fn from(chain_arg: ChainArg) -> Self {
        match chain_arg {
            ChainArg::Devnet => aligned_sdk::types::Chain::Devnet,
            ChainArg::Holesky => aligned_sdk::types::Chain::Holesky,
        }
    }
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ProvingSystemArg {
    #[clap(name = "GnarkPlonkBls12_381")]
    GnarkPlonkBls12_381,
    #[clap(name = "GnarkPlonkBn254")]
    GnarkPlonkBn254,
    #[clap(name = "Groth16Bn254")]
    Groth16Bn254,
    #[clap(name = "SP1")]
    SP1,
    #[clap(name = "Halo2KZG")]
    Halo2KZG,
    #[clap(name = "Halo2IPA")]
    Halo2IPA,
    #[clap(name = "Risc0")]
    Risc0,
}

const ANVIL_PRIVATE_KEY: &str = "2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6"; // Anvil address 9

impl From<ProvingSystemArg> for ProvingSystemId {
    fn from(proving_system: ProvingSystemArg) -> Self {
        match proving_system {
            ProvingSystemArg::GnarkPlonkBls12_381 => ProvingSystemId::GnarkPlonkBls12_381,
            ProvingSystemArg::GnarkPlonkBn254 => ProvingSystemId::GnarkPlonkBn254,
            ProvingSystemArg::Groth16Bn254 => ProvingSystemId::Groth16Bn254,
            ProvingSystemArg::SP1 => ProvingSystemId::SP1,
            ProvingSystemArg::Halo2KZG => ProvingSystemId::Halo2KZG,
            ProvingSystemArg::Halo2IPA => ProvingSystemId::Halo2IPA,
            ProvingSystemArg::Risc0 => ProvingSystemId::Risc0,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), AlignedError> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let args: AlignedArgs = AlignedArgs::parse();

    match args.command {
        Submit(submit_args) => {
            let batch_inclusion_data_directory_path =
                PathBuf::from(&submit_args.batch_inclusion_data_directory_path);

            std::fs::create_dir_all(&batch_inclusion_data_directory_path).map_err(|e| {
                SubmitError::IoError(batch_inclusion_data_directory_path.clone(), e)
            })?;

            let repetitions = submit_args.repetitions;
            let connect_addr = submit_args.connect_addr.clone();

            let keystore_path = &submit_args.keystore_path;
            let private_key = &submit_args.private_key;

            if keystore_path.is_some() && private_key.is_some() {
                warn!("Can't have a keystore path and a private key as input. Please use only one");
                return Ok(());
            }

            let wallet = if let Some(keystore_path) = keystore_path {
                let password = rpassword::prompt_password("Please enter your keystore password:")
                    .map_err(|e| SubmitError::GenericError(e.to_string()))?;
                Wallet::decrypt_keystore(keystore_path, password)
                    .map_err(|e| SubmitError::GenericError(e.to_string()))?
            } else if let Some(private_key) = private_key {
                private_key
                    .parse::<LocalWallet>()
                    .map_err(|e| SubmitError::GenericError(e.to_string()))?
            } else {
                warn!("Missing keystore used for payment. This proof will not be included if sent to Eth Mainnet");
                LocalWallet::from_str(ANVIL_PRIVATE_KEY).expect("Failed to create wallet")
            };

            let verification_data = verification_data_from_args(submit_args)?;

            let verification_data_arr = vec![verification_data; repetitions];

            info!("Submitting proofs to the Aligned batcher...");

            let aligned_verification_data_vec =
                submit_multiple(&connect_addr, &verification_data_arr, wallet).await?;

            if let Some(aligned_verification_data_vec) = aligned_verification_data_vec {
                let mut unique_batch_merkle_roots = HashSet::new();

                for aligned_verification_data in aligned_verification_data_vec {
                    save_response(
                        batch_inclusion_data_directory_path.clone(),
                        &aligned_verification_data,
                    )?;
                    unique_batch_merkle_roots.insert(aligned_verification_data.batch_merkle_root);
                }

                if unique_batch_merkle_roots.len() > 1 {
                    info!("Proofs submitted to aligned. See the batches in the explorer:");
                } else {
                    info!("Proofs submitted to aligned. See the batch in the explorer:");
                }

                for batch_merkle_root in unique_batch_merkle_roots {
                    info!(
                        "https://explorer.alignedlayer.com/batches/0x{}",
                        hex::encode(batch_merkle_root)
                    );
                }
            } else {
                error!("No batch inclusion data was received from the batcher");
            }
        }

        VerifyProofOnchain(verify_inclusion_args) => {
            let chain = verify_inclusion_args.chain.into();
            let batch_inclusion_file =
                File::open(verify_inclusion_args.batch_inclusion_data.clone()).map_err(|e| {
                    SubmitError::IoError(verify_inclusion_args.batch_inclusion_data.clone(), e)
                })?;

            let reader = BufReader::new(batch_inclusion_file);

            let aligned_verification_data: AlignedVerificationData =
                serde_json::from_reader(reader).map_err(SubmitError::SerdeError)?;

            info!("Verifying response data matches sent proof data...");
            let response = verify_proof_onchain(
                aligned_verification_data,
                chain,
                &verify_inclusion_args.eth_rpc_url,
            )
            .await?;

            if response {
                info!("Your proof was verified in Aligned and included in the batch!");
            } else {
                info!("Your proof was not included in the batch.");
            }
        }
        GetVerificationKeyCommitment(args) => {
            let content = read_file(args.input_file)?;

            let hash = get_verification_key_commitment(&content);

            info!("Commitment: {}", hex::encode(hash));
            if let Some(output_file) = args.output_file {
                let mut file = File::create(output_file.clone())
                    .map_err(|e| SubmitError::IoError(output_file.clone(), e))?;

                file.write_all(hex::encode(hash).as_bytes())
                    .map_err(|e| SubmitError::IoError(output_file.clone(), e))?;
            }
        }
        DepositToBatcher(deposit_to_batcher_args) => {
            if !deposit_to_batcher_args.amount.ends_with("ether") {
                error!("Amount should be in the format XX.XXether");
                return Ok(());
            }

            let chain: aligned_sdk::types::Chain = deposit_to_batcher_args.chain.into();

            let amount = deposit_to_batcher_args.amount.replace("ether", "");

            let eth_rpc_url = deposit_to_batcher_args.eth_rpc_url;

            let eth_rpc_provider = Provider::<Http>::try_from(eth_rpc_url).map_err(|e| {
                SubmitError::EthError(format!("Error while connecting to Ethereum: {}", e))
            })?;

            let keystore_path = &deposit_to_batcher_args.keystore_path;

            let mut wallet = if let Some(keystore_path) = keystore_path {
                let password = rpassword::prompt_password("Please enter your keystore password:")
                    .map_err(|e| SubmitError::GenericError(e.to_string()))?;
                Wallet::decrypt_keystore(keystore_path, password)
                    .map_err(|e| SubmitError::GenericError(e.to_string()))?
            } else {
                warn!("Missing keystore used for payment.");
                return Ok(());
            };

            match chain {
                Chain::Devnet => wallet = wallet.with_chain_id(31337u64),
                Chain::Holesky => wallet = wallet.with_chain_id(17000u64),
            }

            let client = SignerMiddleware::new(eth_rpc_provider.clone(), wallet.clone());

            let balance = client
                .get_balance(wallet.address(), None)
                .await
                .map_err(|e| {
                    SubmitError::EthError(format!("Error while getting balance: {}", e))
                })?;

            let amount_ether = parse_ether(&amount)
                .map_err(|e| SubmitError::EthError(format!("Error while parsing amount: {}", e)))?;

            if amount_ether <= U256::from(0) {
                error!("Amount should be greater than 0");
                return Ok(());
            }

            if balance < amount_ether {
                error!("Insufficient funds to pay to the batcher. Please deposit some Ether in your wallet.");
                return Ok(());
            }

            let batcher_addr = Address::from_str(&deposit_to_batcher_args.batcher_eth_address)
                .map_err(|e| {
                    SubmitError::EthError(format!("Error while parsing batcher address: {}", e))
                })?;

            let tx = TransactionRequest::new()
                .to(batcher_addr)
                .value(amount_ether)
                .from(wallet.address());

            info!("Sending {} ether to the batcher", amount);

            let tx = client
                .send_transaction(tx, None)
                .await
                .map_err(|e| {
                    SubmitError::EthError(format!("Error while sending transaction: {}", e))
                })?
                .await
                .map_err(|e| {
                    SubmitError::EthError(format!("Error while sending transaction: {}", e))
                })?;

            if let Some(tx) = tx {
                info!(
                    "Payment sent to the batcher successfully. Tx: 0x{:x}",
                    tx.transaction_hash
                );
            } else {
                error!("Transaction failed");
            }
        }
        GetUserBalance(get_user_balance_args) => {
            let eth_rpc_url = get_user_balance_args.eth_rpc_url;

            let eth_rpc_provider = Provider::<Http>::try_from(eth_rpc_url).map_err(|e| {
                SubmitError::EthError(format!("Error while connecting to Ethereum: {}", e))
            })?;

            let user_address =
                Address::from_str(&get_user_balance_args.user_address).map_err(|e| {
                    SubmitError::EthError(format!("Error while parsing user address: {}", e))
                })?;

            let batcher_addr = Address::from_str(&get_user_balance_args.batcher_eth_address)
                .map_err(|e| {
                    SubmitError::EthError(format!("Error while parsing batcher address: {}", e))
                })?;

            let balance = get_user_balance(eth_rpc_provider, batcher_addr, user_address)
                .await
                .map_err(|e| {
                    SubmitError::EthError(format!("Error while getting user balance: {}", e))
                })?;

            info!(
                "User {} has {} ether in the batcher",
                user_address,
                format_ether(balance)
            );
        }
    }

    Ok(())
}

fn verification_data_from_args(args: SubmitArgs) -> Result<VerificationData, SubmitError> {
    let proving_system = args.proving_system_flag.into();

    // Read proof file
    let proof = read_file(args.proof_file_name)?;

    let mut pub_input: Option<Vec<u8>> = None;
    let mut verification_key: Option<Vec<u8>> = None;
    let mut vm_program_code: Option<Vec<u8>> = None;

    match proving_system {
        ProvingSystemId::SP1 => {
            vm_program_code = Some(read_file_option(
                "--vm_program",
                args.vm_program_code_file_name,
            )?);
        }
        ProvingSystemId::Risc0 => {
            vm_program_code = Some(read_file_option(
                "--vm_program",
                args.vm_program_code_file_name,
            )?);
            pub_input = Some(read_file_option(
                "--public_input",
                args.pub_input_file_name,
            )?);
        }
        ProvingSystemId::Halo2KZG
        | ProvingSystemId::Halo2IPA
        | ProvingSystemId::GnarkPlonkBls12_381
        | ProvingSystemId::GnarkPlonkBn254
        | ProvingSystemId::Groth16Bn254 => {
            verification_key = Some(read_file_option("--vk", args.verification_key_file_name)?);
            pub_input = Some(read_file_option(
                "--public_input",
                args.pub_input_file_name,
            )?);
        }
    }

    let proof_generator_addr = Address::from_str(&args.proof_generator_addr).map_err(|e| {
        SubmitError::InvalidAddress(args.proof_generator_addr.clone(), e.to_string())
    })?;

    Ok(VerificationData {
        proving_system,
        proof,
        pub_input,
        verification_key,
        vm_program_code,
        proof_generator_addr,
    })
}

fn read_file(file_name: PathBuf) -> Result<Vec<u8>, SubmitError> {
    std::fs::read(&file_name).map_err(|e| SubmitError::IoError(file_name, e))
}

fn read_file_option(param_name: &str, file_name: Option<PathBuf>) -> Result<Vec<u8>, SubmitError> {
    let file_name = file_name.ok_or(SubmitError::MissingParameter(param_name.to_string()))?;
    read_file(file_name)
}

fn save_response(
    batch_inclusion_data_directory_path: PathBuf,
    aligned_verification_data: &AlignedVerificationData,
) -> Result<(), SubmitError> {
    let batch_merkle_root = &hex::encode(aligned_verification_data.batch_merkle_root)[..8];
    let batch_inclusion_data_file_name = batch_merkle_root.to_owned()
        + "_"
        + &aligned_verification_data.index_in_batch.to_string()
        + ".json";

    let batch_inclusion_data_path =
        batch_inclusion_data_directory_path.join(batch_inclusion_data_file_name);

    let data = serde_json::to_vec(&aligned_verification_data)?;

    let mut file = File::create(&batch_inclusion_data_path)
        .map_err(|e| SubmitError::IoError(batch_inclusion_data_path.clone(), e))?;
    file.write_all(data.as_slice())
        .map_err(|e| SubmitError::IoError(batch_inclusion_data_path.clone(), e))?;
    info!(
        "Batch inclusion data written into {}",
        batch_inclusion_data_path.display()
    );

    Ok(())
}

pub async fn get_user_balance(
    provider: Provider<Http>,
    contract_address: Address,
    user_address: Address,
) -> Result<U256, ProviderError> {
    let selector = &ethers::utils::keccak256("UserBalances(address)".as_bytes())[..4];

    let encoded_params = ethers::abi::encode(&[ethers::abi::Token::Address(user_address)]);

    let mut call_data = selector.to_vec();
    call_data.extend_from_slice(&encoded_params);

    let tx = TypedTransaction::Legacy(TransactionRequest {
        to: Some(NameOrAddress::Address(contract_address)),
        data: Some(Bytes(call_data.into())),
        ..Default::default()
    });

    let result = provider.call_raw(&tx).await?;

    if result.len() == 32 {
        let balance = U256::from_big_endian(&result);
        Ok(balance)
    } else {
        Err(ProviderError::CustomError(
            "Invalid response from contract".to_string(),
        ))
    }
}

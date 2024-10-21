use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;

use aligned_sdk::communication::serialization::cbor_deserialize;
use aligned_sdk::core::{
    errors::{AlignedError, SubmitError},
    types::{AlignedVerificationData, Network, ProvingSystemId, VerificationData},
};
use aligned_sdk::sdk::get_chain_id;
use aligned_sdk::sdk::get_next_nonce;
use aligned_sdk::sdk::{deposit_to_aligned, get_balance_in_aligned};
use aligned_sdk::sdk::{get_vk_commitment, is_proof_verified, save_response, submit_multiple};
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

use crate::AlignedCommands::DepositToBatcher;
use crate::AlignedCommands::GetUserBalance;
use crate::AlignedCommands::GetVkCommitment;
use crate::AlignedCommands::Submit;
use crate::AlignedCommands::VerifyProofOnchain;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct AlignedArgs {
    #[clap(subcommand)]
    pub command: AlignedCommands,
}

#[allow(clippy::large_enum_variant)]
#[derive(Subcommand, Debug)]
pub enum AlignedCommands {
    #[clap(about = "Submit proof to the batcher")]
    Submit(SubmitArgs),
    #[clap(about = "Verify the proof was included in a verified batch on Ethereum")]
    VerifyProofOnchain(VerifyProofOnchainArgs),
    #[clap(about = "Get commitment for file", name = "get-vk-commitment")]
    GetVkCommitment(GetVkCommitmentArgs),
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
        name = "Batcher connection address",
        long = "batcher_url",
        default_value = "ws://localhost:8080"
    )]
    batcher_url: String,
    #[arg(
        name = "Ethereum RPC provider connection address",
        long = "rpc_url",
        default_value = "http://localhost:8545"
    )]
    eth_rpc_url: String,
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
    #[arg(
        name = "Max Fee",
        long = "max_fee",
        default_value = "1300000000000000" // 13_000 gas per proof * 100 gwei gas price (upper bound)
    )]
    max_fee: String, // String because U256 expects hex
    #[arg(name = "Nonce", long = "nonce")]
    nonce: Option<String>, // String because U256 expects hex
    #[arg(
        name = "The working network's name",
        long = "network",
        default_value = "devnet"
    )]
    network: NetworkArg,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct DepositToBatcherArgs {
    #[arg(
        name = "Path to local keystore",
        long = "keystore_path",
        required = true
    )]
    keystore_path: Option<PathBuf>,
    #[arg(
        name = "Ethereum RPC provider address",
        long = "rpc_url",
        default_value = "http://localhost:8545"
    )]
    eth_rpc_url: String,
    #[arg(
        name = "The working network's name",
        long = "network",
        default_value = "devnet"
    )]
    network: NetworkArg,
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
        long = "rpc_url",
        default_value = "http://localhost:8545"
    )]
    eth_rpc_url: String,
    #[arg(
        name = "The working network's name",
        long = "network",
        default_value = "devnet"
    )]
    network: NetworkArg,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct GetVkCommitmentArgs {
    #[arg(name = "Verification key file path", long = "verification_key_file")]
    verification_key_file: PathBuf,
    #[arg(name = "Proving system", long = "proving_system")]
    proving_system: ProvingSystemArg,
    #[arg(name = "Output file", long = "output")]
    output_file: Option<PathBuf>,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct GetUserBalanceArgs {
    #[arg(
        name = "The working network's name",
        long = "network",
        default_value = "devnet"
    )]
    network: NetworkArg,
    #[arg(
        name = "Ethereum RPC provider address",
        long = "rpc_url",
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

#[derive(Debug, Clone, ValueEnum, Copy)]
enum NetworkArg {
    Devnet,
    Holesky,
    HoleskyStage,
}

impl From<NetworkArg> for Network {
    fn from(env_arg: NetworkArg) -> Self {
        match env_arg {
            NetworkArg::Devnet => Network::Devnet,
            NetworkArg::Holesky => Network::Holesky,
            NetworkArg::HoleskyStage => Network::HoleskyStage,
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

            let max_fee =
                U256::from_dec_str(&submit_args.max_fee).map_err(|_| SubmitError::InvalidMaxFee)?;

            let repetitions = submit_args.repetitions;
            let connect_addr = submit_args.batcher_url.clone();

            let keystore_path = &submit_args.keystore_path;
            let private_key = &submit_args.private_key;

            if keystore_path.is_some() && private_key.is_some() {
                warn!("Can't have a keystore path and a private key as input. Please use only one");
                return Ok(());
            }

            let mut wallet = if let Some(keystore_path) = keystore_path {
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
                match LocalWallet::from_str(ANVIL_PRIVATE_KEY) {
                    Ok(wallet) => wallet,
                    Err(e) => {
                        warn!(
                            "Failed to create wallet from anvil private key: {}",
                            e.to_string()
                        );
                        return Ok(());
                    }
                }
            };

            let eth_rpc_url = submit_args.eth_rpc_url.clone();

            let chain_id = get_chain_id(eth_rpc_url.as_str()).await?;
            wallet = wallet.with_chain_id(chain_id);

            let nonce = match &submit_args.nonce {
                Some(nonce) => U256::from_dec_str(nonce).map_err(|_| SubmitError::InvalidNonce)?,
                None => {
                    get_nonce(
                        &eth_rpc_url,
                        wallet.address(),
                        submit_args.network.into(),
                        repetitions,
                    )
                    .await?
                }
            };

            let verification_data = verification_data_from_args(&submit_args)?;

            let verification_data_arr = vec![verification_data; repetitions];

            info!("Submitting proofs to the Aligned batcher...");

            let max_fees = vec![max_fee; repetitions];

            let aligned_verification_data_vec = match submit_multiple(
                &connect_addr,
                submit_args.network.into(),
                &verification_data_arr,
                &max_fees,
                wallet.clone(),
                nonce,
            )
            .await
            {
                Ok(aligned_verification_data_vec) => aligned_verification_data_vec,
                Err(e) => {
                    let nonce_file = format!("nonce_{:?}.bin", wallet.address());

                    handle_submit_err(e, nonce_file.as_str()).await;
                    return Ok(());
                }
            };

            let mut unique_batch_merkle_roots = HashSet::new();

            for aligned_verification_data in aligned_verification_data_vec {
                save_response(
                    batch_inclusion_data_directory_path.clone(),
                    &aligned_verification_data,
                )?;
                unique_batch_merkle_roots.insert(aligned_verification_data.batch_merkle_root);
            }

            match unique_batch_merkle_roots.len() {
                1 => info!("Proofs submitted to aligned. See the batch in the explorer:"),
                _ => info!("Proofs submitted to aligned. See the batches in the explorer:"),
            }

            for batch_merkle_root in unique_batch_merkle_roots {
                info!(
                    "https://explorer.alignedlayer.com/batches/0x{}",
                    hex::encode(batch_merkle_root)
                );
            }
        }

        VerifyProofOnchain(verify_inclusion_args) => {
            let batch_inclusion_file =
                File::open(verify_inclusion_args.batch_inclusion_data.clone()).map_err(|e| {
                    SubmitError::IoError(verify_inclusion_args.batch_inclusion_data.clone(), e)
                })?;

            let reader = BufReader::new(batch_inclusion_file);

            let aligned_verification_data: AlignedVerificationData =
                cbor_deserialize(reader).map_err(SubmitError::SerializationError)?;

            info!("Verifying response data matches sent proof data...");
            let response = is_proof_verified(
                &aligned_verification_data,
                verify_inclusion_args.network.into(),
                &verify_inclusion_args.eth_rpc_url,
            )
            .await?;

            if response {
                info!("Your proof was verified in Aligned and included in the batch!");
            } else {
                info!("Your proof was not included in the batch.");
            }
        }
        GetVkCommitment(args) => {
            let verification_key_bytes = read_file(args.verification_key_file)?;
            let proving_system = args.proving_system.into();

            let vk_commitment = get_vk_commitment(&verification_key_bytes, proving_system);

            info!("Commitment: {}", hex::encode(vk_commitment));
            if let Some(output_file) = args.output_file {
                let mut file = File::create(output_file.clone())
                    .map_err(|e| SubmitError::IoError(output_file.clone(), e))?;

                file.write_all(hex::encode(vk_commitment).as_bytes())
                    .map_err(|e| SubmitError::IoError(output_file.clone(), e))?;
            }
        }
        DepositToBatcher(deposit_to_batcher_args) => {
            if !deposit_to_batcher_args.amount.ends_with("ether") {
                error!("Amount should be in the format XX.XXether");
                return Ok(());
            }

            let amount = deposit_to_batcher_args.amount.replace("ether", "");

            let amount_ether = parse_ether(&amount).map_err(|e| {
                SubmitError::EthereumProviderError(format!("Error while parsing amount: {}", e))
            })?;

            let eth_rpc_url = deposit_to_batcher_args.eth_rpc_url;

            let eth_rpc_provider =
                Provider::<Http>::try_from(eth_rpc_url.clone()).map_err(|e| {
                    SubmitError::EthereumProviderError(format!(
                        "Error while connecting to Ethereum: {}",
                        e
                    ))
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

            let chain_id = get_chain_id(eth_rpc_url.as_str()).await?;
            wallet = wallet.with_chain_id(chain_id);

            let client = SignerMiddleware::new(eth_rpc_provider.clone(), wallet.clone());

            match deposit_to_aligned(amount_ether, client, deposit_to_batcher_args.network.into())
                .await
            {
                Ok(receipt) => {
                    info!(
                        "Payment sent to the batcher successfully. Tx: 0x{:x}",
                        receipt.transaction_hash
                    );
                }
                Err(e) => {
                    error!("Transaction failed: {:?}", e);
                }
            }
        }
        GetUserBalance(get_user_balance_args) => {
            let user_address = H160::from_str(&get_user_balance_args.user_address).unwrap();
            match get_balance_in_aligned(
                user_address,
                &get_user_balance_args.eth_rpc_url,
                get_user_balance_args.network.into(),
            )
            .await
            {
                Ok(balance) => {
                    info!(
                        "User {} has {} ether in the batcher",
                        user_address,
                        format_ether(balance)
                    );
                }
                Err(e) => {
                    error!("Error while getting user balance: {:?}", e);
                    return Ok(());
                }
            }
        }
    }

    Ok(())
}

fn verification_data_from_args(args: &SubmitArgs) -> Result<VerificationData, SubmitError> {
    let proving_system = args.proving_system_flag.clone().into();

    // Read proof file
    let proof = read_file(args.proof_file_name.clone())?;

    let mut pub_input: Option<Vec<u8>> = None;
    let mut verification_key: Option<Vec<u8>> = None;
    let mut vm_program_code: Option<Vec<u8>> = None;

    match proving_system {
        ProvingSystemId::SP1 => {
            vm_program_code = Some(read_file_option(
                "--vm_program",
                args.vm_program_code_file_name.clone(),
            )?);
        }
        ProvingSystemId::Risc0 => {
            vm_program_code = Some(read_file_option(
                "--vm_program",
                args.vm_program_code_file_name.clone(),
            )?);

            // Risc0 and have zero or none public inputs
            pub_input = args
                .pub_input_file_name
                .clone()
                .map(read_file)
                .transpose()?;
        }
        ProvingSystemId::GnarkPlonkBls12_381
        | ProvingSystemId::GnarkPlonkBn254
        | ProvingSystemId::Groth16Bn254 => {
            verification_key = Some(read_file_option(
                "--vk",
                args.verification_key_file_name.clone(),
            )?);
            pub_input = Some(read_file_option(
                "--public_input",
                args.pub_input_file_name.clone(),
            )?);
        }
    }

    let proof_generator_addr = Address::from_str(&args.proof_generator_addr).map_err(|e| {
        SubmitError::InvalidEthereumAddress(format!("Error while parsing address: {}", e))
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

async fn handle_submit_err(err: SubmitError, nonce_file: &str) {
    match err {
        SubmitError::InvalidNonce => {
            error!("Invalid nonce. try again");
        }
        SubmitError::ProofQueueFlushed => {
            error!("Batch was reset. try resubmitting the proof");
        }
        SubmitError::InvalidProof(reason) => error!("Submitted proof is invalid: {}", reason),
        SubmitError::InsufficientBalance => {
            error!("Insufficient balance to pay for the transaction")
        }
        _ => {}
    }

    delete_file(nonce_file).unwrap_or_else(|e| {
        error!("Error while deleting nonce file: {}", e);
    });
}

fn read_file(file_name: PathBuf) -> Result<Vec<u8>, SubmitError> {
    std::fs::read(&file_name).map_err(|e| SubmitError::IoError(file_name, e))
}

fn read_file_option(param_name: &str, file_name: Option<PathBuf>) -> Result<Vec<u8>, SubmitError> {
    let file_name = file_name.ok_or(SubmitError::MissingRequiredParameter(
        param_name.to_string(),
    ))?;
    read_file(file_name)
}

fn write_file(file_name: &str, content: &[u8]) -> Result<(), SubmitError> {
    std::fs::write(file_name, content)
        .map_err(|e| SubmitError::IoError(PathBuf::from(file_name), e))
}

fn delete_file(file_name: &str) -> Result<(), io::Error> {
    std::fs::remove_file(file_name)
}

async fn get_nonce(
    eth_rpc_url: &str,
    address: Address,
    network: Network,
    proof_count: usize,
) -> Result<U256, AlignedError> {
    let nonce = get_next_nonce(eth_rpc_url, address, network).await?;

    let nonce_file = format!("nonce_{:?}.bin", address);

    let local_nonce = read_file(PathBuf::from(nonce_file.clone())).unwrap_or(vec![0u8; 32]);
    let local_nonce = U256::from_big_endian(local_nonce.as_slice());

    let nonce = if local_nonce > nonce {
        local_nonce
    } else {
        nonce
    };

    let mut nonce_bytes = [0; 32];

    (nonce + U256::from(proof_count)).to_big_endian(&mut nonce_bytes);

    write_file(nonce_file.as_str(), &nonce_bytes)?;

    Ok(nonce)
}

pub async fn get_user_balance(
    provider: Provider<Http>,
    contract_address: Address,
    user_address: Address,
) -> Result<U256, ProviderError> {
    let selector = &ethers::utils::keccak256("user_balances(address)".as_bytes())[..4];

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

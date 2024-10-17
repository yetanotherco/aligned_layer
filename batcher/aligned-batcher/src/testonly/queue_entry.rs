use aligned_sdk::core::types::{
    NoncedVerificationData, ProvingSystemId, VerificationData, VerificationDataCommitment,
};
use ethers::types::{Address, Signature, U256};

use crate::types::batch_queue::BatchQueueEntry;

impl BatchQueueEntry {
    pub async fn new_for_test(proving_system: ProvingSystemId, sender: Address) -> Self {
        let dummy_signature = Signature {
            r: U256::from(1),
            s: U256::from(2),
            v: 3,
        };
        BatchQueueEntry {
            sender,
            messaging_sink: None,
            verification_data_commitment: VerificationDataCommitment::default(),
            nonced_verification_data: NoncedVerificationData {
                verification_data: VerificationData {
                    proving_system,
                    proof: vec![],
                    pub_input: None,
                    verification_key: None,
                    vm_program_code: None,
                    proof_generator_addr: Address::default(),
                },
                nonce: U256::zero(),
                max_fee: Default::default(),
                chain_id: Default::default(),
                payment_service_addr: Default::default(),
            },
            signature: dummy_signature,
        }
    }
}

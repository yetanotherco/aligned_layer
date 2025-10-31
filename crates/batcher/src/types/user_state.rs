use ethers::types::U256;

pub(crate) struct UserState {
    pub nonce: U256,
    pub last_max_fee_limit: U256,
    pub total_fees_in_queue: U256,
    pub proofs_in_batch: usize,
}

impl UserState {
    pub(crate) fn new(nonce: U256) -> Self {
        UserState {
            nonce,
            last_max_fee_limit: U256::max_value(),
            total_fees_in_queue: U256::zero(),
            proofs_in_batch: 0,
        }
    }
}

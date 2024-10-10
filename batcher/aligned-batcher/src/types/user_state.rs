use ethers::types::U256;

pub(crate) struct UserState {
    pub nonce: U256,
    /// The minimum fee of a pending proof for a user.
    /// This should always be the fee of the biggest pending nonce by the user.
    /// This is used to check if a user is submitting a proof with a higher nonce and higher fee,
    /// which is invalid and should be rejected.
    pub min_fee: U256,
    pub proofs_in_batch: usize,
}

impl UserState {
    pub(crate) fn new(nonce: U256) -> Self {
        UserState {
            nonce,
            min_fee: U256::max_value(),
            proofs_in_batch: 0,
        }
    }
}

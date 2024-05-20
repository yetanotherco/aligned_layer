use rand::{Rng, thread_rng};
use crate::types::VerificationData;

pub fn select_random_verification_data(batch: &[VerificationData]) -> &VerificationData {
    let mut rng = thread_rng();
    let random_index = rng.gen_range(0..batch.len());
    &batch[random_index]
}

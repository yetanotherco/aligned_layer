use halo2_proofs::{
    plonk::{read_fr, read_params, verify_proof, VerifyingKey},
    poly::{
        commitment::Params,
        kzg::{
            commitment::KZGCommitmentScheme, multiopen::VerifierSHPLONK, strategy::SingleStrategy,
        },
    },
    transcript::{Blake2bRead, Challenge255, TranscriptReadBuffer},
    SerdeFormat,
};
use halo2curves::bn256::{Bn256, G1Affine};
use log::error;
use std::io::BufReader;

pub fn verify_halo2_kzg(proof: &[u8], public_input: &[u8], verification_key: &[u8]) -> bool {
    // For Halo2 the verification key contains the serialized cs, vk, and params with there respective sizes serialized as u32 values (4 bytes) => 3 * 4 bytes = 12:
    // We therefore require that the verification key is greater than 12 bytes and treat the case that buffer lengths and buffers themselves are 0 size as false.
    // [ cs_len | vk_len | vk_params_len | cs_bytes | vk_bytes | vk_params_bytes ].
    if proof.is_empty() || verification_key.len() <= 12 || public_input.is_empty() {
        error!("Input buffer length zero size");
        return false;
    }

    let Ok((cs_bytes, vk_bytes, vk_params_bytes)) = read_params(verification_key) else {
        error!("Failed to deserialize verifiation parameter buffers from parameters buffer");
        return false;
    };

    let Ok(cs) = bincode::deserialize(cs_bytes) else {
        error!("Failed to deserialize constraint system");
        return false;
    };

    let Ok(vk) =
        VerifyingKey::<G1Affine>::read(&mut BufReader::new(vk_bytes), SerdeFormat::RawBytes, cs)
    else {
        error!("Failed to deserialize verification key");
        return false;
    };

    let Ok(params) = Params::read::<_>(&mut BufReader::new(vk_params_bytes)) else {
        error!("Failed to deserialize verification parameters");
        return false;
    };

    let Ok(res) = read_fr(public_input) else {
        error!("Failed to deserialize public inputs");
        return false;
    };

    let strategy = SingleStrategy::new(&params);
    let instances = res;
    let mut transcript = Blake2bRead::<&[u8], G1Affine, Challenge255<_>>::init(proof);
    verify_proof::<
        KZGCommitmentScheme<Bn256>,
        VerifierSHPLONK<Bn256>,
        Challenge255<G1Affine>,
        Blake2bRead<&[u8], G1Affine, Challenge255<G1Affine>>,
        SingleStrategy<Bn256>,
    >(&params, &vk, strategy, &[vec![instances]], &mut transcript)
    .is_ok()
}

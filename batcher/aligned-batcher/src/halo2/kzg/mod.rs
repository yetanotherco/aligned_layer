use halo2_proofs::{
    plonk::{verify_proof, VerifyingKey},
    poly::{
        commitment::Params,
        kzg::{
            commitment::KZGCommitmentScheme, multiopen::VerifierSHPLONK, strategy::SingleStrategy,
        },
    },
    transcript::{Blake2bRead, Challenge255, TranscriptReadBuffer},
    SerdeFormat,
};
use halo2curves::bn256::{Bn256, Fr, G1Affine};
use std::io::{BufReader, ErrorKind, Read};

//TODO(pat): refactor halo2 verification to a common create to eliminate deduplicated code.

// MaxConstraintSystemSize 2KB
pub const MAX_CONSTRAINT_SYSTEM_SIZE: usize = 2 * 1024;

// MaxVerificationKeySize 1KB
pub const MAX_VERIFIER_KEY_SIZE: usize = 1024;

// MaxKzgParamsSize 4KB
pub const MAX_KZG_PARAMS_SIZE: usize = 4 * 1024;

pub fn verify_halo2_kzg(proof: &[u8], public_input: &[u8], verification_key: &[u8]) -> bool {
    let mut cs_buffer = [0u8; MAX_CONSTRAINT_SYSTEM_SIZE];
    let cs_len_buf: [u8; 4] = verification_key[..4]
        .try_into()
        .map_err(|_| "Failed to convert slice to [u8; 4]")
        .unwrap();
    let cs_len = u32::from_le_bytes(cs_len_buf) as usize;
    let cs_offset = 12;
    cs_buffer[..cs_len].clone_from_slice(&verification_key[cs_offset..(cs_offset + cs_len)]);

    // Select Verifier Key Bytes
    let mut vk_buffer = [0u8; MAX_VERIFIER_KEY_SIZE];
    let vk_len_buf: [u8; 4] = verification_key[4..8]
        .try_into()
        .map_err(|_| "Failed to convert slice to [u8; 4]")
        .unwrap();
    let vk_len = u32::from_le_bytes(vk_len_buf) as usize;
    let vk_offset = cs_offset + cs_len;
    vk_buffer[..vk_len].clone_from_slice(&verification_key[vk_offset..(vk_offset + vk_len)]);

    // Select KZG Params Bytes
    let mut kzg_params_buffer = [0u8; MAX_KZG_PARAMS_SIZE];
    let kzg_len_buf: [u8; 4] = verification_key[8..12]
        .try_into()
        .map_err(|_| "Failed to convert slice to [u8; 4]")
        .unwrap();
    let kzg_params_len = u32::from_le_bytes(kzg_len_buf) as usize;
    let kzg_offset = vk_offset + vk_len;
    kzg_params_buffer[..kzg_params_len].clone_from_slice(&verification_key[kzg_offset..]);

    if let Ok(cs) = bincode::deserialize(&cs_buffer[..]) {
        if let Ok(vk) = VerifyingKey::<G1Affine>::read(
            &mut BufReader::new(&vk_buffer[..]),
            SerdeFormat::RawBytes,
            cs,
        ) {
            if let Ok(params) = Params::read::<_>(&mut BufReader::new(&kzg_params_buffer[..])) {
                if let Ok(res) = read_fr(&public_input[..]) {
                    let strategy = SingleStrategy::new(&params);
                    let instances = res.as_slice();
                    let mut transcript =
                        Blake2bRead::<&[u8], G1Affine, Challenge255<_>>::init(&proof[..]);
                    return verify_proof::<
                        KZGCommitmentScheme<Bn256>,
                        VerifierSHPLONK<'_, Bn256>,
                        Challenge255<G1Affine>,
                        Blake2bRead<&[u8], G1Affine, Challenge255<G1Affine>>,
                        SingleStrategy<'_, Bn256>,
                    >(
                        &params, &vk, strategy, &[&[instances]], &mut transcript
                    )
                    .is_ok();
                }
            }
        }
    }
    false
}

fn read_fr(mut buf: &[u8]) -> Result<Vec<Fr>, ErrorKind> {
    let mut instances = Vec::with_capacity(buf.len() / 32);
    // Buffer to store each 32-byte slice
    let mut buffer = [0; 32];

    loop {
        // Read 32 bytes into the buffer
        match buf.read_exact(&mut buffer) {
            Ok(_) => {
                instances.push(Fr::from_bytes(&buffer).unwrap());
            }
            Err(ref e) if e.kind() == ErrorKind::UnexpectedEof => {
                // If end of file reached, break the loop
                break;
            }
            Err(e) => {
                eprintln!("Error Deserializing Public Inputs: {}", e);
                return Err(ErrorKind::Other);
            }
        }
    }

    Ok(instances)
}

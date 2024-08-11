use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{
        verify_proof, Advice, Circuit, Column, ConstraintSystem, ErrorFront, Fixed, Instance,
        VerifyingKey,
    },
    poly::{
        commitment::Params,
        kzg::{
            commitment::KZGCommitmentScheme, multiopen::VerifierSHPLONK, strategy::SingleStrategy,
        },
        Rotation,
    },
    transcript::{Blake2bRead, Challenge255, TranscriptReadBuffer},
    SerdeFormat,
};
use halo2curves::bn256::{Bn256, Fr, G1Affine};
use log::error;
use std::{
    io::{BufReader, ErrorKind, Read},
    slice,
};

#[no_mangle]
pub extern "C" fn verify_halo2_kzg_proof_ffi(
    proof_buf: *const u8,
    proof_len: u32,
    params_buf: *const u8,
    params_len: u32,
    public_input_buf: *const u8,
    public_input_len: u32,
) -> bool {
    //TODO
    // - Abstract deserialization logic to external function.

    if proof_buf.is_null()
        || params_buf.is_null()
        || public_input_buf.is_null()
    {
        error!("Input buffer length null");
        return false;
    }

    // NOTE: Params contains the cs, vk, and params with there respective sizes serialized in front as u32 values [ 12 bytes | cs_bytes | vk_bytes | vk_params_bytes ].
    if proof_len == 0 || params_len <= 12 || public_input_len == 0 {
        error!("Input buffer length zero size");
        return false;
    }

    let proof_bytes = unsafe { slice::from_raw_parts(proof_buf, proof_len as usize) };

    let params_bytes = unsafe { slice::from_raw_parts(params_buf, params_len as usize) };

    let public_input_bytes =
        unsafe { slice::from_raw_parts(public_input_buf, public_input_len as usize) };

    if let Ok((cs_bytes, vk_bytes, vk_params_bytes)) = deserialize_verification_params(&params_bytes) {
        if let Ok(cs) = bincode::deserialize(&cs_bytes) {
            if let Ok(vk) =
                VerifyingKey::<G1Affine>::read(&mut BufReader::new(vk_bytes), SerdeFormat::RawBytes, cs)
            {
                if let Ok(params) = Params::read::<_>(&mut BufReader::new(vk_params_bytes)) {
                    if let Ok(res) = read_fr(public_input_bytes) {
                        let strategy = SingleStrategy::new(&params);
                        let instances = res;
                        let mut transcript =
                            Blake2bRead::<&[u8], G1Affine, Challenge255<_>>::init(proof_bytes);
                        return verify_proof::<
                            KZGCommitmentScheme<Bn256>,
                            VerifierSHPLONK<Bn256>,
                            Challenge255<G1Affine>,
                            Blake2bRead<&[u8], G1Affine, Challenge255<G1Affine>>,
                            SingleStrategy<Bn256>,
                        >(
                            &params, &vk, strategy, &[vec![instances]], &mut transcript
                        )
                        .is_ok();
                    }
                    error!("Failed to deserialize public inputs");
                }
                error!("Failed to deserialize verification parameters");
            }
            error!("Failed to deserialize verification key");
        }
        error!("Failed to deserialize verifiation parameter buffers from parameters buffer ");
    }
    false
}

fn deserialize_verification_params(buf: &[u8]) -> Result<(&[u8], &[u8], &[u8]), ErrorKind> {
    // Deserialize
    let cs_len_buf: [u8; 4] = buf[..4]
        .try_into()
        .map_err(|_| "Failed to convert slice to [u8; 4]")
        .unwrap();
    let cs_len = u32::from_le_bytes(cs_len_buf) as usize;
    let vk_len_buf: [u8; 4] = buf[4..8]
        .try_into()
        .map_err(|_| "Failed to convert slice to [u8; 4]")
        .unwrap();
    let vk_len = u32::from_le_bytes(vk_len_buf) as usize;
    let params_len_buf: [u8; 4] = buf[8..12]
        .try_into()
        .map_err(|_| "Failed to convert slice to [u8; 4]")
        .unwrap();
    let params_len = u32::from_le_bytes(params_len_buf) as usize;

    //Verify declared lengths are less than total length.
    if (12 + cs_len + vk_len + params_len) > buf.len() {
        error!("Serialized parameter lengths greater than parameter bytes length");
        return Err(ErrorKind::Other)
    }

    // Select Constraint System Bytes
    let cs_offset = 12;
    let cs_buffer = &buf[cs_offset..(cs_offset + cs_len)];

    // Select Verifier Key Bytes
    let vk_offset = cs_offset + cs_len;
    let vk_buffer = &buf[vk_offset..(vk_offset + vk_len)];

    // Select ipa Params Bytes
    let params_offset = vk_offset + vk_len;
    let params_buffer = &buf[params_offset..(params_offset + params_len)];

    Ok((cs_buffer, vk_buffer, params_buffer))
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

// HALO2 Circuit Example
#[derive(Clone, Copy)]
struct StandardPlonkConfig {
    a: Column<Advice>,
    b: Column<Advice>,
    c: Column<Advice>,
    q_a: Column<Fixed>,
    q_b: Column<Fixed>,
    q_c: Column<Fixed>,
    q_ab: Column<Fixed>,
    constant: Column<Fixed>,
    #[allow(dead_code)]
    instance: Column<Instance>,
}

impl StandardPlonkConfig {
    fn configure(meta: &mut ConstraintSystem<Fr>) -> Self {
        let [a, b, c] = [(); 3].map(|_| meta.advice_column());
        let [q_a, q_b, q_c, q_ab, constant] = [(); 5].map(|_| meta.fixed_column());
        let instance = meta.instance_column();

        [a, b, c].map(|column| meta.enable_equality(column));

        meta.create_gate(
            "q_a·a + q_b·b + q_c·c + q_ab·a·b + constant + instance = 0",
            |meta| {
                let [a, b, c] = [a, b, c].map(|column| meta.query_advice(column, Rotation::cur()));
                let [q_a, q_b, q_c, q_ab, constant] = [q_a, q_b, q_c, q_ab, constant]
                    .map(|column| meta.query_fixed(column, Rotation::cur()));
                let instance = meta.query_instance(instance, Rotation::cur());
                Some(
                    q_a * a.clone()
                        + q_b * b.clone()
                        + q_c * c
                        + q_ab * a * b
                        + constant
                        + instance,
                )
            },
        );

        StandardPlonkConfig {
            a,
            b,
            c,
            q_a,
            q_b,
            q_c,
            q_ab,
            constant,
            instance,
        }
    }
}

#[derive(Clone, Default)]
struct StandardPlonk(Fr);

impl Circuit<Fr> for StandardPlonk {
    type Config = StandardPlonkConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<Fr>) -> Self::Config {
        StandardPlonkConfig::configure(meta)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<Fr>,
    ) -> Result<(), ErrorFront> {
        layouter.assign_region(
            || "",
            |mut region| {
                region.assign_advice(|| "", config.a, 0, || Value::known(self.0))?;
                region.assign_fixed(|| "", config.q_a, 0, || Value::known(-Fr::one()))?;

                region.assign_advice(|| "", config.a, 1, || Value::known(-Fr::from(5u64)))?;
                for (idx, column) in (1..).zip([
                    config.q_a,
                    config.q_b,
                    config.q_c,
                    config.q_ab,
                    config.constant,
                ]) {
                    region.assign_fixed(|| "", column, 1, || Value::known(Fr::from(idx as u64)))?;
                }

                let a = region.assign_advice(|| "", config.a, 2, || Value::known(Fr::one()))?;
                a.copy_advice(|| "", &mut region, config.b, 3)?;
                a.copy_advice(|| "", &mut region, config.c, 4)?;
                Ok(())
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use ff::{Field, PrimeField};
    use halo2_proofs::{
        plonk::{create_proof, keygen_pk, keygen_vk_custom, verify_proof},
        poly::kzg::{
            commitment::{KZGCommitmentScheme, ParamsKZG},
            multiopen::ProverSHPLONK,
        },
        transcript::{
            Blake2bRead, Blake2bWrite, Challenge255, TranscriptReadBuffer, TranscriptWriterBuffer,
        },
    };
    use rand_core::OsRng;
    use std::{
        fs::File,
        io::{BufWriter, Read, Write},
    };

    const PROOF: &[u8] = include_bytes!("../../../../scripts/test_files/halo2_kzg/proof.bin");

    const PUB_INPUT: &[u8] =
        include_bytes!("../../../../scripts/test_files/halo2_kzg/pub_input.bin");

    const PARAMS: &[u8] = include_bytes!("../../../../scripts/test_files/halo2_kzg/params.bin");

    #[test]
    fn halo2_serialization_works() {
        // Setup Proof Params
        let circuit = StandardPlonk(Fr::random(OsRng));
        let params = ParamsKZG::<Bn256>::setup(4, OsRng);
        let compress_selectors = true;
        let vk =
            keygen_vk_custom(&params, &circuit, compress_selectors).expect("vk should not fail");
        let pk = keygen_pk(&params, vk.clone(), &circuit).expect("pk should not fail");
        let instances = vec![vec![circuit.0]];
        let cs = vk.clone().cs;

        // Create Proof
        let mut transcript = Blake2bWrite::<_, _, Challenge255<_>>::init(vec![]);
        create_proof::<
            KZGCommitmentScheme<Bn256>,
            ProverSHPLONK<'_, Bn256>,
            Challenge255<G1Affine>,
            _,
            Blake2bWrite<Vec<u8>, G1Affine, Challenge255<_>>,
            _,
        >(
            &params,
            &pk,
            &[circuit],
            &[instances.clone()],
            OsRng,
            &mut transcript,
        )
        .expect("prover should not fail");

        let proof = transcript.finalize();
        let vk_params = params.verifier_params();
        let strategy = SingleStrategy::new(&vk_params);
        let mut transcript = Blake2bRead::<_, _, Challenge255<_>>::init(&proof[..]);

        // Verify Proof
        verify_proof::<
            KZGCommitmentScheme<Bn256>,
            VerifierSHPLONK<Bn256>,
            Challenge255<G1Affine>,
            Blake2bRead<&[u8], G1Affine, Challenge255<G1Affine>>,
            SingleStrategy<Bn256>,
        >(
            &vk_params,
            &vk,
            strategy,
            &[instances.clone()],
            &mut transcript,
        )
        .expect("verifier should not fail");

        //write proof
        std::fs::write("proof.bin", &proof[..]).expect("should succeed to write new proof");

        //write public inputs
        let f = File::create("pub_input.bin").unwrap();
        let mut writer = BufWriter::new(f);
        instances.to_vec().into_iter().flatten().for_each(|fp| {
            writer.write(&fp.to_repr()).unwrap();
        });
        writer.flush().unwrap();

        let mut vk_buf = Vec::new();
        vk.write(&mut vk_buf, SerdeFormat::RawBytes).unwrap();
        let vk_len = vk_buf.len();

        let mut kzg_params_buf = Vec::new();
        vk_params.write(&mut kzg_params_buf).unwrap();
        let kzg_params_len = kzg_params_buf.len();

        //Write everything to parameters file
        let params_file = File::create("params.bin").unwrap();
        let mut writer = BufWriter::new(params_file);
        let cs_buf = bincode::serialize(&cs).unwrap();

        //Write Parameter Lengths as u32
        writer
            .write_all(&(cs_buf.len() as u32).to_le_bytes())
            .unwrap();
        writer.write_all(&(vk_len as u32).to_le_bytes()).unwrap();
        writer
            .write_all(&(kzg_params_len as u32).to_le_bytes())
            .unwrap();

        //Write Parameters
        writer.write_all(&cs_buf).unwrap();
        writer.write_all(&vk_buf).unwrap();
        writer.write_all(&kzg_params_buf).unwrap();
        writer.flush().unwrap();

        //read proof
        let proof = std::fs::read("proof.bin").expect("should succeed to read proof");

        //read instances
        let mut f = File::open("pub_input.bin").unwrap();
        let mut buf = Vec::new();
        f.read_to_end(&mut buf).unwrap();
        let res = read_fr(&buf).unwrap();
        let instances = res;

        let mut f = File::open("params.bin").unwrap();
        let mut params_buf = Vec::new();
        f.read_to_end(&mut params_buf).unwrap();

        let (cs_bytes, vk_bytes, vk_params_bytes) = deserialize_verification_params(&params_buf).unwrap();

        let cs = bincode::deserialize(cs_bytes).unwrap();
        let vk = VerifyingKey::<G1Affine>::read(
            &mut BufReader::new(vk_bytes),
            SerdeFormat::RawBytes,
            cs,
        )
        .unwrap();
        let params =
            Params::read::<_>(&mut BufReader::new(vk_params_bytes)).unwrap();

        let strategy = SingleStrategy::new(&params);
        let mut transcript = Blake2bRead::<_, _, Challenge255<_>>::init(&proof[..]);
        assert!(verify_proof::<
            KZGCommitmentScheme<Bn256>,
            VerifierSHPLONK<Bn256>,
            Challenge255<G1Affine>,
            Blake2bRead<&[u8], G1Affine, Challenge255<G1Affine>>,
            SingleStrategy<Bn256>,
        >(
            &vk_params,
            &vk,
            strategy,
            &[vec![instances]],
            &mut transcript
        )
        .is_ok());
        std::fs::remove_file("proof.bin").unwrap();
        std::fs::remove_file("pub_input.bin").unwrap();
        std::fs::remove_file("params.bin").unwrap();
    }

    #[test]
    fn verify_halo2_plonk_proof() {

        let proof_len = PROOF.len();
        let proof_buffer = PROOF.as_ptr();

        let params_len = PARAMS.len();
        let params_bytes = PARAMS.as_ptr();

        let public_input_len = PUB_INPUT.len();
        let public_input_buffer = PUB_INPUT.as_ptr();

        let result = verify_halo2_kzg_proof_ffi(
            proof_buffer,
            proof_len as u32,
            params_bytes,
            params_len as u32,
            public_input_buffer,
            public_input_len as u32,
        );
        assert!(result)
    }

    #[test]
    fn verify_halo2_plonk_proof_aborts_with_bad_proof() {
        // Select Proof Bytes
        let proof_len = PROOF.len();
        let proof_buffer = PROOF.as_ptr();

        let params_len = PARAMS.len();
        let params_bytes = PARAMS.as_ptr();

        // Select Public Input Bytes
        let public_input_len = PUB_INPUT.len();
        let public_input_buffer = PUB_INPUT.as_ptr();

        let result = verify_halo2_kzg_proof_ffi(
            proof_buffer,
            (proof_len - 1) as u32,
            params_bytes,
            params_len as u32,
            public_input_buffer,
            public_input_len as u32,
        );
        assert!(!result)
    }
}

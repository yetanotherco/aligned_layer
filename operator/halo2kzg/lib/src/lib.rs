use std::io::{BufReader, ErrorKind, Read};
use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{
        verify_proof, Advice, Circuit, Column,
        ConstraintSystem, ErrorFront, Fixed, Instance, VerifyingKey
    },
    poly::{
        kzg::{
            commitment::KZGCommitmentScheme,
            multiopen::VerifierSHPLONK,
            strategy::SingleStrategy,
        },
        commitment::Params,
        Rotation,
    },
    transcript::{
        Blake2bRead, Challenge255, TranscriptReadBuffer,
    },
    SerdeFormat,
};
use halo2curves::bn256::{Bn256, Fr, G1Affine};

//TODO: Set these to true max bounds
pub const MAX_PROOF_SIZE: usize = 2048;

pub const MAX_KZG_PARAMS_SIZE: usize = 2308;

pub const MAX_CONSTRAINT_SYSTEM_SIZE: usize = 791;

pub const MAX_VERIFIER_KEY_SIZE: usize = 518;

pub const MAX_PUBLIC_INPUT_SIZE: usize = 1024 * 1024;

#[no_mangle]
pub extern "C" fn print_pub_len(public_input_len: usize) {
    println!("pub input len in Rust {}", public_input_len);
}

#[no_mangle]
pub extern "C" fn verify_halo2_kzg_proof_ffi(
proof_bytes: &[u8; MAX_PROOF_SIZE],
proof_len: usize,
cs_bytes: &[u8; MAX_CONSTRAINT_SYSTEM_SIZE],
cs_len: usize,
verifier_key_bytes: &[u8; MAX_VERIFIER_KEY_SIZE],
vk_len: usize,
kzg_params_bytes: &[u8; MAX_KZG_PARAMS_SIZE],
kzg_params_len: usize,
public_input_bytes: &[u8; MAX_PUBLIC_INPUT_SIZE],
public_input_len: usize,
) -> bool {
    println!("proof_len {}", proof_len);
    println!("cs_len {}", cs_len);
    println!("vk_len {}", vk_len);
    println!("kzg_params_len {}", kzg_params_len);
    println!("public_buffer_len {}", public_input_bytes.len());
    println!("public_input_len {}", public_input_len);
    //NOTE: SingleStrategy is for single proofs so that setting will not change across invocations
    if let Ok(cs) = bincode::deserialize(&cs_bytes[..cs_len]) {
        if let Ok(vk) = VerifyingKey::<G1Affine>::read(&mut BufReader::new(&verifier_key_bytes[..vk_len]), SerdeFormat::RawBytes, cs) {
            if let Ok(params) = Params::read::<_>(&mut BufReader::new(&kzg_params_bytes[..kzg_params_len])) {
                if let Ok(res) = read_fr(&public_input_bytes[..public_input_len]) {
                    let strategy = SingleStrategy::new(&params);
                    let instances = res.as_slice();
                    let mut transcript = Blake2bRead::<&[u8], G1Affine, Challenge255<_>>::init(&proof_bytes[..proof_len]);
                    return verify_proof::<
                        KZGCommitmentScheme<Bn256>,
                        VerifierSHPLONK<'_, Bn256>,
                        Challenge255<G1Affine>,
                        Blake2bRead<&[u8], G1Affine, Challenge255<G1Affine>>,
                        SingleStrategy<'_, Bn256>,
                    >(&params, &vk, strategy, &[&[instances]], &mut transcript).is_ok()
                }
            }
        }
    }
	false
}

//TODO: Clean these up
fn read_fr(mut buf: &[u8]) -> Result<Vec<Fr>, ErrorKind> {
    //TODO: make this capacity the size of the file / 32
    let mut instances = Vec::new();
    // Buffer to store each 32-byte slice
    let mut buffer = [0; 32];
    
    // Loop until end of file
    loop {
        // Read 32 bytes into the buffer
        match buf.read_exact(&mut buffer) {
            Ok(_) => {
                // Process the buffer here (printing as an example)
                instances.push(Fr::from_bytes(&buffer).unwrap());

            },
            Err(ref e) if e.kind() == ErrorKind::UnexpectedEof => {
                // If end of file reached, break the loop
                break;
            },
            Err(e) => {
                // Handle other errors
                eprintln!("Error reading file: {}", e);
                return Err(ErrorKind::Other)
            }
        }
    }
    
    Ok(instances)
}

/*
TODO:
- End-to-End ffi tests working in makefile
- Tests work with lookups
- Set Max sizes to correct size -> 1 Kb larger than tests
- Clean up error handling
- Read everything from single file
- Serialize and append length to start of parameters
*/

// HALO2 Circuit with Lookups

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

    use rand_core::OsRng;
    use std::{
        fs::File,
        io::{BufReader, BufWriter, Write, Read},
    };
    use ff::{Field, PrimeField};
    use halo2_backend::plonk::circuit::ConstraintSystemBack;
    use halo2_proofs::{
        plonk::{
            create_proof, keygen_pk, keygen_vk_custom, pk_read, verify_proof, VerifyingKey
        },
        poly::{
            kzg::{
                commitment::{KZGCommitmentScheme, ParamsKZG},
                multiopen::ProverSHPLONK,

            },
        },
        transcript::{
            Blake2bRead, Blake2bWrite, Challenge255, TranscriptReadBuffer, TranscriptWriterBuffer,
        }
    };

	#[test]
	fn halo2_serialization_works() {
        let k = 4;
        let circuit = StandardPlonk(Fr::random(OsRng));
        let params = ParamsKZG::<Bn256>::setup(k, OsRng);
        let compress_selectors = true;
        let vk = keygen_vk_custom(&params, &circuit, compress_selectors).expect("vk should not fail");
        let cs = vk.clone().cs;
        let pk = keygen_pk(&params, vk.clone(), &circuit).expect("pk should not fail");

        // write pk
        let f = File::create("pk.bin").unwrap();
        let mut writer = BufWriter::new(f);
        pk.write(&mut writer, SerdeFormat::RawBytes).unwrap();
        writer.flush().unwrap();

        // read pk
        let f = File::open("pk.bin").unwrap();
        let mut reader = BufReader::new(f);
        #[allow(clippy::unit_arg)]
        let pk = pk_read::<G1Affine, _, StandardPlonk>(
            &mut reader,
            SerdeFormat::RawBytes,
            k,
            &circuit,
            compress_selectors,
        )
        .unwrap();

        let instances: &[&[Fr]] = &[&[circuit.0]];
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
            &[circuit.clone()],
            &[instances],
            OsRng,
            &mut transcript,
        )
        .expect("prover should not fail");
        let proof = transcript.finalize();

        //write proof
        std::fs::write("proof.bin", &proof[..])
        .expect("should succeed to write new proof");

        //read proof
        let proof = std::fs::read("proof.bin").expect("should succeed to read proof");

        //write instances
        let f = File::create("pub_input.bin").unwrap();
        let mut writer = BufWriter::new(f);
        instances.to_vec().into_iter().flatten().for_each(|fp| { writer.write(&fp.to_repr()).unwrap(); });
        writer.flush().unwrap();

        //read instances
        let mut f = File::open("pub_input.bin").unwrap();
        let mut buf = Vec::new();
        f.read_to_end(&mut buf).unwrap();
        let res = read_fr(&buf).unwrap();
        let instances = res.as_slice();
        
        let mut vk_buf = Vec::new();
        vk.write(&mut vk_buf, SerdeFormat::RawBytes).unwrap();
        let vk_len = vk_buf.len();
        let mut kzg_params_buf = Vec::new();
        params.write(&mut kzg_params_buf).unwrap();
        let kzg_params_len = kzg_params_buf.len();

        //Write everything to parameters file
        let params_file = File::create("params.bin").unwrap();
        let mut writer = BufWriter::new(params_file);
        let cs_buf = bincode::serialize(&cs).unwrap();
        //Write Parameter Lengths as u32
        writer.write_all(&(cs_buf.len() as u32).to_le_bytes()).unwrap();
        writer.write_all(&(vk_len as u32).to_le_bytes()).unwrap();
        writer.write_all(&(kzg_params_len as u32).to_le_bytes()).unwrap();
        //Write Parameters
        writer.write_all(&cs_buf).unwrap();
        writer.write_all(&vk_buf).unwrap();
        writer.write_all(&kzg_params_buf).unwrap();
        writer.flush().unwrap();

        let strategy = SingleStrategy::new(&params);
        let mut transcript = Blake2bRead::<_, _, Challenge255<_>>::init(&proof[..]);
        assert!(verify_proof::<
            KZGCommitmentScheme<Bn256>,
            VerifierSHPLONK<'_, Bn256>,
            Challenge255<G1Affine>,
            Blake2bRead<&[u8], G1Affine, Challenge255<G1Affine>>,
            SingleStrategy<'_, Bn256>,
        >(
            &params,
            &vk,
            strategy,
            &[&[instances]],
            &mut transcript
        )
        .is_ok());
	}

    const PROOF: &[u8] = include_bytes!("../proof.bin");

    const PUB_INPUT: &[u8] = include_bytes!("../pub_input.bin");

    const PARAMS: &[u8] = include_bytes!("../params.bin");

	#[test]
	fn verify_halo2_proof_works() {
        // Select Proof Bytes
        let mut proof_buffer = [0u8; MAX_PROOF_SIZE];
        let proof_len = PROOF.len();
        proof_buffer[..proof_len].clone_from_slice(PROOF);

        // Select Constraint System Bytes
        let mut cs_buffer = [0u8; MAX_CONSTRAINT_SYSTEM_SIZE];
        let cs_len_buf: [u8; 4] = PARAMS[..4].try_into().map_err(|_| "Failed to convert slice to [u8; 4]").unwrap();
        let cs_len = u32::from_le_bytes(cs_len_buf) as usize;
        let cs_offset = 12;
        cs_buffer[..cs_len].clone_from_slice(&PARAMS[cs_offset..(cs_offset + cs_len)]);

        // Select Verifier Key Bytes
        let mut vk_buffer = [0u8; MAX_VERIFIER_KEY_SIZE];
        let vk_len_buf: [u8; 4] = PARAMS[4..8].try_into().map_err(|_| "Failed to convert slice to [u8; 4]").unwrap();
        let vk_len = u32::from_le_bytes(vk_len_buf) as usize;
        let vk_offset = cs_offset + cs_len;
        vk_buffer[..vk_len].clone_from_slice(&PARAMS[vk_offset..(vk_offset + vk_len)]);

        // Select KZG Params Bytes
        let mut kzg_params_buffer = [0u8; MAX_KZG_PARAMS_SIZE];
        let kzg_len_buf: [u8; 4] = PARAMS[8..12].try_into().map_err(|_| "Failed to convert slice to [u8; 4]").unwrap();
        let kzg_params_len = u32::from_le_bytes(kzg_len_buf) as usize;
        let kzg_offset = vk_offset + vk_len;
        kzg_params_buffer[..kzg_params_len].clone_from_slice(&PARAMS[kzg_offset..]);

        // Select Public Input Bytes
        let mut public_input_buffer = [0u8; MAX_PUBLIC_INPUT_SIZE];
        let public_input_len = PUB_INPUT.len();
        public_input_buffer[..public_input_len].clone_from_slice(PUB_INPUT);

        let result = verify_halo2_kzg_proof_ffi(&proof_buffer, proof_len, &cs_buffer, cs_len, &vk_buffer, vk_len, &kzg_params_buffer, kzg_params_len, &public_input_buffer, public_input_len);
        assert!(result)
	}

	#[test]
	fn verify_halo2_proof_aborts_with_bad_proof() {
        // Select Proof Bytes
        let mut proof_buffer = [42u8; MAX_PROOF_SIZE];
        let proof_len = PROOF.len();
        proof_buffer[..proof_len].clone_from_slice(PROOF);

        // Select Constraint System Bytes
        let mut cs_buffer = [0u8; MAX_CONSTRAINT_SYSTEM_SIZE];
        let cs_len_buf: [u8; 4] = PARAMS[..4].try_into().map_err(|_| "Failed to convert slice to array").unwrap();
        let cs_len = u32::from_le_bytes(cs_len_buf) as usize;
        let cs_offset = 12;
        cs_buffer[..cs_len].clone_from_slice(&PARAMS[cs_offset..(cs_offset + cs_len)]);

        // Select Verifier Key Bytes
        let mut vk_buffer = [0u8; MAX_VERIFIER_KEY_SIZE];
        let vk_len_buf: [u8; 4] = PARAMS[4..8].try_into().map_err(|_| "Failed to convert slice to array").unwrap();
        let vk_len = u32::from_le_bytes(vk_len_buf) as usize;
        let vk_offset = cs_offset + cs_len;
        vk_buffer[..vk_len].clone_from_slice(&PARAMS[vk_offset..(vk_offset + vk_len)]);

        // Select KZG Params Bytes
        let mut kzg_params_buffer = [0u8; MAX_KZG_PARAMS_SIZE];
        let kzg_len_buf: [u8; 4] = PARAMS[8..12].try_into().map_err(|_| "Failed to convert slice to array").unwrap();
        let kzg_params_len = u32::from_le_bytes(kzg_len_buf) as usize;
        let kzg_offset = vk_offset + vk_len;
        kzg_params_buffer[..kzg_params_len].clone_from_slice(&PARAMS[kzg_offset..]);

        // Select Public Input Bytes
        let mut public_input_buffer = [0u8; MAX_PUBLIC_INPUT_SIZE];
        let public_input_len = PUB_INPUT.len();
        public_input_buffer[..public_input_len].clone_from_slice(PUB_INPUT);

        let result = verify_halo2_kzg_proof_ffi(&proof_buffer, proof_len - 1, &cs_buffer, cs_len, &vk_buffer, vk_len, &kzg_params_buffer, kzg_params_len, &public_input_buffer, public_input_len);
        assert!(!result)
	}
}
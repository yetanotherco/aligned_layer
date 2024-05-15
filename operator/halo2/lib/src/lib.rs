use std::{
    fs::File,
    io::{BufReader, BufWriter, ErrorKind, Write, Read},
};

use ff::{Field, PrimeField};
use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{
        create_proof, keygen_pk, keygen_vk_custom, pk_read, vk_read, verify_proof, Advice, Circuit, Column,
        ConstraintSystem, ErrorFront, Fixed, Instance,
    },
    poly::{
        kzg::{
            commitment::{KZGCommitmentScheme, ParamsKZG},
            multiopen::{ProverGWC, VerifierGWC},
            strategy::SingleStrategy,
        },
        commitment::Params,
        Rotation,
    },
    transcript::{
        Blake2bRead, Blake2bWrite, Challenge255, TranscriptReadBuffer, TranscriptWriterBuffer,
    },
    SerdeFormat,
};
use halo2curves::bn256::{Bn256, Fr, G1Affine};
use rand_core::OsRng;


pub const MAX_PROOF_SIZE: usize = 1024;

//TODO: write this out based on size of G1Affine elements
pub const MAX_KZG_PARAMS_SIZE: usize = 4 * 1024;

//TODO: write this out based on size of G1Affine elements
pub const MAX_VERIFIER_KEY_SIZE: usize = 2 * 1024;

//TODO: write this out based on size of G1Affine elements
pub const MAX_PUBLIC_INPUT_SIZE: usize = 2 * 1024;

//NOTE(pat): For now we only support a single strategy and single parameter configuration.
//NOTE(pat): We can't use generics over FFI so we need to have multiple implementations for each field/curve -> Use Bn254
//NOTE(pat): We can't use generics over FFI so we need multiple implementations for each plonk implementation
#[no_mangle]
pub extern "C" fn verify_halo2_proof_ffi(
proof_bytes: &[u8; MAX_PROOF_SIZE],
proof_len: usize,
verifier_params_bytes: &[u8; MAX_KZG_PARAMS_SIZE],
vk_len: usize,
kzg_param_len: usize,
public_input_bytes: &[u8; MAX_PUBLIC_INPUT_SIZE],
public_input_len: usize,
) -> () {
    /*
    //NOTE: SingleStrategy is for single proofs so that setting will not change across invocations
	if let Ok(proof) = bincode::deserialize(&proof_bytes[..proof_len]) {
        //Read vk
        if let Ok(vk) = pk_read::<G1Affine, _, StandardPlonk>(&BufReader::new(&mut verifier_params_bytes[..vk_len]), SerdeFormat::RawBytes, k ) {
            if let Ok(params) = Params::read::<_>(&verifier_params_bytes[vk_len..kzg_param_len]) {
                if let Ok(public_input) = read_fr(&public_input_bytes[..public_input_len]){
                    let strategy = SingleStrategy::new(&params);
                    let mut transcript = Blake2bRead::<_, _, Challenge255<_>>::init(&proof[..]);
                    return verify_proof::<
                        KZGCommitmentScheme<Bn256>,
                        VerifierSHPLONK<'_, Bn256>,
                        Challenge255<G1Affine>,
                        Blake2bRead<&[u8], G1Affine, Challenge255<G1Affine>>,
                        SingleStrategy<'_, Bn256>,
                    >(&params, &vk, strategy, &[instances], &mut transcript).is_ok()
                }
            }
        }
	}

	false
    */
}

fn read_fr(mut file: &File) -> Result<Vec<Fr>, ErrorKind> {
    //TODO: make this capacity the size of the file / 32
    let mut instances = Vec::new();
    // Buffer to store each 32-byte slice
    let mut buffer = [0; 32];
    
    // Loop until end of file
    loop {
        // Read 32 bytes into the buffer
        match file.read_exact(&mut buffer) {
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

#[cfg(test)]
mod tests {
    use super::*;

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
        #[cfg(feature = "circuit-params")]
        type Params = ();

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

	#[test]
	fn halo2_serialization_works() {
        let k = 4;
        let circuit = StandardPlonk(Fr::random(OsRng));
        let params = ParamsKZG::<Bn256>::setup(k, OsRng);
        let compress_selectors = true;
        let vk = keygen_vk_custom(&params, &circuit, compress_selectors).expect("vk should not fail");
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

        //write vk
        let f = File::create("vk.bin").unwrap();
        let mut writer = BufWriter::new(f);
        vk.write(&mut writer, SerdeFormat::RawBytes).unwrap();
        writer.flush().unwrap();

        //read vk
        let f = File::open("vk.bin").unwrap();
        let mut reader = BufReader::new(f);
        #[allow(clippy::unit_arg)]
        let vk = vk_read::<G1Affine, _, StandardPlonk>(
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
            ProverGWC<'_, Bn256>,
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

        //write params
        let f = File::create("kzg_params.bin").unwrap();
        let mut writer = BufWriter::new(f);
        params.write(&mut writer).unwrap();
        writer.flush().unwrap();

        //read params
        let f = File::open("kzg_params.bin").unwrap();
        let mut reader = BufReader::new(f);
        let params = Params::read::<_>(&mut reader).unwrap();

        //write proof
        std::fs::write("plonk_proof.bin", &proof[..])
        .expect("should succeed to write new proof");

        //read proof
        let proof = std::fs::read("plonk_proof.bin").expect("should succeed to read proof");

        //write instances
        let f = File::create("pub_input.bin").unwrap();
        let mut writer = BufWriter::new(f);
        instances.to_vec().into_iter().flatten().for_each(|fp| { writer.write(&fp.to_repr()).unwrap(); });
        writer.flush().unwrap();

        //read instances
        let f = File::open("pub_input.bin").unwrap();
        let res = read_fr(&f).unwrap();
        let instances = res.as_slice();

        let strategy = SingleStrategy::new(&params);
        let mut transcript = Blake2bRead::<_, _, Challenge255<_>>::init(&proof[..]);
        assert!(verify_proof::<
            KZGCommitmentScheme<Bn256>,
            VerifierGWC<'_, Bn256>,
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

    const PROOF: &[u8] = include_bytes!("../plonk_proof.bin");

    //const PUB_INPUT: &[u8] = include_bytes!("../pub_input.bin");

    const VERIFIER_KEY: &[u8] = include_bytes!("../vk.bin");

    const KZG_PARAMS: &[u8] = include_bytes!("../kzg_params.bin");

    const VERIFIER_KEY_LEN: usize = 2308;

    const KZG_PARAMS_LEN: usize = 2308;


	#[test]
	fn verify_halo2_proof_works() {
        //TODO: move to writing to single params file
        // Select Proof Bytes
        let mut proof_buffer = [0u8; MAX_PROOF_SIZE];
        let proof_len = PROOF.len();
        proof_buffer[..proof_len].clone_from_slice(PROOF);

        // Select Verifier Key Bytes
        let mut vk_buffer = [0u8; MAX_VERIFIER_KEY_SIZE];
        let vk_len = VERIFIER_KEY.len();
        vk_buffer[..vk_len].clone_from_slice(VERIFIER_KEY);

        // Select KZG Params Bytes
        let mut kzg_params_buffer = [0u8; MAX_KZG_PARAMS_SIZE];
        let kzg_params_len = KZG_PARAMS.len();
        kzg_params_buffer[..kzg_params_len].clone_from_slice(KZG_PARAMS);

        //let result = verify_halo2_proof_ffi(proof_buffer, proof_len, vk_buffer, vk_len, kzg_params_buffer, kzg_param_len, pub_input, pub_input_len);
        //assert!(result)
	}

	#[test]
	fn verify_halo2_proof_aborts_with_bad_proof() {
        // Select Proof Bytes
        let mut proof_buffer = [42u8; MAX_PROOF_SIZE];
        let proof_size = PROOF.len();
        proof_buffer[..proof_size].clone_from_slice(PROOF);

        // Select Verifier Key Bytes
        let mut vk_buffer = [0u8; MAX_VERIFIER_KEY_SIZE];
        let vk_size = VERIFIER_KEY.len();
        vk_buffer[..vk_size].clone_from_slice(KZG_PARAMS);

        // Select KZG Params Bytes
        let mut kzg_params_buffer = [0u8; MAX_KZG_PARAMS_SIZE];
        let kzg_params_size = KZG_PARAMS.len();
        kzg_params_buffer[vk_size..kzg_params_size].clone_from_slice(KZG_PARAMS);

        //assert!(result)
	}
}
use std::{fs::File, io::{BufWriter, Write}};
use halo2_proofs::{
    circuit::{Layouter, SimpleFloorPlanner, Value},
    plonk::{
        create_proof, keygen_pk, keygen_vk_custom, Advice, Circuit, Column,
        ConstraintSystem, ErrorFront, Fixed, Instance
    },
    poly::{
        kzg::{
            commitment::{KZGCommitmentScheme, ParamsKZG},
            multiopen::ProverSHPLONK,
        },
        commitment::Params,
        Rotation,
    },
    transcript::{
        Blake2bWrite, Challenge255, TranscriptWriterBuffer
    },
    SerdeFormat,
};
use halo2curves::bn256::{Bn256, Fr, G1Affine};
use rand_core::OsRng;
use ff::{Field, PrimeField};

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

fn main() {
    let k = 4;
    let circuit = StandardPlonk(Fr::random(OsRng));
    let params = ParamsKZG::<Bn256>::setup(k, OsRng);
    let compress_selectors = true;
    let vk = keygen_vk_custom(&params, &circuit, compress_selectors).expect("vk should not fail");
    let cs = vk.clone().cs;
    let pk = keygen_pk(&params, vk.clone(), &circuit).expect("pk should not fail");

    let instances = vec![vec![circuit.0]];
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
        &[instances.clone()],
        OsRng,
        &mut transcript,
    )
    .expect("prover should not fail");
    let proof = transcript.finalize();

    //write proof
    std::fs::write("proof.bin", &proof[..])
    .expect("should succeed to write new proof");

    //write instances
    let f = File::create("pub_input.bin").unwrap();
    let mut writer = BufWriter::new(f);
    instances.to_vec().into_iter().flatten().for_each(|fp| { writer.write(&fp.to_repr()).unwrap(); });
    writer.flush().unwrap();

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
}
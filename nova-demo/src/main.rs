use std::{
    collections::HashMap,
    env::{self, current_dir},
    time::Instant,
};

// consider nova scotia as some middleware, that will make it easy for you to interact
// with nova.
use nova_scotia::{
    circom::reader::load_r1cs, create_public_params, create_recursive_circuit, FileLocation, F, S,
};
use nova_snark::{provider, CompressedSNARK, PublicParams};
use serde_json::json;

fn run_test(circuit_filepath: String, witness_gen_filepath: String) {
    /*
    1. Define the curve cycle that we want to use.
    We will use the bn256/grumpkin curve cycle.
    */
    type G1 = provider::bn256_grumpkin::bn256::Point;
    type G2 = provider::bn256_grumpkin::grumpkin::Point;

    /*
    2. Load the r1cs and witness generator files.
    */
    println!(
        "Running test with witness generator: {} and group: {}",
        witness_gen_filepath,
        std::any::type_name::<G1>()
    );
    let iteration_count = 5;
    let root = current_dir().unwrap();

    let circuit_file = root.join(circuit_filepath);
    let r1cs = load_r1cs::<G1, G2>(&FileLocation::PathBuf(circuit_file));
    let witness_generator_file = root.join(witness_gen_filepath);
    /*
    3. Setuping the private auxiliary inputs that we will
    use when folding. They are two public inputs at
    each folding steps (step_in[0], step_in[1]):
        step_out[0] <== step_in[0] + adder;
        step_out[1] <== step_in[0] + step_in[1];

    adder is the private input (auxiliary input) that we have.

    step_in[0], step_in[1], adder
        10,        10,        0
        10,        20,        1
        11,        30,        2
        13,        41,        3
        16,        54,        4
        20,        70,        5 <-- state of things when we output results
    */
    let mut private_inputs = Vec::new();
    for i in 0..iteration_count {
        let mut private_input = HashMap::new();
        private_input.insert("adder".to_string(), json!(i));
        private_inputs.push(private_input);
    }

    /*
    4. Set the starting public inputs that we are going to use
    */
    let start_public_input = [F::<G1>::from(10), F::<G1>::from(10)];

    /*
    5. Create the public  parameters for the recursive snark.
    */
    let pp: PublicParams<G1, G2, _, _> = create_public_params(r1cs.clone());

    /*
    6. We can print some info about the recursive snark that we are building
    */
    println!(
        "Number of constraints per step (primary circuit): {}",
        pp.num_constraints().0
    );
    println!(
        "Number of constraints per step (secondary circuit): {}",
        pp.num_constraints().1
    );

    println!(
        "Number of variables per step (primary circuit): {}",
        pp.num_variables().0
    );
    println!(
        "Number of variables per step (secondary circuit): {}",
        pp.num_variables().1
    );

    println!("Creating a RecursiveSNARK...");

    /*
    7. Create the recursive snark.
    */
    println!("Creating a RecursiveSNARK...");
    let start = Instant::now();
    let recursive_snark = create_recursive_circuit(
        FileLocation::PathBuf(witness_generator_file.clone()),
        r1cs.clone(),
        private_inputs,
        start_public_input.to_vec(),
        &pp,
    )
    .unwrap();
    println!("RecursiveSNARK creation took {:?}", start.elapsed());

    /*
    8. Verify it
    */
    // TODO: empty?
    let z0_secondary = [F::<G2>::from(0)];

    // verify the recursive SNARK
    println!("Verifying a RecursiveSNARK...");
    let start = Instant::now();
    let res = recursive_snark.verify(&pp, iteration_count, &start_public_input, &z0_secondary);
    println!(
        "RecursiveSNARK::verify: {:?}, took {:?}",
        res,
        start.elapsed()
    );
    assert!(res.is_ok());

    let z_last = res.unwrap().0;

    assert_eq!(z_last[0], F::<G1>::from(20));
    assert_eq!(z_last[1], F::<G1>::from(70));

    /*
    The proof is quite large... so we will compress it
    9. Generate a compressed snark using SPARTAN
    */
    println!("Generating a CompressedSNARK using Spartan with IPA-PC...");
    let start = Instant::now();
    let (pk, vk) = CompressedSNARK::<_, _, _, _, S<G1>, S<G2>>::setup(&pp).unwrap();
    let res = CompressedSNARK::<_, _, _, _, S<G1>, S<G2>>::prove(&pp, &pk, &recursive_snark);
    println!(
        "CompressedSNARK::prove: {:?}, took {:?}",
        res.is_ok(),
        start.elapsed()
    );
    assert!(res.is_ok());
    let compressed_snark = res.unwrap();

    /*
    10. Verify the compressed snark
    */
    // verify the compressed SNARK
    println!("Verifying a CompressedSNARK...");
    let start = Instant::now();
    let res = compressed_snark.verify(
        &vk,
        iteration_count,
        start_public_input.to_vec(),
        z0_secondary.to_vec(),
    );
    println!(
        "CompressedSNARK::verify: {:?}, took {:?}",
        res.is_ok(),
        start.elapsed()
    );
    assert!(res.is_ok());

    /*
    Ensure that you get the following output in your terminal
    RecursiveSNARK::verify: Ok(([
        0x0000000000000000000000000000000000000000000000000000000000000014,
        0x0000000000000000000000000000000000000000000000000000000000000046],
        [0x0000000000000000000000000000000000000000000000000000000000000000]
    ))
    */
}

fn main() {
    let circuit_filepath = "circuits/main.r1cs";
    let witness_gen_filepath = "circuits/main_js/main.wasm";

    run_test(
        circuit_filepath.to_string().clone(),
        witness_gen_filepath.to_string(),
    );
}

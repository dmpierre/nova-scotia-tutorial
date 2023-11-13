use std::{collections::HashMap, env::current_dir, time::Instant};

// consider nova scotia as some middleware, that will make it easy for you to interact
// with nova.
use nova_scotia::{
    circom::reader::load_r1cs, create_public_params, create_recursive_circuit, FileLocation, F, S,
};
use nova_snark::{provider, CompressedSNARK, PublicParams};
use serde_json::json;

fn run_test() {
    /*
    1. Define the curve cycle that we want to use.
    We will use the bn256/grumpkin curve cycle
    */

    /*
    2. Load the r1cs and witness generator files.
    */

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

    /*
    4. Set the public inputs that we are going to use
    */

    /*
    5. Create the public parameters for the recursive snark.
    */

    /*
    6. We can print some info about the recursive snark that we are building
    */

    /*
    7. Create the recursive snark.
    */

    /*
    8. Verify it
    */

    /*
    9. Generate a compressed snark using SPARTAN
    */

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
    let cur_dir = env::current_dir().unwrap();
    let circuit_filepath = "circuits/main.r1cs";
    let witness_gen_filepath = "circuits/main_js/main.wasm";

    run_test();
}

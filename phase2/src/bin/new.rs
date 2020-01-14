extern crate rand;
extern crate phase2;
extern crate exitcode;

use std::fs::File;
use phase2::parameters::MPCParameters;
use phase2::circom_circuit::CircomCircuit;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: \n<in_circuit.json> <out_params.params>");
        std::process::exit(exitcode::USAGE);
    }
    let circuit_filename = &args[1];
    let params_filename = &args[2];

    let should_filter_points_at_infinity = false;

    // Import the circuit and create the initial parameters using phase 1
    println!("Creating initial parameters for {}...", circuit_filename);
    let params = {
        let c = CircomCircuit {
            file_name: &circuit_filename,
            witness: "",
            has_witness: false,
        };
        MPCParameters::new(c, should_filter_points_at_infinity).unwrap()
    };

    println!("Writing initial parameters to {}.", params_filename);
    let mut f = File::create(params_filename).unwrap();
    params.write(&mut f).expect("unable to write params");
}

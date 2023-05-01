//TODO: Remove for prod
#![allow(
    unused_import_braces,
    dead_code,
    unused_imports,
    unused_variables,
    unreachable_code,
    non_snake_case
)]

use hcl::Body;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::HashMap;
use std::fs;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct ArbitraryData(HashMap<String, serde_yaml::Value>);

fn main() {
    //TODO: should we use if let here?
    if let Ok(file) = fs::File::open(String::from("terraform-output.yaml")) {
        // let input: ArbitraryData = serde_yaml::from_reader(file).unwrap();
        let input: serde_yaml::Value = serde_yaml::from_reader(file).unwrap();
        println!("{:?}", input);
        //TODO: See about a way to convert to HCL output
        // let output: Body = hcl::from_body(input).unwrap();
    };
    todo!("main function");
}

//TODO: References
// https://docs.rs/hcl-rs/latest/hcl/
// https://github.com/serde-rs/json/issues/144

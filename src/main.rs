//use serde_yaml;
//use yaml_macro_proc::yaml_struct;
//
//yaml_struct!("data.yaml");
//
//fn main() {
//    let data = GeneratedStruct {
//        name: String::from("John Doe"),
//        age: 30,
//        active: true,
//    };
//
//    println!("{:?}", data);
//}

use std::{env, fs};
use yaml_macro_proc::yaml_struct;

fn main() {
    yaml_struct!("data.yaml");
    let current_dir = env::current_dir().expect("Failed to get current directory");
    println!("Current directory: {:?}", current_dir);

    let file = fs::read_to_string("./data.yaml").expect("Unable to read file");
    println!("File content:\n{}", file);
}

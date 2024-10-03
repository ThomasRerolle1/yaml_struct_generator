use serde_yaml;
use std::fs;
use yaml_macro_proc::yaml_struct;

yaml_struct!("./data.yaml");

fn main() {
    let yaml_content = fs::read_to_string("./data.yaml").expect("Unable to read the YAML file");

    //let data1: GeneratedStruct = serde_yaml::from_str(&yaml_content).expect("Unable to parse YAML");
    //println!("data1: '{:?}'", data1);

    let yaml_data: GeneratedStruct =
        serde_yaml::from_str(&yaml_content).expect("Unable to parse YAML");
    println!("Yaml value : {:?}", yaml_data);
}

//use std::{env, fs};
//use yaml_macro_proc::yaml_struct;
//
//fn main() {
//    yaml_struct!("./data.yaml");
//    let current_dir = env::current_dir().expect("Failed to get current directory");
//    println!("Current directory: {:?}", current_dir);
//
//    let file = fs::read_to_string("./data.yaml").expect("Unable to read file");
//    println!("File content:\n{}", file);
//}

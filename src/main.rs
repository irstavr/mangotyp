use clap::{Arg, Command};
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

fn main() {
     let matches = Command::new("magotyp")
        .version("1.0.0")
        .author("Irini St")
        .about("From Rust types to Typescript types")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .required(true)
                .help("The input Rust file"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .required(true)
                .help("The output Typescript file"),
        )
        .get_matches();

    let input_filename = matches
        .get_one::<String>("input")
        .expect("input required");
        
    let output_filename = matches
        .get_one::<String>("output")
        .expect("output required");

    dbg!(input_filename);
    dbg!(output_filename);

    let input_path = Path::new(input_filename);

    let mut input_file =
        File::open(input_path).
            expect(&format!("Cannot open file {}", input_path.display()));

    let mut input_file_text = String::new();

    input_file
        .read_to_string(&mut input_file_text)
        .expect("Unable to read file");

    // This is our tokenized version of Rust file ready to process
    let input_syntax: syn::File = 
        syn::parse_file(&input_file_text).expect("Cannot parse file!");

}

pub mod types;
pub mod enums;
pub mod structs;


use clap::{Arg, Command};
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

fn main() {
     let matches = Command::new("mangotyp")
        .about("From Rust types to Typescript types")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .required(true)
                .help("the Rust input file"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .required(true)
                .help("the Typescript output file"),
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
        .expect("Cannot read file");

    // This is our tokenized version of Rust file ready to process
    let input_syntax: syn::File = 
        syn::parse_file(&input_file_text).expect("Cannot parse file!");

    // stores the output of the Typescript file 
    // we will continuously append to as we process the Rust file
    let mut output_text = String::new();

    for item in input_syntax.items.iter() {
        match item {
            // This `Item::Type` enum variant matches our type alias
            syn::Item::Type(item_type) => {
                let type_text = types::parse_token_type(item_type);
                output_text.push_str(&type_text);
            }
            syn::Item::Enum(item_enum) => {
                let enum_text = enums::parse_enum(item_enum);
                output_text.push_str(&enum_text);
            }
            syn::Item::Struct(item_struct) => {
                let struct_text = structs::parse_struct(item_struct);
                output_text.push_str(&struct_text);
            }
            _ => {
                dbg!("Unimplemented type!");
            }
        }
    }

    let mut output_file = File::create(output_filename).unwrap();

    write!(output_file, "{}", output_text).expect("Cannot write to output file");
}


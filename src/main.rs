pub mod types;
pub mod enums;
pub mod structs;
pub mod std_types;


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

    let input_syntax: syn::File = read_input_file(input_filename.to_string());

    write_output_file(input_syntax, output_filename.to_string());
}


/// It reads the file into a string, and parses it into a Rust syntax tree
/// 
/// Arguments:
/// 
/// * `input_filename`: String - This is the name of the file we want to read.
fn read_input_file(input_filename: String) -> syn::File {

    let input_path = Path::new(&input_filename);

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

    input_syntax
}


/// We write to `output_filename` the standard library types of Rust as we translate them into Typescript, 
/// and then we write to it the result of the parsed Rust file `input_syntax` to TypeScript.
/// 
/// Arguments:
/// 
/// * `input_syntax`: This is the parsed Rust file that we will be converting to Typescript.
/// * `output_filename`: The name of the file we want to write to.
fn write_output_file(input_syntax: syn::File, output_filename: String) {
    // stores the output of the Typescript file 
    // we will continuously append to as we process the Rust file
    let mut output_text = String::new();

    output_text.push_str(&std_types::translate_std_types());

    output_text.push_str(&parse_input_file(input_syntax));

    let mut output_file = File::create(output_filename).unwrap();

    write!(output_file, "{}", output_text).expect("Cannot write to output file");
}


/// Iterates over the items in the file, and match on the type of item to Typescript equivalent type.
/// 
/// Arguments:
/// * `file`: syn::File
/// 
/// Returns:  
/// A string of text with the TypeScript type.
fn parse_input_file(file: syn::File) -> String {
    let mut output_text = String::new();

    for item in file.items.iter() {
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
    output_text
}


/// 
/// 
/// 
/// 
/// 
/// TESTS 
/// 
/// 
/// 
/// 
/// 

#[cfg(test)]
mod tests {
    // Import the necessary modules
    use std::{
        fs::File,
        io::{Read},
    };
    use super::*;

    
    #[test]
    fn test_types() {
        let mut input_file = File::open("./tests/cases/type_test.rs").unwrap();

        let mut input_file_text = String::new();

        input_file.read_to_string(&mut input_file_text).unwrap();

        let input_syntax: syn::File =
            syn::parse_file(&input_file_text).expect("Cannot parse file");

        let typescript_types = parse_input_file(input_syntax);

        assert_eq!(r#"export type Integer32 = number;"#, &typescript_types);
    }

    #[test]
    fn test_struct() {
        let mut input_file = File::open("./tests/cases/struct_test.rs").unwrap();
        let mut input_file_text = String::new();

        input_file.read_to_string(&mut input_file_text).unwrap();

        let input_syntax: syn::File =
            syn::parse_file(&input_file_text).expect("Cannot parse file");

        let typescript_types = parse_input_file(input_syntax);

        assert_eq!(
            r#"export interface Person {name:string;age:number;has_gut_issues:boolean;};"#,
            &typescript_types
        );
    }

    #[test]
    fn test_enum() {
        let mut input_file = File::open("./tests/cases/enum_test.rs").unwrap();

        let mut input_file_text = String::new();

        input_file.read_to_string(&mut input_file_text).unwrap();

        let input_syntax: syn::File =
            syn::parse_file(&input_file_text).expect("Unable to parse file");

        let typescript_types = parse_input_file(input_syntax);

        assert_eq!(
            r#"export type HealthStatus =  | { test: "Protein" , result: number} | { test: "Triglycerid" , result: number} | { test: "Fats" , result: number};"#,
            &typescript_types
        );
    }
}
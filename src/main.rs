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
                let type_text = parse_item_type(item_type);
                output_text.push_str(&type_text);
            }
            _ => {
                dbg!("Unimplemented type!");
            }
        }
    }

    let mut output_file = File::create(output_filename).unwrap();

    write!(output_file, "{}", output_text).expect("Cannot write to output file");
}

/// 
/// Converts a Rust item type to a Typescript one
///
/// ## Example:
/// **Input:** type Integer32 = i32;
/// **Output:** export type Integer32 = number;
fn parse_item_type(item_type: &syn::ItemType) -> String {
    let mut output_text = String::new();

    output_text.push_str("export type ");

    // `ident` is the name of the type alias, e.g. `Integer32`
    output_text.push_str(&item_type.ident.to_string());
    output_text.push_str(" = ");

    let type_string = parse_type(&item_type.ty);
    output_text.push_str(&type_string);
    output_text.push_str(";");

    output_text
}

/// Converts a Rust type into a Typescript type
///
/// ## Example:
/// **Input:**  (i32, i32) / Option<String>
/// **Output:** \[number, number\] / Option<string>;
fn parse_type(syn_type: &syn::Type) -> String {
    let mut output_text = String::new();

    match syn_type {
        // Primitive types like i32 will match Path
        // We currently do not do anything with full paths
        // so we take only the last() segment (the type name)
        syn::Type::Path(type_path) => {
            let segment = type_path.path.segments.last().unwrap();

            let field_type = segment.ident.to_string();

            let ts_field_type = parse_type_ident(&field_type).to_owned();
            output_text.push_str(&ts_field_type);

            match &segment.arguments {
                // For simple types with no arguments, e.g. i32 
                syn::PathArguments::None => {}
                _ => {
                    dbg!("Unimplemented token!");
                }
            }
        }
        _ => {
            dbg!("Unimplemented token!");
        }
    }

    output_text
}


/// 
/// Convert a primitive Rust ident to an equivalent Typescript type name
/// Translate primitive types to Typescript equivalent otherwise
/// returns the ident untouched
///
/// ## Example:
/// **Input:** i32 / Option / bool;
/// **Output:** number / Option / boolean;
fn parse_type_ident(ident: &str) -> &str {
    match ident {
        // All of Rust's many different types of numbers will simply be treated as a number when deserialized in TS;)
        "i8" | "i16" | "i32" | "i64" | "i128" | "u8" | "u16" | "u32" 
            | "u64" | "f32" | "f64" | "isize" | "usize" => "number",
        "String" | "str" | "char" => "string",
        "bool" => "boolean",
        _ => ident,
    }
}

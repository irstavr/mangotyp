/// 
/// It takes a Rust type alias and returns a TypeScript type alias
/// 
/// ## Example:
/// Input:  type Integer32 = i32;
/// Output: export type Integer32 = number;
/// 
/// Arguments:
/// * `item_type`: the Rust token to parse
/// 
/// Returns:
/// A string of the type alias in TypeScript
pub fn parse_token_type(item_type: &syn::ItemType) -> String {
    let mut output_text = String::new();

    output_text.push_str("export type ");

    // `ident` is the name of the type alias, e.g. `Integer32`
    let ident = &item_type.ident;
    output_text.push_str(&ident.to_string());
    output_text.push_str(" = ");

    let type_string = parse_type(&item_type.ty);
    output_text.push_str(&type_string);
    output_text.push_str(";");

    output_text
}

/// It takes a Rust type and returns a Typescript type
///
/// if it's a path, takes the last segment and parse it as an identifier. 
/// If it's a tuple, parses each element as a type.
/// 
/// ## Example:
/// Input:  (i32, i32) / Option<String>
/// Output: \[number, number\] / Option<string>;
/// 
/// Arguments:
/// * `syn_type`: The type to parse
/// 
/// Returns:
/// A string representation of the TypeScript type of the field
pub fn parse_type(syn_type: &syn::Type) -> String {
    let mut output_text = String::new();

    match syn_type {
        // Primitive types like i32 will match Path
        // We currently do not do anything with full paths (for example `std:fs:File`)
        // so we take only the last() segment (the type name)
        syn::Type::Path(type_path) => {
            let segment = type_path.path.segments.last().unwrap();

            let field_type = segment.ident.to_string();

            let ts_field_type = parse_ident(&field_type).to_owned();
            output_text.push_str(&ts_field_type);

            match &segment.arguments {
                // For simple types with no arguments, e.g. i32 
                syn::PathArguments::None => {}

                // e.g: HashMap<String, MyObject>
                syn::PathArguments::AngleBracketed(angle_bracket_args) => {
                    output_text.push_str("<");
                    let args = angle_bracket_args.args.iter();
                    for arg in args {
                        match arg {
                            syn::GenericArgument::Type(inner_type) => {
                                output_text.push_str(&parse_type(inner_type));
                                output_text.push_str(",");
                            }
                            
                            _ => {
                                dbg!("Unimplemented token");
                            }
                        }
                    }
                    output_text.push_str(">");
                }
                
                _ => {
                    dbg!("Unimplemented token!");
                }
            }
        }

        // For Tuple types, e.g. [i32, i32]
        syn::Type::Tuple(type_tuple) => {
            output_text.push_str("[");
            for elem in type_tuple.elems.iter() {
                output_text.push_str(&parse_type(elem));
                output_text.push_str(",");
            }
            output_text.push_str("]");
        }
        
        _ => {
            dbg!("Unimplemented token!");
        }
    }

    output_text
}


/// 
/// It takes a Primitive Rust ident and returns its equivalent TypeScript type name in a string.
/// ## Example:
/// Input:  i32 / Option / bool;
/// Output: number / Option / boolean;
/// 
/// Arguments:
/// * `ident`: The name of the type we're parsing.
/// 
/// Returns: A string slice of the result TypeScript representation
fn parse_ident(ident: &str) -> &str {
    match ident {
        // All of Rust's many different types of numbers will simply be treated as a number when deserialized in TS;)
        "i8" | "i16" | "i32" | "i64" | "i128" | "u8" | "u16" | "u32" 
            | "u64" | "f32" | "f64" | "isize" | "usize" => "number",
        "String" | "str" | "char" => "string",
        "bool" => "boolean",
        _ => ident,
    }
}

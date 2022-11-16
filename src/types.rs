/// 
/// Converts a Rust item type to a Typescript one
///
/// ## Example:
/// **Input:** type Integer32 = i32;
/// **Output:** export type Integer32 = number;
pub fn parse_item_type(item_type: &syn::ItemType) -> String {
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
pub fn parse_type(syn_type: &syn::Type) -> String {
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
/// Convert a Primitive Rust ident to an equivalent TypeScript type name
/// Translate Primitive types to TypeScript equivalent otherwise
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

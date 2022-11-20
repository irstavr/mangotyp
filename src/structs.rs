/// 
/// It takes a Rust struct and returns a TypeScript interface
///
/// ## Example:
///
/// Input:
/// struct Person {
///     name: String,
///     age: u32,
///     has_gut_issues: bool,
/// }
///
/// Output:
/// export interface Person {
///     name: string;
///     age: number;
///     has_gut_issues: boolean;
/// }
/// 
/// Arguments:
/// 
/// * `item_struct`: &syn::ItemStruct
/// 
/// Returns:
/// 
/// A string of text that is the interface definition
pub fn parse_struct(item_struct: &syn::ItemStruct) -> String {
    let mut output_text = String::new();

    let struct_name = item_struct.ident.to_string();

    output_text.push_str("export interface");
    output_text.push_str(" ");
    output_text.push_str(&struct_name);
    output_text.push_str(" ");
    output_text.push_str("{");

    match &item_struct.fields {
        syn::Fields::Named(named_fields) => {
            for named_field in named_fields.named.iter() {
                match &named_field.ident {
                    Some(ident) => {
                        let field_name = ident.to_string();
                        output_text.push_str(&field_name);
                        output_text.push_str(":");
                    }
                    None => {
                        dbg!("Unimplemented token");
                    }
                }
                let field_type = super::types::parse_type(&named_field.ty);
                output_text.push_str(&field_type);
                output_text.push_str(";");
            }
        }

        // the tuple structs are serialized as interfaces
        // to align with `serde's default handling`
        // the fields named for the numerical index 
        syn::Fields::Unnamed(fields) => {
            // e.g.: struct BestStruct (i32, Anything);
            // output: export interface BestStruct { 0: i32, 1: Anything }
            for (index, field) in fields.unnamed.iter().enumerate() {
                output_text.push_str(&index.to_string());
                output_text.push_str(":");
                output_text.push_str(&super::types::parse_type(&field.ty));
                output_text.push_str(";");
            }
        }
        syn::Fields::Unit => (),
    }

    output_text.push_str("}");
    output_text.push_str(";");

    output_text
}

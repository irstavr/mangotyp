/// 
/// It takes a Rust enum and returns an equivalent Typescript type.
/// 
/// We iterate over each variant in the enum, 
/// and for each variant creates a union type with the
/// variant name as a string literal and the variant's fields as a type
/// 
/// ## Examples
///
/// Input:
/// enum HealthStatus {
///     Protein(i32),
///     Triglycerids(i32),
///     Fats(i32),
/// }
///
/// Output:
/// export type HealthStatus =
///   | { test: "Protein"; result: number }
///   | { test: "Triglycerids"; result: number }
///   | { test: "Fats"; result: number };
/// 
/// Arguments:
/// * `item_enum`: The enum we're parsing
/// 
/// Returns: 
/// A string of text that is the TypeScript representation of the enum
pub fn parse_enum(item_enum: &syn::ItemEnum) -> String {
    let mut output_text = String::new();

    output_text.push_str("export type");
    output_text.push_str(" ");

    let enum_name = item_enum.ident.to_string();
    output_text.push_str(&enum_name);
    output_text.push_str(" ");
    output_text.push_str("=");
    output_text.push_str(" ");

    for variant in item_enum.variants.iter() {
        // Use the pipe character for union types
        // TS also allows it before the first type as valid syntax
        // See: https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#union-types
        output_text.push_str(" | {");
        output_text.push_str(" ");

        // We make the assumption that enums will be using serde's "Adjacently Tagged" attribute
        // #[serde(tag = "test", content = "result")]
        // See: https://serde.rs/enum-representations.html#adjacently-tagged
        output_text.push_str("test: \"");
        let variant_name = variant.ident.to_string();
        output_text.push_str(&variant_name);
        output_text.push_str("\" , result: ");

        match &variant.fields {
            syn::Fields::Named(named_fields) => {
                output_text.push_str("{");
                for field in named_fields.named.iter() {
                    if let Some(ident) = &field.ident {
                        output_text.push_str(&ident.to_string());
                        output_text.push_str(":");

                        let field_type = super::types::parse_type(&field.ty);
                        output_text.push_str(&field_type);
                        output_text.push_str(";");
                    }
                }
                output_text.push_str("}");
            }

            syn::Fields::Unnamed(unnamed_fields) => {
                // Currently only support a single unnamed field: e.g the i32 in Protein(i32)
                let unnamed_field = unnamed_fields.unnamed.first().unwrap();
                let field_type = super::types::parse_type(&unnamed_field.ty);
                output_text.push_str(&field_type);
            }

            syn::Fields::Unit => {
                output_text.push_str("undefined");
            }
        }

        output_text.push_str("}");
    }
    output_text.push_str(";");

    output_text
}

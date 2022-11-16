/// It creates a string of TypeScript code that defines the standard library types that Rust uses
/// 
/// `HashSet<T, string>` can be deserialized as an object in the TS form of `Record<T, undefined>`,
/// `HashMap<T, U>` can be deserialized as an object in the TS form of `Record<T, U>`, 
/// `Vec<T>` can be thought in TS of as `Array<T, undefined>`, 
/// `Option<T>` can be thought of as `T | undefined`,
/// `Result<T, U>` can be thought of as `T | U`
/// 
/// Returns: 
/// A String representing the typescript translation 
pub fn translate_std_types() -> String {
    let mut output_text = String::new();

    output_text
        .push_str("type HashSet<T extends number | string> = Record<T, undefined>;");
    output_text.push_str("type HashMap<T extends number | string, U> = Record<T, U>;");
    output_text.push_str("type Vec<T> = Array<T>;");
    output_text.push_str("type Option<T> = T | undefined;");
    output_text.push_str("type Result<T, U> = T | U;");
    // ...to add more here...

    output_text
}

/// 
/// support for some of Rust's most common types in the standard library like Option, and HashMap
/// and translate them to equivalent types in TypeScript
/// 
pub fn create_std_types() -> String {
    let mut output_text = String::new();

    // HashSet<T, string> can be deserialized as an object in the TS form of Record<T, undefined>
    output_text
        .push_str("type HashSet<T extends number | string> = Record<T, undefined>;");
    // HashMap<T, U> can be deserialized as an object in the TS form of `Record<T, U>`
    output_text.push_str("type HashMap<T extends number | string, U> = Record<T, U>;");
    // Vec<T> can be thought in TS of as `Array<T, undefined>`
    output_text.push_str("type Vec<T> = Array<T>;");
    // Option<T> can be thought of as `T | undefined`
    output_text.push_str("type Option<T> = T | undefined;");
    // Result<T, U> can be thought of as `T | U`
    output_text.push_str("type Result<T, U> = T | U;");
    // ...to add more here...

    output_text
}

pub fn runtime_concat_string(a: &str, b: &str) -> String {
    let mut value = a.to_string();
    value.push_str(b);
    value
}

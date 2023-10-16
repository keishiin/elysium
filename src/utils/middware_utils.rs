pub fn split_by_double_quotes(input: &str) -> Option<&str> {
    let start = input.find('"')? + 1;
    let end = input.rfind('"')?;
    Some(&input[start..end])
}
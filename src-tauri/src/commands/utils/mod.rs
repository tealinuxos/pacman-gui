pub fn split_by(input: String, pattern: &str) -> Vec<String>
{
    let vec: Vec<String> = input
        .split(pattern)
        .map(|i| String::from(i))
        .collect();

    vec
}

pub fn split_and_filter(input: String, pattern: &str, filter: &str) -> Vec<String>
{
    let vec: Vec<String> = input
        .split(pattern)
        .map(|i| String::from(i))
        .filter(|i| i != filter)
        .collect();

    vec
}

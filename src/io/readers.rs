use std::str::FromStr;

#[allow(unused_assignments)]
pub fn parse_vec<T>(s: &str) -> Option<Vec<T>>
    where T: FromStr
{
    if s.is_empty() {
        return None;
    }
    let mut cleaned_first_token = String::new();
    let mut cleaned_last_token = String::new();
    let mut tokens = s.split(", ").collect::<Vec<&str>>();
    let last_token_index = tokens.len() - 1;
    if tokens[0].is_empty() || tokens[last_token_index].is_empty() {
        return None;
    }
    cleaned_first_token = String::from(tokens[0].clone());
    if cleaned_first_token.remove(0) != '[' {
        return None;
    }
    tokens[0] = &cleaned_first_token;
    cleaned_last_token = String::from(tokens[last_token_index].clone());
    if cleaned_last_token.pop().unwrap() != ']' {
        return None;
    }
    tokens[last_token_index] = &cleaned_last_token;
    let mut xs = Vec::new();
    let mut buffer = String::new();
    for token in &tokens {
        if !buffer.is_empty() {
            buffer.push_str(", ");
        }
        buffer.push_str(token);
        if let Ok(x) = buffer.parse() {
            xs.push(x);
            buffer.clear();
        }
    }
    if buffer.is_empty() { Some(xs) } else { None }
}

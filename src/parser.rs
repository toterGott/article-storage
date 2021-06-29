use regex::Regex;

pub fn is_link(message: &str) -> bool {
    let re = Regex::new(r"(http|https)://\S*").unwrap();
    re.is_match(message)
}
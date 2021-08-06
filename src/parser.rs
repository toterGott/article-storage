use std::iter::Enumerate;
use std::str::Chars;

use regex::Regex;

// TODO ignore text in message if link is present
pub fn is_link(message: &str) -> bool {
    let re = Regex::new(r"(http|https)://\S*").unwrap();
    re.is_match(message)
}

// possible solution
// let re = Regex::new("href=\"(.*?)\"").unwrap();

// TODO optimize
pub async fn parse_links(html: &str) -> Vec<String> {
    let mut links: Vec<String> = Vec::new();
    log::info!("Parsing...");
    let html_chars: Vec<char> = html.chars().collect();
    for index in 0..html_chars.len() - 1 {
        if is_href(index, &html_chars) {
            let link = parse_link(index, &html_chars);
            println!("Link: {}", link);
            links.push(link);
        }
    }
    links
}

fn parse_link(index: usize, html_chars: &Vec<char>) -> String {
    let link_start = index + 6;
    let mut link_chars: Vec<char> = Vec::new();
    for i in link_start..html_chars.len() - 1 {
        if html_chars[i] == '"' { break; }
        link_chars.push(html_chars[i])
    }
    let link: String = link_chars.iter().collect();
    link
}

fn is_href(index: usize, html_chars: &Vec<char>) -> bool {
    return html_chars[index] == 'h' &&
        html_chars[index + 1] == 'r' &&
        html_chars[index + 2] == 'e' &&
        html_chars[index + 3] == 'f' &&
        html_chars[index + 4] == '=' &&
        html_chars[index + 5] == '"';
}

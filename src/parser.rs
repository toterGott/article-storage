use regex::Regex;

pub fn is_link(message: &str) -> bool {
    let re = Regex::new(r"(http|https)://\S*").unwrap();
    re.is_match(message)
}

pub fn parse_links(html: &str) -> Vec<&str> {
    // TODO find out why correct pattern isn't working
    let re = Regex::new("href=\"(.*?)\"").unwrap();
    let mut v: Vec<&str> = Vec::new();
    log::info!("Parsing...");
    for link in re.find_iter(html) {
        log::info!("Link: {:?}", link);
        v.push(link.as_str());
    };
    v
}

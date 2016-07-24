use regex::Regex;

pub struct Post {
    pub author: String,
    pub title: String,
    pub link: String,
    pub description: String,
    pub guid: String,
}

fn xml_parse(data: String, tag: &str) -> String {
    let pattern = format!(r"(<{}>)(.+?)(</{}>)", tag, tag);
    let re = match Regex::new(&pattern) {
        Ok(re) => re,
        Err(err) => panic!("{}", err),
    };

    match re.captures(&data) {
        Some(cap) => cap.at(2).unwrap_or("").to_string(),
        _ => "".to_string()
    }
}

pub fn rss_parse(data: &Vec<u8>) -> Vec<Post> {
    let content = String::from_utf8_lossy(data).into_owned();

    let mut posts : Vec<Post> = vec![];

    let re = Regex::new(r"<item>[\s\S]*?</item>").unwrap();

    for cap in re.captures_iter(&content) {
        let item = match cap.at(0) {
            Some(x) => x,
            _ => ""
        };
        if item == "" {
            continue;
        }

        let next_post = Post{
            author: xml_parse(item.to_string(), "author"),
            title: xml_parse(item.to_string(), "title"),
            link: xml_parse(item.to_string(), "link"),
            description: xml_parse(item.to_string(), "description"),
            guid: xml_parse(item.to_string(), "guid"),
        };

        posts.push(next_post);
    }
    posts
}

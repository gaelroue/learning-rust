pub struct Post {
    pub author: String,
    pub title: String,
    pub link: String,
    pub description: String,
    pub guid: String,
}

pub fn rss_parse(data: &Vec<u8>) {
    let content = String::from_utf8_lossy(data);
    println!("{}", content);
}

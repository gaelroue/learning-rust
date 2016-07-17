pub fn rss_parse(data: &Vec<u8>) {
    let content = String::from_utf8_lossy(data);
    println!("{}", content);
}

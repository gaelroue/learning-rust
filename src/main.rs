extern crate curl;
extern crate regex;

use curl::easy::Easy;
use std::path::Path;

mod rss;
mod ini;

fn main() {
    let path = Path::new("feed.ini");

    let feed = ini::Section::new("Feed", vec!["name", "url"]);

    let rss_list = feed.load_conf(&path);

    for (rss, url) in &rss_list {
        let mut handle = Easy::new();
        let mut data = Vec::new();

        println!("\n-------------------- {} -----------------\n", rss);
        handle.url(url).unwrap();
        {
            let mut transfer = handle.transfer();
            transfer.write_function(|buf| {
                data.extend_from_slice(buf);
                Ok(buf.len())
            }).unwrap();
            transfer.perform().unwrap();
        }

        let posts = rss::rss_parse(&data);

        for post in posts.iter() {
            println!("{} -> {}", post.title, post.link);
        }
    }

}

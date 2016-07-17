//extern crate curl;

//use curl::easy::Easy;
use std::path::Path;

//mod rss;
mod ini;

fn main() {
    let path = Path::new("toto.txt");

    let feed = ini::Section::new("Feed", vec!["name", "url"]);

    let rss_list = feed.load_conf(&path);
    for (rss, url) in &rss_list {
        println!("{} : {}", rss, url);
    }

    //let mut handle = Easy::new();
    //let mut data = Vec::new();
    //handle.url("https://news.ycombinator.com/rss").unwrap();
    //{
    //    let mut transfer = handle.transfer();
    //    transfer.write_function(|buf| {
    //        data.extend_from_slice(buf);
    //        Ok(buf.len())
    //    }).unwrap();
    //    transfer.perform().unwrap();
    //}
    //rss::rss_parse(&data);
}

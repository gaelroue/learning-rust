//extern crate curl;

//use curl::easy::Easy;
use std::path::Path;

//mod rss;
mod ini;

fn main() {
    let rss_test = "
<?xml version=\"1.0\" encoding=\"UTF-8\" ?>
<rss version=\"2.0\">
    <channel>
        <title>RSS Title</title>
        <description>This is an example of an RSS feed</description>
        <link>http://www.example.com/main.html</link>
        <lastBuildDate>Mon, 06 Sep 2010 00:01:00 +0000 </lastBuildDate>
        <pubDate>Sun, 06 Sep 2009 16:20:00 +0000</pubDate>
        <ttl>1800</ttl>

        <item>
            <title>Example entry</title>
            <description>Here is some text containing an interesting description.</description>
            <link>http://www.example.com/blog/post/1</link>
            <guid isPermaLink=\"true\">7bd204c6-1655-4c27-aeee-53f933c5395f</guid>
            <pubDate>Sun, 06 Sep 2009 16:20:00 +0000</pubDate>
        </item>
    </channel>
</rss>
";

    let path = Path::new("feed.ini");

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

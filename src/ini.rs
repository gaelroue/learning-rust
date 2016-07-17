use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;
use std::collections::HashMap;

pub struct Section <'a>{
    pub name: &'a str,
    pub keys: Vec<&'a str>,
}

impl<'a> Section<'a> {
    pub fn new(name: &'a str, keys: Vec<&'a str>) -> Section <'a> {
        Section {
            name: name,
            keys: keys,
        }
    }

    pub fn find_section(&self, buf: &str) -> bool {
        let pattern = format!("[{}]", self.name);

        buf.starts_with(&pattern)
    }

    pub fn get_key(&self, buf: &str) -> (usize, usize) {
        for (i, key) in self.keys.iter().enumerate() {
            if buf.starts_with(key) == true {
                let index = match buf.find('=') {
                   Some(x) => x,
                   None => 0
                };
                
                if index > 0 {
                    return (i, index);
                }

            }
        }
        (0, 0)
    }

    pub fn load_conf(&self, path: &Path) -> HashMap<String, String> {
        // Open file, if doesn t exist, create it
        let mut file =  OpenOptions::new().read(true).write(true).create(true).open(path).unwrap();

        let mut buf = String::new();
        file.read_to_string(&mut buf).expect("Error while reading configuration file");

        let mut lines = buf.lines(); 
        let mut line = lines.next();

        let mut rss_list = HashMap::new();
        // iter over file
        while line.is_some() {
            // if a comment is find, trimmed to remove whitespace on the left
             if line.unwrap().trim_left().starts_with('#') == true {
                 line = lines.next();
                 continue;
             }
             // iter until section is found
             if self.find_section(line.unwrap()) == false {
                 line = lines.next();
                 continue;
             }
             // get next line after section line
             line = lines.next();

             let mut name = "";
             let mut url = "";

             // iter over section
             while line.is_some() {
                 // if we arrive at next section stop
                 if line.unwrap().trim_left().starts_with('[') == true {
                     break;
                 }
                 let (k, index) = self.get_key(line.unwrap());
                 //if index equals 0, it means key=value not found
                 if index == 0 {
                     line = lines.next();
                     continue;
                 }
                 if k == 0 {
                    name = line.unwrap()[(index + 1)..].trim();
                 } else {
                    url = line.unwrap()[(index + 1)..].trim();
                 }

                 if name != "" && url != "" {
                     rss_list.insert(name.to_owned(), url.to_owned());
                 }
                 line = lines.next();
             }
        }
        rss_list
    }
}

use lazy_regex::lazy_regex;
use std::{fs::OpenOptions, io::{BufWriter, Read, Write}};

fn main() {
    let code = read();
    let mut out = output();
    let regex =
        lazy_regex!(r#"aria-label="([a-zA-ZА-Яа-яіІ0-9 \-\"\'\(\)\:]+) Custom Field. (.+)""#);
    for cap in regex.captures_iter(&code) {
        let cap1 = if cap[1].starts_with("Set value for ") {
            &cap[1][14..]
        } else {
            &cap[1]
        };

        let cap2 = cap.get(2).unwrap().as_str();
        let cap2 = cap2.strip_suffix(" aria-expanded=\"false").unwrap_or(cap2);

        // Fix CSV '"' escape
        let cap2 = cap2.replace("\"", "\"\"");

        writeln!(out, "\"{cap1}\",\"{cap2}\"").unwrap();
    }
}

pub fn read() -> String {
    let mut file = std::fs::File::open("code.html").unwrap();
    let mut buf = String::with_capacity(file.metadata().unwrap().len() as usize);
    file.read_to_string(&mut buf).unwrap();
    buf
}

pub fn output() -> BufWriter<std::fs::File> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("output.csv")
        .unwrap();
    BufWriter::new(file)
}

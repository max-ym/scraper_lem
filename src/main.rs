use lazy_regex::lazy_regex;
use std::io::Read;

fn main() {
    let code = read();
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

        println!("\"{cap1}\",\"{cap2}\"");
    }
}

pub fn read() -> String {
    let mut file = std::fs::File::open("code.html").unwrap();
    let mut buf = String::with_capacity(file.metadata().unwrap().len() as usize);
    file.read_to_string(&mut buf).unwrap();
    buf
}

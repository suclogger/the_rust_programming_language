use std::{env, fs};
use std::io::Read;

fn main() {
    let mut file = std::fs::File::open("./Master the Coding Interview Big Tech (FAANG) Interviews.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut line_num = 1 ;
    for line in contents.split("\n") {
        println!("{} - {}", line_num, line);
        fs::rename(format!("./lesson{}.mp4", line_num), format!("./lesson{}-{}.mp4", line_num, line));
        line_num += 1;
    }
}

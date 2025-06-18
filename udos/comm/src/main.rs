#![forbid(unsafe_code)]

use std::{fs::File, io::BufRead, io::BufReader};
use std::collections::HashSet;
use std::io;

fn read_lines(path: &str) -> io::Result<HashSet<String>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader.lines().collect()
}

fn main() {

    let args = std::env::args().collect::<Vec<String>>();

    let path1 = &args[1];
    let path2 = &args[2];

    let lines_file1 = read_lines(path1).unwrap();
    let lines_file2 = read_lines(path2).unwrap();
    
    for line_file1 in &lines_file1 {
        for line_file2 in &lines_file2 {
            if *line_file2 == *line_file1 {
                println!("{}", *line_file1);
            }
        }
    }

}

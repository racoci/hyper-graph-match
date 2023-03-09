use std::io::{self, BufRead};
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    let mut count = 0;
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line == "-" {
            break;
        }
        for word in line.split_whitespace() {
            map.entry(word.to_owned())
                .or_insert_with(|| {
                    let id = count;
                    count += 1;
                    id
                });
        }
    }

    println!("{:#?}", map);
}
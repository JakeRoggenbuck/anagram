use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let mut words = HashMap::<String, Vec<String>>::new();
    let file = File::open("../wordlist.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let ol = line?;

        let h = ol.chars().sorted().collect::<String>();
        let k = words.entry(h).or_insert(Vec::<String>::new());
        k.push(ol);
    }

    let mut keyvec: Vec<String> = words
        .iter()
        .filter(|v| v.1.len() > 1)
        .map(|x| x.0)
        .cloned()
        .collect();

    keyvec.sort_by(|a, b| {
        if a.len() > b.len() {
            Ordering::Less
        } else if a.len() == b.len() {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });

    for x in keyvec.iter().take(200) {
        println!("{:?}", words[x]);
    }

    Ok(())
}

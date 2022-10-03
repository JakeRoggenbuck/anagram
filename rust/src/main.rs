use itertools::Itertools;
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

    Ok(())
}

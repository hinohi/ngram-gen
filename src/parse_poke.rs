use std::collections::HashSet;

use serde::Deserialize;

#[derive(Deserialize)]
struct Poke {
    alt: String,
}

pub fn parse() -> HashSet<String> {
    let mut rdr = csv::Reader::from_path("poke.csv").unwrap();
    rdr.deserialize()
        .map(|r| {
            let r: Poke = r.unwrap();
            r.alt
        })
        .collect()
}

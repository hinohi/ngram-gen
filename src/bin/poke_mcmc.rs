use ngram_gen::*;

fn main() {
    let poke = parse_poke::parse();
    let bigram = make_char_bigram_dist(poke.iter());
    for (k1, v) in bigram.iter() {
        for (k2, v) in v.iter() {
            println!("{:?} {:?} {}", k1, k2, v);
        }
    }
}

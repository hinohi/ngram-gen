use rand_pcg::Mcg128Xsl64;

use ngram_gen::*;

fn main() {
    let mut rng = Mcg128Xsl64::new(1);

    let poke = parse_poke::parse();
    let bigram = make_char_bigram_dist(poke.iter());
    loop {
        let mut s = String::new();
        let mut state = Char::Start;
        loop {
            let next_state = bigram.get(&state).unwrap().sample(&mut rng);
            match next_state {
                Char::Char(c) => s.push(*c),
                Char::End => break,
                Char::Start => unreachable!(),
            }
            state = *next_state;
        }
        println!("{}", s);
    }
}

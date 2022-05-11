pub mod parse_poke;

use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Char {
    Start,
    Char(char),
    End,
}

pub fn make_char_bigram_dist<S, I>(mut iter: I) -> HashMap<Char, HashMap<Char, u32>>
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    let mut map = HashMap::new();
    while let Some(s) = iter.next() {
        let mut pre = Char::Start;
        for c in s.as_ref().chars() {
            let now = Char::Char(c);
            *map.entry(pre)
                .or_insert_with(HashMap::new)
                .entry(now)
                .or_insert(0) += 1;
            pre = Char::Char(c);
        }
        *map.entry(pre)
            .or_insert_with(HashMap::new)
            .entry(Char::End)
            .or_insert(0) += 1;
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::hashmap;

    #[test]
    fn test_make_bigram() {
        let ans = make_char_bigram_dist(vec!["abc", "abab"].into_iter());
        let expect = hashmap! {
            Char::Start => hashmap!{Char::Char('a') => 2},
            Char::Char('a') => hashmap!{Char::Char('b') => 3},
            Char::Char('b') => hashmap!{Char::Char('c') => 1, Char::Char('a') => 1, Char::End => 1},
            Char::Char('c') => hashmap!{Char::End => 1},
        };
        assert_eq!(ans, expect);
    }
}

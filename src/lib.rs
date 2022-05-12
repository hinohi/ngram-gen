mod dist;
pub mod parse_poke;

use std::collections::HashMap;

use dist::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Char {
    Start,
    Char(char),
    End,
}

pub fn make_char_bigram_dist<S, I>(iter: I) -> HashMap<Char, Dist<Char>>
where
    I: Iterator<Item = S>,
    S: AsRef<str>,
{
    let mut map = HashMap::<_, Dist<_>>::new();
    for s in iter {
        let mut pre = Char::Start;
        for c in s.as_ref().chars() {
            let now = Char::Char(c);
            map.entry(pre).or_default().add(now);
            pre = Char::Char(c);
        }
        map.entry(pre).or_default().add(Char::End);
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
            Char::Start => Dist::from_hashmap(hashmap!{Char::Char('a') => 2}),
            Char::Char('a') => Dist::from_hashmap(hashmap!{Char::Char('b') => 3}),
            Char::Char('b') => Dist::from_hashmap(hashmap!{Char::Char('c') => 1, Char::Char('a') => 1, Char::End => 1}),
            Char::Char('c') => Dist::from_hashmap(hashmap!{Char::End => 1}),
        };
        assert_eq!(ans, expect);
    }
}

pub mod parse_poke;

use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Char {
    Start,
    Char(char),
    End,
}

pub fn make_bigram<'a, I>(mut iter: I) -> HashMap<(Char, Char), u32>
where
    I: Iterator<Item = &'a str>,
{
    let mut map = HashMap::new();
    while let Some(s) = iter.next() {
        let mut pre = Char::Start;
        for c in s.chars() {
            *map.entry((pre, Char::Char(c))).or_insert(0) += 1;
            pre = Char::Char(c);
        }
        *map.entry((pre, Char::End)).or_insert(0) += 1;
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_bigram() {
        let ans = make_bigram(vec!["abc", "abab"].into_iter());
        let expect = {
            let mut m = HashMap::new();
            m.insert((Char::Start, Char::Char('a')), 2);
            m.insert((Char::Char('a'), Char::Char('b')), 3);
            m.insert((Char::Char('b'), Char::Char('c')), 1);
            m.insert((Char::Char('b'), Char::Char('a')), 1);
            m.insert((Char::Char('c'), Char::End), 1);
            m.insert((Char::Char('b'), Char::End), 1);
            m
        };
        assert_eq!(ans, expect);
    }
}

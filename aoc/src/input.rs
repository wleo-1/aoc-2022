use itertools::Itertools;
use std::fmt::Debug;
use std::str::{Chars, FromStr};

#[derive(Clone)]
pub struct Input<'a> {
    chars: Chars<'a>,
}

impl<'a> Input<'a> {
    pub fn new(str: &'a str) -> Self {
        Self { chars: str.chars() }
    }

    pub fn next(&mut self) -> Option<char> {
        self.chars.next()
    }

    pub fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    pub fn skip(&mut self, count: usize) {
        self.chars.by_ref().take(count).for_each(drop);
    }

    pub fn expect(&mut self, expect: &str) {
        for char in expect.chars() {
            assert_eq!(self.next(), Some(char));
        }
    }

    pub fn take(&mut self, pred: impl Fn(&char) -> bool) -> String {
        self.chars.peeking_take_while(pred).collect()
    }

    pub fn take_all(&mut self) -> String {
        self.chars.by_ref().collect()
    }

    pub fn parse_int<T: FromStr>(&mut self) -> T
    where
        T::Err: Debug,
    {
        self.chars
            .peeking_take_while(|char| char.is_ascii_digit())
            .collect::<String>()
            .parse::<T>()
            .unwrap()
    }
}

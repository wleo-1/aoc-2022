use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Debug;
use std::iter::Peekable;
use std::str::{Chars, FromStr};

pub struct Extract {
    patterns: Vec<Pattern>,
    output: HashMap<String, Vec<String>>,
}

enum Pattern {
    Fragment(String),
    Value {
        name: String,
        kind: Value,
        list: bool,
    },
}

#[derive(Clone, Copy)]
enum Value {
    Char,
    Number,
    Word,
    String,
}

impl Extract {
    pub fn compile(str: &str) -> Self {
        let mut extract = Extract {
            patterns: Vec::new(),
            output: HashMap::new(),
        };

        let mut chars = str.chars().peekable();
        let mut fragment = String::new();

        while let Some(char) = chars.next() {
            if char == '{' {
                extract.patterns.push(Pattern::Fragment(fragment));
                fragment = String::new();

                // value stuff

                let name = chars
                    .peeking_take_while(|char| char.is_ascii_alphabetic() || *char == '_')
                    .collect();

                assert_eq!(chars.next(), Some(':'));

                let kind = match chars.next().expect("value kind") {
                    'c' => Value::Char,
                    'n' => Value::Number,
                    'w' => Value::Word,
                    's' => Value::String,
                    _ => panic!("invalid value kind"),
                };

                let list = chars.next_if_eq(&',').is_some();

                assert_eq!(chars.next(), Some('}'));

                extract.patterns.push(Pattern::Value { name, kind, list });
            } else {
                fragment.push(char);
            }
        }

        extract.patterns.push(Pattern::Fragment(fragment));

        extract
    }

    pub fn parse_str(&mut self, str: &str) {
        self.parse(&mut str.chars().peekable());
    }

    pub fn parse(&mut self, chars: &mut Peekable<Chars>) {
        self.output.clear();

        for pattern in self.patterns.iter() {
            match pattern {
                Pattern::Fragment(str) => {
                    for char in str.chars() {
                        assert_eq!(Some(char), chars.next());
                    }
                }
                Pattern::Value { name, kind, list } => {
                    let mut vec = vec![Self::parse_value(*kind, chars)];

                    if *list {
                        while chars.next_if_eq(&',').is_some() {
                            assert_eq!(chars.next(), Some(' '));
                            vec.push(Self::parse_value(*kind, chars));
                        }
                    }

                    assert!(self.output.insert(name.clone(), vec).is_none());
                }
            }
        }
    }

    fn parse_value(kind: Value, chars: &mut Peekable<Chars>) -> String {
        let pred = match kind {
            Value::Char => return chars.next().expect("char").to_string(),
            Value::Number => {
                let str = chars
                    .next_if_eq(&'-')
                    .into_iter()
                    .chain(chars.peeking_take_while(char::is_ascii_digit))
                    .collect::<String>();

                assert!(!str.is_empty());

                return str;
            }
            Value::Word => char::is_ascii_alphabetic,
            Value::String => char::is_ascii_graphic,
        };

        let str = chars.peeking_take_while(pred).collect::<String>();

        assert!(!str.is_empty());

        str
    }

    pub fn get<T: FromStr>(&mut self, name: &str) -> T
    where
        T::Err: Debug,
    {
        self.output.get(name).expect("value")[0].parse().unwrap()
    }

    pub fn get_list<T: FromStr>(&mut self, name: &str) -> Vec<T>
    where
        T::Err: Debug,
    {
        self.output
            .get(name)
            .expect("value")
            .into_iter()
            .map(|str| str.parse().unwrap())
            .collect()
    }
}

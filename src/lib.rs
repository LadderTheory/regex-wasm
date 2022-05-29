mod utils;

use std::{str::Chars, f32::consts::E};

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


fn range_quantifier(arg: String) -> Result<Vec<u32>, String> {
    let substring = arg.clone();
    let mut range = vec![];

    let mut sub_chars = substring.chars();

    let comma_parse = |x: &mut Chars, r| {
        Ok(r)
    };

    match sub_chars.next() {
        Some('{') => {
            match Parser::<u32>::parse_num(&mut sub_chars) {
                Ok(x) => {
                    range.push(x);
                    
                    match sub_chars.next() {
                        Some('}') => Ok(range),
                        Some(',') => comma_parse(&mut sub_chars, range),
                        Some(x) => Err(format!("{}", x)),
                        None => Err("Invalid Sequence".to_string()),
                    }
                },
                Err(_) => Err("First argument is not a number".to_string()),
            }
        },
        _ => Err("Improper Delimiter".to_string())
    }
}

struct Parser<T> {
    #[allow(dead_code)]
    placeholder: T
}

impl<T> Parser<T> {
    fn parse_one(sub: &mut impl Iterator<Item = char>, filter: char) -> bool {
        match sub.next() {
            Some(x) => if x == filter { true } else { false },
            _ => false,
        }
    }

    fn parse_num(sub: &mut impl Iterator<Item = char>) -> Result<T, T::Err> where T: std::str::FromStr {
        let mut num = String::from("");

        loop {
            match sub.nth(0).clone() {
                Some(x) => {
                    if sub.nth(0).clone().unwrap().is_digit(10) {
                        num = format!("{}{}", num, sub.nth(0).unwrap());
                        sub.next();
                    } else {
                        break;
                    }
                },
                None => { break; },
            }
        }

        num.parse::<T>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_num() {
        assert_eq!(Parser::<u32>::parse_num(&mut "1234".chars()), Ok(1234));
        assert_eq!(Parser::<u32>::parse_num(&mut "123n4".chars()), Ok(123));
        assert!(Parser::<u32>::parse_num(&mut "n1234".chars()).is_err());

        let mut h = "123}}".chars();
        let _ = Parser::<u32>::parse_num(&mut h);

        assert_eq!(h.collect::<String>(), "}}");
    }

    #[test]
    fn parse_one() {
        let mut h = "{hh}".chars();
        assert_eq!(Parser::<u8>::parse_one(&mut h, '{'), true);
        assert_eq!(Parser::<u8>::parse_one(&mut h, 'h'), true);
        assert_eq!(Parser::<u8>::parse_one(&mut h, '}'), false);
    }

    #[test]
    fn range_qualifier_test() {
        let regex = "{234}".to_string();

        assert_eq!(range_quantifier(regex), Ok(vec![234]));
    }

    #[test]
    fn play() {
        let mut x = [1].iter();
        &x.peekable().peek();

        assert_eq!(x.cloned().next(), None);
    }
}
use itertools::Itertools;
use std::fmt::Display;

pub fn parse_strings(strings: &mut impl Iterator<Item = impl Into<String> + Display>) -> String {
    strings.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_strings() {
        assert_eq!(parse_strings(&mut vec![""].into_iter()), String::from(""));
        assert_eq!(
            parse_strings(&mut vec!["1", "2"].into_iter()),
            String::from("1 2")
        );

        assert_eq!(
            parse_strings(&mut vec!["100", "200"].into_iter()),
            String::from("100 200")
        );
    }
}

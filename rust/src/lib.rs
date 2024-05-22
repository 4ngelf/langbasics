/// Trait to encode text with [__ROT13__](https://wikipedia.org/wiki/ROT13)
///
/// # Example:
/// ```
/// use rot13::Rot13;
///
/// // It works on strings and characters
/// let character = 'D';
/// let text = "This is an slice or an owned String";
///
/// assert_eq!(character.rot13(), 'Q');
/// assert_eq!(text.rot13(), "Guvf vf na fyvpr be na bjarq Fgevat");
/// ```
pub trait Rot13 {
    type Output;
    fn rot13(&self) -> Self::Output;
}

impl Rot13 for char {
    type Output = char;
    fn rot13(&self) -> char {
        match *self {
            'A'..='M' | 'a'..='m' => ((*self as u8) + 13) as char,
            'N'..='Z' | 'n'..='z' => ((*self as u8) - 13) as char,
            _ => *self,
        }
    }
}

impl Rot13 for str {
    type Output = String;
    fn rot13(&self) -> String {
        self.chars().map(|ch| ch.rot13()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROTATED: &str = "NOPQRSTUVWXYZABCDEFGHIJKLMnopqrstuvwxyzabcdefghijklm";

    #[test]
    fn rot13_random_chars() {
        assert_eq!('a'.rot13(), 'n');
        assert_eq!('j'.rot13(), 'w');
        assert_eq!('i'.rot13(), 'v');
        assert_eq!('O'.rot13(), 'B');
    }

    #[test]
    fn rot13_chars() {
        for (character, expected) in ('A'..='Z').chain('a'..='z').zip(ROTATED.chars()) {
            assert_eq!(character.rot13(), expected);
        }
    }

    #[test]
    fn rot13_string() {
        let operand: String = ('A'..='Z').chain('a'..='z').collect();
        let expected: String = String::from(ROTATED);
        assert_eq!(operand.rot13(), expected)
    }
}

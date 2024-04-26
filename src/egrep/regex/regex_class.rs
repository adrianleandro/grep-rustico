#[derive(Debug)]
pub enum RegexClass {
    Alphanumeric,
    Alphabetic,
    Digit,
    Lowercase,
    Uppercase,
    Space,
    Punct,
}

impl RegexClass {
    pub fn new(clase: &str) -> Option<RegexClass> {
        match clase {
            ":alnum:" => Some(Self::Alphanumeric),
            ":alpha:" => Some(Self::Alphabetic),
            ":digit:" => Some(Self::Digit),
            ":lower:" => Some(Self::Lowercase),
            ":upper:" => Some(Self::Uppercase),
            ":space:" => Some(Self::Space),
            ":punct:" => Some(Self::Punct),
            _ => None,
        }
    }

    pub fn contiene(&self, caracter: char) -> bool {
        match self {
            Self::Alphanumeric => caracter.is_ascii_alphanumeric(),
            Self::Alphabetic => caracter.is_ascii_alphabetic(),
            Self::Digit => caracter.is_ascii_digit(),
            Self::Lowercase => caracter.is_ascii_lowercase(),
            Self::Uppercase => caracter.is_ascii_uppercase(),
            Self::Space => caracter.is_ascii_whitespace(),
            Self::Punct => caracter.is_ascii_punctuation(),
        }
    }
}

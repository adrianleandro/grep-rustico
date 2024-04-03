#[derive(Debug, Clone)]
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

    pub fn contiene(&self, caracter: char) -> Option<char> {
        match self {
            Self::Digit => Some(caracter),
            _ => None,
        }
    }
}

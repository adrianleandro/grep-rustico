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
        match (self, caracter) {
            (Self::Alphanumeric, '0'..='9'|'A'..='Z'|'a'..='z') => true,
            (Self::Alphabetic, 'A'..='Z'|'a'..='z') => true,
            (Self::Digit, '0'..='9') => true,
            (Self::Lowercase, 'a'..='z') => true,
            (Self::Uppercase, 'A'..='Z') => true,
            (Self::Space, ' ') => true,
            (Self::Punct, '.'|','|':'|';') => true,
            _ => false,
        }
    }
}

use super::regex_class::RegexClass;
#[derive(Debug, Clone)]
pub enum RegexValue {
    Literal(char),
    Comodin,
    Clase(RegexClass),
}

impl RegexValue {
    pub fn matches(&self, value: &str) -> usize {
        match self {
            RegexValue::Literal(l) => {
                //dbg!(value.chars().next(), l);
                match value.chars().next() {
                    Some(c) => {
                        if *l == c {
                            l.len_utf8()
                        } else {
                            0
                        }
                    }
                    None => 0,
                }
            }
            RegexValue::Comodin => {
                if let Some(c) = value.chars().next() {
                    dbg!(c);
                    c.len_utf8()
                } else {
                    0
                }
            }
            RegexValue::Clase(clase) => 0,
        }
    }
}

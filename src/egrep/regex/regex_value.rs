use super::regex_class::RegexClass;
#[derive(Debug)]
pub enum RegexValue {
    Literal(char),
    Comodin,
    Opcion(Vec<char>),
    Clase(RegexClass),
}

impl RegexValue {
    pub fn matches(&self, value: &str) -> usize {
        match self {
            RegexValue::Literal(l) => {
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
                    c.len_utf8()
                } else {
                    0
                }
            }
            RegexValue::Clase(clase) => {
                match value.chars().next(){
                    Some(caracter) => {
                        if clase.contiene(caracter) {
                            caracter.len_utf8()
                        } else {
                            0
                        }
                    }
                    None => 0,
                }
            },
            RegexValue::Opcion(opciones) => {
                match value.chars().next(){
                    Some(caracter) => {
                        if opciones.contains(&caracter) {
                            caracter.len_utf8()
                        } else {
                            0
                        }
                    }
                    None => 0,
                }
            },
        }
    }
}

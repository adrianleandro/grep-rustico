use super::regex_class::RegexClass;
#[derive(Debug)]
pub enum RegexValue {
    Literal(char),
    Comodin,
    Opcion(Vec<char>, bool),
    Clase(RegexClass, bool),
}

impl RegexValue {
    pub fn matches(&self, value: &str) -> usize {
        match self {
            RegexValue::Literal(literal) => match value.chars().next() {
                Some(caracter) => {
                    if *literal == caracter {
                        literal.len_utf8()
                    } else {
                        0
                    }
                }
                None => 0,
            },
            RegexValue::Comodin => {
                if let Some(caracter) = value.chars().next() {
                    caracter.len_utf8()
                } else {
                    0
                }
            }
            RegexValue::Clase(clase, excluir) => match value.chars().next() {
                Some(caracter) => {
                    if *excluir {
                        if clase.contiene(caracter) {
                            0
                        } else {
                            caracter.len_utf8()
                        }
                    } else if clase.contiene(caracter) {
                        caracter.len_utf8()
                    } else {
                        0
                    }
                }
                None => 0,
            },
            RegexValue::Opcion(opciones, excluir) => match value.chars().next() {
                Some(caracter) => {
                    if *excluir {
                        if opciones.contains(&caracter) {
                            0
                        } else {
                            caracter.len_utf8()
                        }
                    } else if opciones.contains(&caracter) {
                        caracter.len_utf8()
                    } else {
                        0
                    }
                }
                None => 0,
            },
        }
    }
}

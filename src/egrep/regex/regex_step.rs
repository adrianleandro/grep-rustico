use super::{regex_class::RegexClass, regex_rep::RegexRep, regex_value::RegexValue};

#[derive(Debug)]
pub struct RegexStep {
    val: RegexValue,
    rep: RegexRep,
}

impl RegexStep {
    pub fn new(val: RegexValue, rep: RegexRep) -> Self {
        RegexStep { val, rep }
    }

    pub fn new_bracket_expression(expression: String) -> Option<Self> {
        let mut iterador_caracteres = expression.chars();
        let mut opciones: Vec<char> = Vec::new();
        let mut excluir = false;
        while let Some(caracter_dentro_brackets) = iterador_caracteres.next() {
            match caracter_dentro_brackets {
                '[' => {
                    let mut class = String::new();
                    for _ in 0..7 {
                        if let Some(caracter_clase) = iterador_caracteres.next() {
                            class.push(caracter_clase);
                        } else {
                            return None;
                        }
                    }
                    let clase = RegexClass::new(class.as_str());
                    if let Some(clase_encontrada) = clase {
                        return Some(RegexStep::new(
                            RegexValue::Clase(clase_encontrada, excluir),
                            RegexRep::Exact(1),
                        ));
                    } else {
                        return None;
                    }
                }
                ',' => {}
                '^' => excluir = true,
                _ => {
                    opciones.push(caracter_dentro_brackets);
                }
            }
        }
        Some(RegexStep::new(
            RegexValue::Opcion(opciones, excluir),
            RegexRep::Exact(1),
        ))
    }

    pub fn set_exact(&mut self, n: usize) -> &mut Self {
        self.rep = RegexRep::Exact(n);
        self
    }

    pub fn set_any(&mut self) -> &mut Self {
        self.rep = RegexRep::Any;
        self
    }

    pub fn set_repeticiones_rango(&mut self, rango: String) -> &mut Self {
        let mut n: String = String::new();
        let mut m: String = String::new();
        let mut target = &mut n;
        let mut es_exacto = true;
        for caracter_dentro_rango in rango.chars() {
            match caracter_dentro_rango {
                '0'..='9' => target.push(caracter_dentro_rango),
                ',' => {
                    es_exacto = false;
                    target = &mut m
                }
                _ => {}
            }
        }
        match (n.parse::<usize>(), m.parse::<usize>()) {
            (Err(_), Err(_)) => self.set_cero_o_mas(),
            (Err(_), Ok(m)) => self.set_a_m(m),
            (Ok(n), Err(_)) => {
                if es_exacto {
                    self.set_exact(n)
                } else {
                    self.set_n_o_mas(n)
                }
            }
            (Ok(n), Ok(m)) => self.set_n_a_m(n, m),
        }
    }

    pub fn set_cero_o_mas(&mut self) -> &mut Self {
        self.set_n_o_mas(0)
    }

    pub fn set_n_o_mas(&mut self, n: usize) -> &mut Self {
        self.rep = RegexRep::Range {
            min: Some(n),
            max: None,
        };
        self
    }

    pub fn set_a_m(&mut self, m: usize) -> &mut Self {
        self.rep = RegexRep::Range {
            min: None,
            max: Some(m),
        };
        self
    }

    pub fn set_n_a_m(&mut self, n: usize, m: usize) -> &mut Self {
        self.rep = RegexRep::Range {
            min: Some(n),
            max: Some(m),
        };
        self
    }

    pub fn get_repetitions(&self) -> &RegexRep {
        &self.rep
    }

    pub fn get_value(&self) -> &RegexValue {
        &self.val
    }
}

use self::{regex_rep::RegexRep, regex_step::RegexStep, regex_value::RegexValue};

mod regex_class;
mod regex_rep;
mod regex_step;
mod regex_value;

#[derive(Debug)]
pub struct Regex {
    steps: Vec<regex_step::RegexStep>,
}

impl Regex {
    /// Crea una expresion regular con una serie de pasos a seguir para verificar si la misma se encuentra dentro de una linea de texto.
    pub fn new(expression: &str) -> Result<Self, &str> {
        if expression.len() == 0{
            return Err("Expresion regular inválida");
        }

        let mut steps: Vec<regex_step::RegexStep> = Vec::new();

        let mut iterador_caracteres = expression.chars();

        while let Some(c) = iterador_caracteres.next() {
            let step = match c {
                '.' => Some(RegexStep::new(RegexValue::Comodin, RegexRep::Exact(1))),
                'a'..='z' | '0'..='9' | ' ' => {
                    Some(RegexStep::new(RegexValue::Literal(c), RegexRep::Exact(1)))
                }
                '*' => {
                    if let Some(last) = steps.last_mut() {
                        last.set_cero_o_mas();
                    } else {
                        return Err("Se encontró un caracter '*' inesperado");
                    }

                    None
                }
                '+' => {
                    if let Some(last) = steps.last_mut() {
                        last.set_uno_o_mas();
                    } else {
                        return Err("Se encontró un caracter '+' inesperado");
                    }

                    None
                }
                '?' => {
                    if let Some(last) = steps.last_mut() {
                        last.set_n_a_m(0,1);
                    } else {
                        return Err("Se encontró un caracter '?' inesperado");
                    }

                    None
                }
                '\\' => match iterador_caracteres.next() {
                    Some(literal) => Some(RegexStep::new(
                        RegexValue::Literal(literal),
                        RegexRep::Exact(1),
                    )),
                    None => return Err("se encontró un caracter inesperado"),
                },
                '[' => {
                    while let Some(c) = iterador_caracteres.next() {
                        if c == ']' {
                            break;
                        } else {
                        }
                    }
                    Some(RegexStep::new(RegexValue::Literal('a'), RegexRep::Exact(1)))
                }
                _ => return Err("Se encontró un caracter inesperado"),
            };

            if let Some(p) = step {
                steps.push(p);
            }
        }

        Ok(Regex { steps })
    }

    /// Recibe una linea de texto y la evalua según la expresión regular.  
    /// Se devuelve verdadero o falso dependiendo de que la expresión se encuentre en la linea.  
    /// Devuelve error en caso de que la linea de texto contenga algun carácter que no pertenezca al formato ASCII.  
    pub fn testear_linea(&self, value: &str) -> Result<bool, &str> {
        if !value.is_ascii() {
            return Err("el input no es ASCII");
        }

        let mut iter;
        let mut index = 0;

        while index < value.len() {
            iter = self.steps.iter().peekable();
            let comienzo_match = index;
            while let Some(step) = iter.next() {
                let mut step_cumplido = true;
                match step.get_repetitions() {
                    RegexRep::Exact(n) => {
                        for _ in [1..*n] {
                            let size = step.get_value().matches(&value[index..]);

                            if size == 0 {
                                step_cumplido = false;
                                break;
                            }

                            index += size;
                        }
                    }
                    RegexRep::Range { min, max } => {
                        let mut total_matches = 0;
                        let mut seguir_matcheando = true;
                        match (min, max) {
                            (None, None) => {},
                            (Some(n), None) => {
                                while seguir_matcheando {
                                    let size = step.get_value().matches(&value[index..]);

                                    if size != 0 {
                                        index += size;
                                        total_matches += 1;
                                    } else {
                                        seguir_matcheando = false;
                                    }
                                }
                                if(total_matches < *n){
                                    step_cumplido = false;
                                }
                            },
                            (None, Some(m)) => {
                                while seguir_matcheando {
                                    let size = step.get_value().matches(&value[index..]);

                                    if size != 0 {
                                        index += size;
                                        total_matches += 1;
                                    } else {
                                        seguir_matcheando = false;
                                    }
                                }
                                if(total_matches > *m){
                                    step_cumplido = false;
                                }
                            },
                            (Some(n), Some(m)) => {
                                while seguir_matcheando {
                                    let size = step.get_value().matches(&value[index..]);

                                    if size != 0 {
                                        index += size;
                                        total_matches += 1;
                                    } else {
                                        seguir_matcheando = false;
                                    }
                                }
                                if(total_matches > *m || total_matches < *n){
                                    step_cumplido = false;
                                }
                            },
                        }
                    },
                }
                if !step_cumplido {
                    index = comienzo_match + 1;
                    break;
                }
                if let None = iter.peek() {
                    return Ok(true);
                };
            }
        }
        Ok(false)
    }
}

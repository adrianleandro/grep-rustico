use self::{regex_rep::RegexRep, regex_step::RegexStep, regex_value::RegexValue};

mod regex_class;
mod regex_rep;
mod regex_step;
mod regex_value;

#[derive(Debug, Clone)]
pub struct Regex {
    steps: Vec<regex_step::RegexStep>,
}

impl Regex {

    /// Crea una expresion regular con una serie de pasos a seguir para verificar si la misma se encuentra dentro de una linea de texto
    pub fn new(expression: &str) -> Result<Self, &str> {
        let mut steps: Vec<regex_step::RegexStep> = Vec::new();

        let mut iterador_caracteres = expression.chars();

        while let Some(c) = iterador_caracteres.next() {
            let step = match c {
                '.' => Some(RegexStep::new(RegexValue::Comodin, RegexRep::Exact(1))),
                'a'..='z' | '0'..='9' => {
                    Some(RegexStep::new(RegexValue::Literal(c), RegexRep::Exact(1)))
                }
                '*' => {
                    if let Some(last) = steps.last_mut() {
                        last.set_infinite_repetitions();
                    } else {
                        return Err("Se encontró un caracter '*' inesperado");
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
                _ => return Err("Se encontró un caracter inesperado"),
            };

            if let Some(p) = step {
                steps.push(p);
            }
        }

        Ok(Regex { steps })
    }

    pub fn testear_linea(&self, value: &str) -> Result<bool, &str> {
        if !value.is_ascii() {
            return Err("el input no es ASCII");
        }

        let mut iter;
        let mut index = 0;

        loop {
            iter = self.steps.iter();
            while let Some(step) = iter.next() {
                let mut step_cumplido = true;
                let mut total_size = 0;
                match step.get_repetitions() {
                    RegexRep::Exact(n) => {
                        for _ in [1..*n] {
                            let size = step.get_value().matches(&value[index..]);

                            if size == 0 {
                                step_cumplido = false;
                                break;
                            }

                            total_size += size;
                        }
                    }
                    _ => todo!()
                }
                if step_cumplido {
                    index += total_size;
                } else {
                    index += 1;
                    break;
                }
            }
            if let None = iter.next() {
                return Ok(true);
            };

            if index >= value.len() - 1{
                return Ok(false);
            }
        }
    }
}

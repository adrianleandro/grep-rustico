use std::collections::VecDeque;

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

    pub fn test(self, value: &str) -> Result<bool, &str> {
        if !value.is_ascii() {
            return Err("el input no es ASCII");
        }

        let mut queue = VecDeque::from(self.steps);
        let mut index = 0;

        while let Some(step) = queue.pop_front() {
            match step.get_repetitions() {
                RegexRep::Exact(n) => {
                    for _ in [1..*n] {
                        let size = step.get_value().matches(&value[index..]);

                        if size == 0 {
                            // bakctracking?
                            return Ok(false);
                        }

                        index += size;
                    }
                }
                RegexRep::Any => {
                    let mut keep_matching = true;

                    while keep_matching {
                        let match_size = step.get_value().matches(&value[index..]);

                        if match_size != 0 {
                            index += match_size;
                        } else {
                            keep_matching = false;
                        }
                    }
                }
                RegexRep::Range { min, max } => todo!(),
            }
        }

        Ok(true)
    }
}

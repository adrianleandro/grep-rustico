use self::{regex_rep::RegexRep, regex_step::RegexStep, regex_value::RegexValue};

mod regex_class;
mod regex_rep;
mod regex_step;
mod regex_value;

#[derive(Debug)]
pub struct Regex {
    steps: Vec<regex_step::RegexStep>,
    evaluar_desde_principio: bool,
    evaluar_desde_final: bool,
}

impl Regex {
    /// Crea una expresion regular con una serie de pasos a seguir para verificar si la misma se encuentra dentro de una linea de texto.
    pub fn new(expression: &str) -> Result<Self, &str> {
        if expression.is_empty() || !expression.is_ascii() {
            return Err("Expresion regular inválida");
        }

        let mut steps: Vec<regex_step::RegexStep> = Vec::new();
        let mut evaluar_desde_principio = false;
        let mut evaluar_desde_final = false;

        let mut iterador_caracteres = expression.chars().peekable();

        while let Some(c) = iterador_caracteres.next() {
            let step = match c {
                '.' => Some(RegexStep::new(RegexValue::Comodin, RegexRep::Exact(1))),
                '^' => {
                    if steps.last_mut().is_some() {
                        return Err("Se encontró un caracter '^' inesperado");
                    }
                    evaluar_desde_principio = true;
                    None
                }
                '$' => {
                    if iterador_caracteres.peek().is_some() {
                        return Err("Se encontró un caracter '$' inesperado");
                    }
                    evaluar_desde_final = true;
                    None
                }
                '*' => {
                    if let Some(last) = steps.last_mut() {
                        last.set_any();
                    } else {
                        return Err("Se encontró un caracter '*' inesperado");
                    }
                    None
                }
                '+' => {
                    if let Some(last) = steps.last_mut() {
                        last.set_n_o_mas(1);
                    } else {
                        return Err("Se encontró un caracter '+' inesperado");
                    }
                    None
                }
                '?' => {
                    if let Some(last) = steps.last_mut() {
                        last.set_n_a_m(0, 1);
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
                    None => return Err("Se encontró un caracter inesperado"),
                },
                '{' => {
                    let mut rango = String::new();
                    let mut nro_comas = 0;
                    for p in iterador_caracteres.by_ref() {
                        match p {
                            '}' => {
                                if rango.is_empty() {
                                    return Err("Contenido de operador {} invalido");
                                } else {
                                    break;
                                }
                            }
                            ',' => {
                                nro_comas += 1;
                                rango.push(p);
                            }
                            '0'..='9' => {
                                rango.push(p);
                            }
                            _ => {
                                return Err(
                                    "Se encontró un caracter inesperado dentro del operador {}",
                                );
                            }
                        }
                        if nro_comas > 1 {
                            return Err("Contenido de operador {} invalido");
                        }
                    }
                    if let Some(last) = steps.last_mut() {
                        last.set_repeticiones_rango(rango);
                    } else {
                        return Err("Se encontró un caracter '{' inesperado");
                    }
                    None
                }
                '[' => {
                    let mut contenido = String::new();
                    let mut cantidad_corchetes = 1;
                    for a in iterador_caracteres.by_ref() {
                        match a {
                            ']' => {
                                cantidad_corchetes -= 1;
                                if cantidad_corchetes == 0 {
                                    break;
                                }
                            }
                            '[' => cantidad_corchetes += 1,
                            _ => {}
                        }
                        contenido.push(a);
                    }
                    if contenido.is_empty() || cantidad_corchetes > 0 {
                        return Err("Contenido de operador [] inválido");
                    }
                    RegexStep::new_bracket_expression(contenido)
                }
                ' '..='~' => Some(RegexStep::new(RegexValue::Literal(c), RegexRep::Exact(1))),
                _ => return Err("Se encontró un caracter inesperado"),
            };
            if let Some(p) = step {
                steps.push(p);
            }
        }
        if !evaluar_desde_principio && evaluar_desde_final {
            steps.reverse()
        }
        Ok(Regex {
            steps,
            evaluar_desde_principio,
            evaluar_desde_final,
        })
    }

    /// Recibe una linea de texto y la evalua según la expresión regular.  
    /// Se devuelve verdadero o falso dependiendo de que la expresión se encuentre en la linea.  
    /// Devuelve error en caso de que la linea de texto contenga algun carácter que no pertenezca al formato ASCII.  
    pub fn testear_linea(&self, value: &str) -> Result<(usize, usize), &str> {
        if !value.is_ascii() {
            return Err("La linea leida no está en formato ASCII");
        }

        let mut iter;
        let mut index = 0;

        let mut value = value.to_string();
        if self.evaluar_desde_final && !self.evaluar_desde_principio {
            value = value.chars().rev().collect::<String>();
        }

        while index < value.len() {
            iter = self.steps.iter().peekable();
            let comienzo_match = index;
            while let Some(step) = iter.next() {
                let mut step_cumplido = true;
                match step.get_repetitions() {
                    RegexRep::Exact(n) => {
                        for _ in 1..=*n {
                            let size = step.get_value().matches(&value[index..]);
                            if size == 0 {
                                step_cumplido = false;
                                break;
                            }

                            index += size;
                        }
                    }
                    RegexRep::Any => {
                        let mut seguir_matcheando = true;
                        while seguir_matcheando {
                            let size = step.get_value().matches(&value[index..]);
                            let mut next_step_size = 0;
                            if let Some(next_step) = iter.peek() {
                                next_step_size = next_step.get_value().matches(&value[index..]);
                            }
                            match (size, next_step_size) {
                                (0, _) => seguir_matcheando = false,
                                (_, 0) => index += size,
                                (_, _) => seguir_matcheando = false,
                            }
                        }
                    }
                    RegexRep::Range { min, max } => {
                        let mut total_matches = 0;
                        let mut seguir_matcheando = true;
                        while seguir_matcheando {
                            let size = step.get_value().matches(&value[index..]);
                            if size != 0 {
                                index += size;
                                total_matches += 1;
                            } else {
                                seguir_matcheando = false;
                            }
                        }
                        match (min, max) {
                            (None, None) => {}
                            (Some(n), None) => {
                                if total_matches < *n {
                                    step_cumplido = false;
                                }
                            }
                            (None, Some(m)) => {
                                if total_matches > *m {
                                    step_cumplido = false;
                                }
                            }
                            (Some(n), Some(m)) => {
                                if total_matches > *m || total_matches < *n {
                                    step_cumplido = false;
                                }
                            }
                        }
                    }
                }
                if !step_cumplido {
                    if self.evaluar_desde_principio && self.evaluar_desde_final {
                        return Ok((0, 0));
                    }
                    index = comienzo_match + 1;
                    break;
                }
                if iter.peek().is_none() {
                    match (self.evaluar_desde_principio, self.evaluar_desde_final) {
                        (false, true) => {
                            return Ok((value.len() - index, value.len() - comienzo_match))
                        }
                        (true, false) | (false, false) => return Ok((comienzo_match, index)),
                        (true, true) => {
                            if index == (value.len() - 1) {
                                return Ok((comienzo_match, index));
                            } else {
                                return Ok((0, 0));
                            }
                        }
                    }
                };
            }
        }
        Ok((0, 0))
    }
}

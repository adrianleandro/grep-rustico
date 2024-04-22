use std::str::FromStr;

use super::{regex_rep::RegexRep, regex_value::RegexValue};

#[derive(Debug)]
pub struct RegexStep {
    val: RegexValue,
    rep: RegexRep,
}

impl RegexStep {
    pub fn new(val: RegexValue, rep: RegexRep) -> Self {
        RegexStep { val, rep }
    }

    pub fn set_exact(&mut self, n: usize) -> &mut Self {
        self.rep = RegexRep::Exact(n);
        self
    }

    pub fn set_repeticiones_rango(&mut self, rango: String) -> &mut Self {
        let mut n: String = String::new();
        let mut m: String = String::new();
        let mut caracteres = rango.chars();
        let mut target = &mut n;
        let mut es_exacto = true;
        while let Some(c) = caracteres.next() {
            match c {
                '0'..='9' => target.push(c),
                ',' => {
                    es_exacto = false;
                    target = &mut m
                }
                _ => {}
            }
        }
        match (n.parse::<usize>(), m.parse::<usize>()) {
            (Err(_), Err(_)) => self,
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

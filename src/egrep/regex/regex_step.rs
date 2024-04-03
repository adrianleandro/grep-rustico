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

    pub fn set_cero_o_mas(&mut self) -> &mut Self {
        self.set_n_o_mas(0)
    }

    pub fn set_uno_o_mas(&mut self) -> &mut Self {
        self.set_n_o_mas(1)
    }

    pub fn set_n_o_mas(&mut self, n: usize) -> &mut Self {
        self.rep = RegexRep::Range { min: Some(0), max: None };
        self
    }

    pub fn set_n_a_m(&mut self, n: usize, m: usize) -> &mut Self {
        self.rep = RegexRep::Range { min: Some(n), max: Some(m) };
        self
    }
    
    pub fn set_a_m(&mut self, m: usize) -> &mut Self {
        self.rep = RegexRep::Range { min: None, max: Some(m) };
        self
    }

    pub fn get_repetitions(&self) -> &RegexRep {
        &self.rep
    }

    pub fn get_value(&self) -> &RegexValue {
        &self.val
    }
}

use super::{regex_rep::RegexRep, regex_value::RegexValue};

#[derive(Debug, Clone)]
pub struct RegexStep {
    val: RegexValue,
    rep: RegexRep,
}

impl RegexStep {
    pub fn new(val: RegexValue, rep: RegexRep) -> Self {
        RegexStep { val, rep }
    }

    pub fn set_infinite_repetitions(&mut self) -> &mut Self {
        self.rep = RegexRep::Any;
        self
    }

    pub fn get_repetitions(&self) -> &RegexRep {
        &self.rep
    }

    pub fn get_value(&self) -> &RegexValue {
        &self.val
    }
}

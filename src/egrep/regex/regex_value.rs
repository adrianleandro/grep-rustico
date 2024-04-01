use super::regex_class::RegexClass;
#[derive(Debug)]
pub enum RegexValue{
    Literal(char),
    Comodin,
    Clase(RegexClass),
}

impl RegexValue {
    pub fn matches (&self, value:&str) -> usize {
        match self {
            RegexValue::Literal(l) => {
                if value.chars().next() == Some(*l){
                    l.len_utf8() //cantidad consumida por el input
                }else{
                    0
                }
            },
            RegexValue::Comodin =>{
                if let Some(c) = value.chars().next(){
                    c.len_utf8()//cantidad consumida por el input
                }else{
                    0
                }
            },
            RegexValue::Clase(clase) => {
                0
            }
        }
    }
}
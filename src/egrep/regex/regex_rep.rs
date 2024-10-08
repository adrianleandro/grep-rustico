#[derive(Debug)]
pub enum RegexRep {
    Exact(usize),
    Any,
    Range {
        min: Option<usize>,
        max: Option<usize>,
    },
}

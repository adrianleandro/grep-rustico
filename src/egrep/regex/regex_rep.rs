#[derive(Debug)]
pub enum RegexRep {
    Any,
    Exact(usize), //{n}
    Range {
        min: Option<usize>,
        max: Option<usize>,
    },
}

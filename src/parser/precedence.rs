use std::cmp::Ordering;
#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
    Call,
}

impl Ord for Precedence {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Precedence::Lowest, Precedence::Lowest)
            | (Precedence::Equals, Precedence::Equals)
            | (Precedence::LessGreater, Precedence::LessGreater)
            | (Precedence::Sum, Precedence::Sum)
            | (Precedence::Product, Precedence::Product)
            | (Precedence::Prefix, Precedence::Prefix)
            | (Precedence::Call, Precedence::Call) => std::cmp::Ordering::Equal,

            (Precedence::Call, _) => std::cmp::Ordering::Greater,
            (_, Precedence::Call) => std::cmp::Ordering::Less,

            (Precedence::Prefix, _) => std::cmp::Ordering::Greater,
            (_, Precedence::Prefix) => std::cmp::Ordering::Less,

            (Precedence::Product, _) => std::cmp::Ordering::Greater,
            (_, Precedence::Product) => std::cmp::Ordering::Less,

            (Precedence::Sum, _) => std::cmp::Ordering::Greater,
            (_, Precedence::Sum) => std::cmp::Ordering::Less,

            (Precedence::LessGreater, _) => std::cmp::Ordering::Greater,
            (_, Precedence::LessGreater) => std::cmp::Ordering::Less,

            (Precedence::Equals, _) => std::cmp::Ordering::Greater,
            (_, Precedence::Equals) => std::cmp::Ordering::Less,
        }
    }
}

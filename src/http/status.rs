use std::fmt::{Display, Formatter};


#[derive(Clone, Copy)]
pub enum Status {
    OK = 200,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match *self {
            Self::OK => "OK"
        })
    }
} 
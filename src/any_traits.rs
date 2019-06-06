use std::fmt::{Display, Error, Formatter};
use crate::Datum;


impl<'a> Display for Datum<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        use Datum::*;
        match self {
            Collection_(c) => write!(f, "[{}]", c.iter().map(|d| d.to_string()).collect::<Vec<String>>().join(", ")),
            Number_(n) => write!(f, "{}", n),
            String_(s) => write!(f, "{}", s),
            Table_(_) => write!(f, "<Table>"),
            Function_(_) => write!(f, "<Fn>"),
            None_ => write!(f, "<None>"),
            _ => write!(f, ""),
        }
    }
}


impl<'a> From<Datum<'a>> for i32 {
    fn from(datum: Datum) -> Self {
        match datum {
            Datum::Number_(n) => n.round() as i32,
            _ => 0
        }
    }
}


impl<'a> From<Datum<'a>> for f64 {
    fn from(datum: Datum) -> Self {
        match datum {
            Datum::Number_(n) => n,
            _ => 0.0
        }
    }
}
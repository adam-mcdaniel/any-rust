use std::fmt::{Display, Error, Formatter};
use std::ops::{Index,IndexMut};
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



impl<'a, 'b> Index<Datum<'b>> for Datum<'a> {
    type Output = Datum<'a>;

    fn index(&'a self, index: Datum<'b>) -> &'a Self::Output {
        use Datum::*;
        match index {
            Number_(n) => {

            },
            String_(s) => {

            },
            _ => {

            }
        }
        Datum::none()
    }
}

// impl IndexMut<Side> for Balance {
//     fn index_mut<'a>(&'a mut self, index: Side) -> &'a mut Self::Output {
//         println!("Accessing {:?}-side of balance mutably", index);
//         match index {
//             Side::Left => &mut self.left,
//             Side::Right => &mut self.right,
//         }
//     }
// }

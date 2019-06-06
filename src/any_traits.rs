use std::fmt::{Display, Error, Formatter};
use std::sync::Arc;
use std::cell::{RefCell};
use std::rc::Rc;
use std::ops::{Index,IndexMut};
use crate::{Datum, FunctionType};


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

    fn index(&self, index: Datum<'b>) -> &Self::Output {
        use Datum::*;
        match index {
            Number_(n) => {
                match self {
                    Collection_(c) => &c[n as i32 as usize],
                    _ => self
                }
            },

            String_(s) => {
                match self {
                    Table_(rf) =>{
                        unsafe {
                            match (*rf.as_ptr()).get(&s.to_string()) {
                                Some(rf) => &*rf.as_ptr(),
                                None => {
                                    (*rf.as_ptr()).insert(s.to_string(), RefCell::new(Datum::none()));
                                    &*(*rf.as_ptr()).get(&s.to_string()).unwrap().as_ptr()
                                }
                            }
                        }
                    },
                    _ => self
                }
            },
            
            _ => {
                self
            }
        }
    }
}


impl<'a, 'b> IndexMut<Datum<'b>> for Datum<'a> {
    fn index_mut(&mut self, index: Datum<'b>) -> &mut Self::Output {
        use Datum::*;
        match index {
            Number_(n) => {
                match self {
                    Collection_(c) => &mut c[n as i32 as usize],
                    _ => self
                }
            },

            String_(s) => {
                match self {
                    Table_(rf) =>{
                        unsafe {
                            match (*rf.as_ptr()).get(&s.to_string()) {
                                Some(rf) => &mut (*rf.as_ptr()),
                                None => {
                                    (*rf.as_ptr()).insert(s.to_string(), RefCell::new(Datum::table()));
                                    &mut (*(*rf.as_ptr()).get(&s.to_string()).unwrap().as_ptr())
                                }
                            }
                        }
                    },
                    _ => self
                }
            },
            
            _ => {
                self
            }
        }
    }
}


impl<'a> From<Datum<'a>> for FunctionType<'a> {
    fn from(datum: Datum<'a>) -> Self {
        use Datum::*;
        match datum {
            Function_(f) => f.clone(),
            _ => Arc::new(|d: Datum| {d})
        }
    }
}

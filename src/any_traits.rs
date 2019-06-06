use std::fmt::{Display, Error, Formatter};
use std::sync::Arc;
use std::cell::{RefCell};
use std::ops::{Index,IndexMut};
use crate::{Datum, FunctionType, CollectionType};

// Trait for printing datum and converting datum to string
impl<'a> Display for Datum<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        use Datum::*;
        match self {
            Collection_(c) => write!(f, "[{}]", c.iter().map(|d| d.to_string()).collect::<Vec<String>>().join(", ")),
            Number_(n) => write!(f, "{}", n), // Print number
            String_(s) => write!(f, "{}", s), // Print string
            Table_(t) => {
                // Iterate over table and print keys and values 
                let mut result = "".to_owned();
                for (key, value) in (*t.borrow()).iter() {
                    result += &format!("{}: '{}', ", key, value.borrow());
                }
                result.pop();
                result.pop();
                write!(f, "{{{}}}", result)
            },
            Function_(_) => write!(f, "<Fn>"), // Cant print function
            None_ => write!(f, "<None>"), // None is whatever
        }
    }
}

// Convert Datum to FunctionType
// This is used when calling .call_method or .call for datum
impl<'a> From<Datum<'a>> for FunctionType<'a> {
    fn from(datum: Datum<'a>) -> Self {
        use Datum::*;
        match datum {
            Function_(f) => f.clone(),
            _ => Arc::new(|d: Datum| {d})
        }
    }
}

// Convert to collection.
// If the datum is not already a collection,
// destroy data and return an empty collection.
// If the datum is a collection, return the collection
impl<'a> From<Datum<'a>> for CollectionType<'a> {
    fn from(datum: Datum<'a>) -> Self {
        match datum {
            Datum::Collection_(c) => c,
            _ => vec![]
        }
    }
}


// Convert a datum to an i32
// This is convenient for foreign functions especially
impl<'a> From<Datum<'a>> for i32 {
    fn from(datum: Datum) -> Self {
        match datum {
            Datum::Number_(n) => n.round() as i32,
            _ => 0
        }
    }
}


// Convert a datum to an f64
// This is convenient for foreign functions especially
impl<'a> From<Datum<'a>> for f64 {
    fn from(datum: Datum) -> Self {
        match datum {
            Datum::Number_(n) => n,
            _ => 0.0
        }
    }
}


// Convert i32 to Datum
impl<'a> From<i32> for Datum<'a> {
    fn from(n: i32) -> Self {
        Datum::Number_(n as f64)
    }
}


// Convert f64 to Datum
impl<'a> From<f64> for Datum<'a> {
    fn from(n: f64) -> Self {
        Datum::Number_(n)
    }
}

// Convert &str to Datum
impl<'a> From<&str> for Datum<'a> {
    fn from(s: &str) -> Self {
        Datum::string(s)
    }
}

// Convert String to Datum
impl<'a> From<String> for Datum<'a> {
    fn from(s: String) -> Self {
        Datum::string(s)
    }
}


// Immutably index datum with string or number
impl<'a, 'b, I> Index<I> for Datum<'a> where I: Into<Datum<'b>> {
    type Output = Datum<'a>;

    fn index(&self, index: I) -> &Self::Output {
        use Datum::*;
        match index.into() {
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


impl<'a, 'b, I> IndexMut<I> for Datum<'a> where I: Into<Datum<'b>> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        use Datum::*;
        match index.into() {
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

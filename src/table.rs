use std::ops::{Deref, DerefMut};
use std::fmt::{Display, Error, Formatter};

use std::cmp::Eq;
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Table {
    name: String,
    table: RefCell<HashMap<String, RefCell<Self>>>
}


impl Table {
    pub fn new<S: ToString>(name: S) -> Self {
        Self {
            name: name.to_string(),
            table: RefCell::new(HashMap::new())
        }
    }

    pub fn add_key<S: ToString>(&self, name: S) {
        self.table.borrow_mut().insert(
            name.to_string(), 
            RefCell::new(Self::new(""))
        );
    }

    pub fn get<S: ToString>(&self, s: S) -> RefMut<Self> {
        unsafe {
            match (*self.table.as_ptr()).get(&s.to_string()) {
                Some(rf) => rf.borrow_mut(),
                None => {
                    self.add_key(s.to_string());
                    (*self.table.as_ptr()).get(&s.to_string()).unwrap().borrow_mut()
                }
            }
        }
    }
}



impl Display for Table {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Table<{}>", self.name)
    }
}
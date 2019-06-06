use std::cell::{RefCell,};
use std::collections::HashMap;
use std::sync::Arc;


type CollectionType<'a> = Vec<Datum<'a>>;
type StringType = String;
type NumberType = f64;
type FunctionType<'a> = Arc<dyn 'a + Fn(Datum) -> Datum>;
type TableType<'a> = RefCell<HashMap<StringType, RefCell<Datum<'a>>>>;


#[derive(Clone)]
pub enum Datum<'a> {
    Collection_(CollectionType<'a>),
    String_(StringType),
    Number_(NumberType),
    Function_(FunctionType<'a>),
    Table_(TableType<'a>),
    None_
}


impl<'a> Datum<'a> {
    pub fn collection<C: Into<Vec<Datum<'a>>>>(collection: C) -> Self {
        Datum::Collection_(collection.into())
    }

    pub fn string<S: ToString>(s: S) -> Self {
        Datum::String_(s.to_string())
    }

    pub fn number<N: Into<NumberType>>(n: N) -> Self {
        Datum::Number_(n.into())
    }

    pub fn function<F: 'a + Fn(Datum) -> Datum>(f: F) -> Self {
        Datum::Function_(Arc::new(f))
    }
    
    pub fn none() -> Self { Datum::None_ }
    
    pub fn table() -> Self {
        Datum::Table_(
            TableType::new(HashMap::new())
        )
    }
}
use std::cell::{RefCell,};
use std::collections::HashMap;
use std::sync::Arc;


#[macro_export]
macro_rules! args {
    ($($e:expr),*) => {
        any::collection(vec![$($e),*])
    };
}

#[macro_export]
macro_rules! returns {
    ($($e:expr),+) => {
        any::collection(vec![$($e),+])
    };
}

#[macro_export]
macro_rules! function {
    (($($arg:ident),*) $b:expr) => {{
        any::function(|__ARGS__| {
            let mut __ARGS_COUNT__ = 0;
            $(
            let mut $arg = __ARGS__[any::number(__ARGS_COUNT__)].clone(); __ARGS_COUNT__ += 1;
            )+

            return $b;
        })
    }}
}


pub type CollectionType<'a> = Vec<Datum<'a>>;
pub type StringType = String;
pub type NumberType = f64;
pub type FunctionType<'a> = Arc<dyn 'a + Fn(Datum) -> Datum>;
pub type TableType<'a> = RefCell<HashMap<StringType, RefCell<Datum<'a>>>>;


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


    pub fn call(&self, call_value: Self) -> Self {
        let args = CollectionType::from(call_value);

        let result = (*FunctionType::from(self.clone())) (
            Self::collection(args)
        );
        let result_vec: CollectionType = result.into();
        return Self::collection(result_vec);
    }


    // Methods are called like so.
    // The method looks like this: Arc<dyn Fn(Datum) -> Datum>
    // 
    // It is called like so.
    // 
    // [this, results...] = (*method) (
    //      Self::collection(vec![self.clone(), args...]) //make a list of args with this at front
    // ) // returns list of return values with modified this at front
    // 
    pub fn call_method<S: ToString>(&mut self, method_name: S, call_value: Self) -> Self {
        let method = self[Self::string(method_name)].clone();
        let mut args = vec![self.clone()];
        args.extend(CollectionType::from(call_value));
        
        let result = (*FunctionType::from(method)) (
            Self::collection(args)
        );

        // self = &result[Self::number(0)];
        *self = result[Self::number(0)].clone();
        let result_vec: CollectionType = result.into();

        let without_this = result_vec[1..].to_vec();
        Self::collection(without_this)
    }
}

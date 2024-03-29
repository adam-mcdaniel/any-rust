use any_rust::{
    Datum as any,
    args, returns, function
};



fn main() {
    let mut obj = any::table();
    
    println!("{}", 
        args!(
            any::number(5)
        )
    );

    obj[any::string("method")] = function!(
        (this, n) {
            n["whoa"] = 5.into();
            n["baby"] = "momma".into();

            this["banana"] = n.clone();
            returns!(this, n, "test".into())
        }
    );

    println!("result {}",
            obj.call_method("method", args!(any::table())
        )
    );


    println!(
        "obj[banana] = {}",
        obj["banana"]
    );

    println!(
        "obj[banana][whoa] = {}",
        obj["banana"]["whoa"]
    );


    let add = function!(
        (x, y) {
            returns!(any::number(f64::from(x) + f64::from(y)))
        }
    );

    
    println!(
        "add: {}",
        add.call(
            args!(5.into(), 23.into())
        )
    );


    // let c = any::collection(vec![
    //         any::string("five"),
    //         any::number(123),
    //         any::function(|d| {
    //             d
    //         }),
    //     ]);

    // println!("collection: {}", c.clone());
    // println!("index: {}", c.clone()[any::number(2)]);

    // let n = any::number(5.1);
    // println!(
    //     "int: '{}'\nfloat: '{}'",
    //     i32::from(n.clone()),
    //     f64::from(n.clone()),
    // );

    // let mut a = any::table();
    // a[any::string("five")] = any::number(7);

    // let mut b = any::table();
    // b[any::string("test")][any::string("whoa")] = a;
    // a = any::number(5);
    // println!("a = {}", a);


    // println!("test = {}", b[any::string("whoa_this_is_wrong")]);
    // b[any::string("whoa_this_is_wrong")] = any::number(19.5);
    // println!("test = {}", b[any::string("whoa_this_is_wrong")]);

    // println!(
    //     "b/test/whoa = {}",
    //     b[any::string("test")][any::string("whoa")]
    // );


    // let s: String = any::number(5.000000001).to_string();
    // println!("out: {}", s);

    // let f = |t: &Table| {
    //     println!("base_table/test = {}", *t.get("test"));
    // };
    // let table = Table::new("base_table");
    // println!("base_table = {}", table);
    // *table.get("test") = Table::new("testing");
    // f(&table);
    // *table.get("test") = Table::new("Whoa!!!");
    // f(&table);


    // let t = MyType::new("One");

    // t.set("test", "Two");
    // t.get("test").set("testing", "Five");

    // println!("One/test = {}", t.get("test").table);
    // println!("One/test/Two = {}", t.get("test").get("testing").table);
}

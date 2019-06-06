use any_rust::Datum as any;


fn main() {

    let c = any::collection(vec![
            any::string("five"),
            any::number(123),
            any::function(|d| {
                d
            }),
        ]);

    println!("collection: {}", c);

    let n = any::number(5.1);
    println!(
        "int: '{}'\nfloat: '{}'",
        i32::from(n.clone()),
        f64::from(n.clone()),
    );

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
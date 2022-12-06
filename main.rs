fn main() {
    let mut v: Vec<(String, i32)>;
    v = vec![("sabiha".to_string(), 21)];
    println!("{:#?}", v);

    // You can add your complex data by push method
    v.push(("sabiha naikwade".to_string(), 22));
    println!("{:?}", v);
}

pub fn run() {
    let mut hello = String::from("Hello ");

    hello.push('W');
    hello.push_str("orld");

    println!("Length: {}", hello.len());
    println!("byte capacity: {}", hello.capacity());

    println!("is empity:: {}", hello.is_empty());

    println!("contain: {}", hello.contains("World"));

    println!("replace: {}", hello.replace("World", "There"));

    for word in hello.split_whitespace() {
        print!("/{}", word);
    }
    println!("{}", hello);
    println!("\n");

    //

    let mut s = String::with_capacity(10);

    s.push('a');
    s.push('a');
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
    println!("{}", s)
}

pub fn run() {
    let x = 1;
    let y = 2.5;
    let z: i64 = 12312312312;
    let l: bool = true;
    let j = 10 > 5;
    let h = if false { 1 } else { 2 };

    // findmax
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    println!("{:?}", (x, y, z, l, j, h))
}

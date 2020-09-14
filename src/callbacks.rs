fn add(value: i32) -> i32 {
    return value + 1;
}
fn teste(value: i32, fun: &dyn Fn(i32) -> i32) -> i32 {
    return fun(value + 1);
}
pub fn run() {
    let t: i32 = teste(1, &add);
    print!("{}\n", t);
}

use std::env;
pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = args[2].clone();

    // println!("{:?}", command)
    if command == "oi" {
        println!("Oi {} ", name)
    }
}

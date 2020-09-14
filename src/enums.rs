enum Movement {
    Up,
    Down,
    Left,
    Right,
}
fn move_avatar(m: Movement) {
    match m {
        Movement::Up => println!("Avatar Up"),
        Movement::Down => println!("Avatar Down"),
        Movement::Left => println!("Avatar Left"),
        Movement::Right => println!("Avatar Right"),
    }
}

pub fn run() {
    let avatar1 = Movement::Up;
    let avatar2 = Movement::Down;
    let avatar3 = Movement::Left;
    let avatar4 = Movement::Right;
    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}

// mod types;
// mod string;
// mod arrays;
// mod structs;
// mod enums;
// mod cli;
// mod tuples;
#[path = "services/database.rs"]
mod database;

fn main() {
    // types::run();
    // arrays::run();
    // structs::run();
    // enums::run();
    // cli::run();
    database::run();
}

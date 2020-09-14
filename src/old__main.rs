// #![feature(proc_macro_hygiene, decl_macro)]

// use rocket::*;

// #[get("/")]
// fn index() -> String {
//     return "Hello, world!";
// }
// #[post("/user", format = "application/json", data = "<user>")]
// fn user(user: User) -> String {
//     print!("{}", name);
//     format!("Hello, {}!", name.as_str())
// }

// fn main() {
//     rocket::ignite().mount("/", routes![index, user]).launch();
// }

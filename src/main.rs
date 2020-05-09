use std::env;
mod morse;

fn main() {
    let mut args_iter = env::args();
    args_iter.next();

    let query = args_iter
        .collect::<Vec<String>>()
        .join(" ")
        .replace("_", "-");

    if morse::is_morse(&query) {
        println!("{}", morse::translate_morse(&query));
    } else {
        println!("{}", morse::translate_latin(&query));
    }
}

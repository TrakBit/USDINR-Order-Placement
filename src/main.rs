use std::env;

fn main() {
    match env::var("CLIENT_ID") {
        Ok(lang) => println!("Language code: {}", lang),
        Err(e) => println!("Couldn't read LANG ({})", e),
    };
}

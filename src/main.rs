#![allow(dead_code, unused_imports, unused_variables, unreachable_code)]

mod keyboard;
mod peripherals;

fn main() {
    match keyboard::Key::try_from('A') {
        Ok(key) => loop {
            println!("{}", key.to_json())
        },
        Err(err) => println!("{}", err.to_json()),
    }
}

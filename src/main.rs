#![allow(dead_code, unused_imports, unused_variables, unreachable_code)]

mod peripherals;
mod keyboard;

fn main() {
    loop {
        println!("{}", keyboard::Key::try_from('a').unwrap().to_json())
    }
}

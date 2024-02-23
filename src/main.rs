mod meow;
use meow::main_functions;
fn main() {
    println!("Hello, world!");
    let a = main_functions::getInput();
    println!("{}", a);
}
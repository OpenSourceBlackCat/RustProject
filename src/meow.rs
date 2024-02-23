pub mod main_functions{
    use std::io;
    pub fn getInput()->String{
        let mut input:String = String::new();
        io::stdin().read_line(&mut input);
        return input
    }
}
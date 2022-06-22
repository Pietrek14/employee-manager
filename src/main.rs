use std::collections::HashMap;
use std::io;
use employee_manager::sentence;
use employee_manager::company::Company;

fn main() {
    let mut company: Company = HashMap::new();

    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Couldn't read from stdin!");

        input = input.trim().to_string();

        if input.is_empty() {
            continue;
        }

        let sentence = input.split(' ').collect::<Vec<&str>>();

        sentence::execute(sentence, &mut company);
    }
}

//! Using a hash map and vectors, create a text interface to allow a
//! user to add employee names to a department in a company.
//! For example, “Add Sally to Engineering” or “Add Amir to Sales.”
//! Then let the user retrieve a list of all people in a department or
//! all people in the company by department, sorted alphabetically.

use std::collections::HashMap;
use std::io;

fn main() {
    let mut employees = HashMap::new();

    println!("Employee program!");
    println!("You can use `{{add/remove}} {{employee}} to {{department}}` to add employees.");
    println!("Write `quit` to exit the program.");

    loop {
        println!("Please introduce your command.");
        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line.");

        if command.trim() == "quit" {
            break;
        }

        let coms: Vec<String> = command.split(" ").map(|s| s.to_string()).collect();

        let cmd = &coms[0];
        let employee = &coms[1];
        let department = &coms[3];

        if cmd == "add" {
            employees.insert(department, employee);
        } else {
            employees.insert(department, employee);
        }

        println!("{}", coms[0]);
    }
}

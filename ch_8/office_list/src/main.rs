// this first iteration is a simple fetch via command line of read-only data
// that is pre-stored in a database file. Down the road it will be 
// revisited (maybe) to become a full CRUD application.

use std::collections::HashMap;

fn main() {
    let mut employees = HashMap::new();

    employees.insert("boromir", "sales");
    employees.insert("frodo", "engineering");
    employees.insert("legolas", "sales");
    employees.insert("merry", "engineering");
    employees.insert("pippen", "hr");
    employees.insert("sam", "hr");
    employees.insert("gandalf", "wizardry");
    employees.insert("aragorn", "sales");
    employees.insert("gollum", "it");
    employees.insert("arwen", "engineering");
    employees.insert("gimli", "it");

    println!("{:?}", employees);
}
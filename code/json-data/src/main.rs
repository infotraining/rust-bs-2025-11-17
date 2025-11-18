use std::fs::File;
use json_data::person::Person;

use json_data::person::create_people;

fn save_to_json_file(people: &Vec<Person>, path: &str) {
    todo!();
}

fn load_from_json_file(path: &str) -> Vec<Person> {
    todo!();
}

fn main() {
    let people = create_people();
    
    let file_path = "people.json";
}
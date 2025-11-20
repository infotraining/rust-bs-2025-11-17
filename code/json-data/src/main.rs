use std::fs::File;
use json_data::person::Person;

use json_data::person::create_people;

/// Save a vector of Person structs to a JSON file
/// # Arguments
/// * `people` - A reference to a vector of Person structs
/// * `path` - A string slice that holds the path to the JSON file  
fn save_to_json_file(people: &Vec<Person>, path: &str) {
    // Serialize the vector of Person structs to JSON and save to a file
    let file = File::create(path).expect("Unable to create file");
    serde_json::to_writer_pretty(file, people).expect("Unable to write data");
}

/// Load JSON data from a file and deserialize it into a vector of Person structs
/// # Arguments
/// * `path` - A string slice that holds the path to the JSON file
/// # Returns
/// * A vector of Person structs
fn load_from_json_file(path: &str) -> Vec<Person> {
    let file = File::open(path).expect("Unable to open file");
    serde_json::from_reader(file).expect("Unable to read data")
}

fn main() {
    let people = create_people();
    
    let file_path = "people.json";
    //save_to_json_file(&people, file_path);

    let loaded_people = load_from_json_file(file_path);
    for person in loaded_people {
        println!("{:?}", person);
    }
}
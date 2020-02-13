use std;
use std::collections::HashMap;

// TASK: Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. 
// For example, “Add Sally to Engineering” or “Add Amir to Sales.” 
// Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

pub fn run() {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Enter command:");

        let mut line = String::new();
        std::io::stdin().read_line(&mut line)
            .expect("Cannot read line");
        let command = parse_line(&line);

        match command {
            Command::AddPerson(model) => {
                cmd_add_person(&model, &mut map);
                println!("added!");
            }
            Command::List => {
                cmd_list(&map);
            }
            Command::Invalid => {
                println!("invalid command!");
            }
        }
    }
}

fn parse_line(line: &str) -> Command {
    let mut name_parts = Vec::new();
    let mut department_parts = Vec::new();
    let mut state = 0;

    for word in line.split_whitespace() {
        match state {
            // add
            0 => match word {
                "list" => {
                    return Command::List;
                }
                "add" => {
                    state += 1;
                }
                _ => return Command::Invalid
            }
            // name
            1 => {
                if word != "to" {
                    name_parts.push(word);
                } else {
                    state += 1;
                }
            }
            // department
            2 => {
                department_parts.push(word);
            }
            _ => return Command::Invalid
        };
    }

    if state == 2 && !name_parts.is_empty() && !department_parts.is_empty() {
        Command::AddPerson(AddPersonModel { 
            name: String::from(name_parts.join(" ")), 
            department: String::from(department_parts.join(" ")) 
        })
    } else {
        Command::Invalid
    }
}

fn cmd_add_person(model: &AddPersonModel, map: &mut HashMap<String, Vec<String>>) {
    let entry = map.entry(model.department.clone()).or_insert(Vec::new());
    entry.push(model.name.clone());
}

fn cmd_list(map: &HashMap<String, Vec<String>>) {
    for (dep, list) in map {
        println!("Department '{}':", dep);
        for person in list {
            println!("  - {}", person);
        }
    }
    println!();
}

struct AddPersonModel { name: String, department: String }

enum Command {
    AddPerson(AddPersonModel),
    List,
    Invalid,
}
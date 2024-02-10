extern crate serde; // 1.0.68
extern crate serde_derive; // 1.0.68
use serde::*;
use serde_with::serde_as; // 1.5.1
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
struct Todo {
    title: String,
    completed: bool,
}
#[serde_as]
#[derive(Deserialize, Serialize)]
struct Todos {
    #[serde_as(as = "Vec<(_, _)>")]
    todos: HashMap<String, Todo>,
}

impl Display for Todos {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.todos)
    }
}

// impl Serialize for Todos {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: Serializer,
//     {
//         let mut seq = serializer.serialize_map(Some(self.todos.len()))?;
//         for (k, v) in &self.todos {
//             seq.serialize_entry(&k.to_string(), &v)?;
//         }
//         seq.end()
//     }
// }

fn main() {
    let mut p = Todos {
        todos: HashMap::new(),
    };
    p.todos.insert(
        "one".to_string(),
        Todo {
            title: "mytitle1".to_string(),
            completed: false,
        },
    );
    p.todos.insert(
        "two".to_string(),
        Todo {
            title: "mytitle2".to_string(),
            completed: true,
        },
    );
    p.todos.insert(
        "three".to_string(),
        Todo {
            title: "mytitle3".to_string(),
            completed: false,
        },
    );
    p.todos.insert(
        "four".to_string(),
        Todo {
            title: "mytitle4".to_string(),
            completed: false,
        },
    );
    println!("WRITING TO FILE...");
    let serialized = serde_json::to_string(&p).unwrap();
    fs::write("src/todos.json", serialized.clone()).expect("Unable to write file");
    println!("{:#?}", serialized);
    println!("READING FROM FILE...");
    let content =
        fs::read_to_string("src/todos.json").expect("Should have been able to read the file");
    let deserialized: Todos = serde_json::from_str(&content).unwrap();
    println!("{:#?}", deserialized.todos)
}

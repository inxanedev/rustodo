use rusqlite::Connection;

use crate::{utils::{clear, get_input}, todo::Todo};

pub fn view_menu(db: &Connection) {
    loop {
        clear();
        let mut stmt = db.prepare("SELECT * FROM todo;").unwrap();
        let todos: Vec<Todo> = stmt.query_map([], |row| {
            Ok(Todo {
                id: row.get(0).unwrap(),
                caption: row.get(1).unwrap(),
                description: row.get(2).unwrap()
            })
        }).unwrap()
            .collect::<Vec<rusqlite::Result<Todo>>>()
            .into_iter()
            .map(|todo| todo.unwrap())
            .collect();

        if todos.len() == 0 {
            clear();
            println!("There are no todos yet!");
            println!("Create one in the 'Create new todo' menu.");
            get_input();
            break;
        }

        for (i, todo) in todos.iter().enumerate() {
            println!("ID: {} - {}", i + 1, todo.caption);
        }

        println!("\nSelect one to view the details, or press enter to go back");
        
        let action = get_input();
        
        if !action.is_empty() {
            let id: u32 = match action.parse() {
                Ok(id) => id,
                Err(_) => {
                    clear();
                    println!("Invalid id passed! It has to be a number.");
                    return;
                }
            };

            let selected_todo = match todos.get((id - 1) as usize) {
                Some(todo) => todo,
                None => {
                    clear();
                    println!("No todo with the specified id exists.");
                    get_input();
                    return;
                }
            };

            clear();

            println!("Todo with ID {}", id);
            println!("{}", selected_todo.caption);
            if selected_todo.description.is_empty() {
                println!("Description: No additional information");
            } else {
                println!("Description: {}", selected_todo.description);
            }
            println!("\nSelect action:");
            println!("1 - Remove this todo");
            println!("2 - Go back");

            let action = get_input();

            if action == "1" {
                db.execute("DELETE FROM todo WHERE id = ?1", [selected_todo.id]).unwrap();
            }
        } else {
            break;
        }
    }
}
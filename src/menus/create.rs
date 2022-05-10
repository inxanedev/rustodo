use rusqlite::Connection;

use crate::utils::{clear, get_input};

macro_rules! exit_check {
    ($x:expr) => {
        if $x == "exit" {
            return;
        }
    };
}

pub fn create_menu(db: &Connection) {
    clear();

    println!("Creating a new todo!");
    println!("If at any point you'd like to quit, type 'exit' and hit enter.");
    println!("Fill in the information:");

    print!("Todo caption: ");
    let caption = get_input();
    exit_check!(caption);
    if caption.is_empty() {
        clear();
        println!("A valid todo must have a caption, but it doesn't need to have a description.");
        println!("Please try again, and add at least a caption.");
        get_input();
        return;
    }

    println!("Todo details/description:");
    let description = get_input();
    exit_check!(description);

    db.execute("INSERT INTO todo (caption, description) VALUES (?1, ?2);", [caption, description]).unwrap();

    clear();
    println!("Todo created! See it in the 'View todos' menu.");
    get_input();
}
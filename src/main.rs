use rusqlite::Connection;

mod menus;
mod utils;
mod todo;

fn main() {
    let conn = Connection::open("./data.db")
        .expect("Couldn't open the database!");
    
    conn.execute("CREATE TABLE IF NOT EXISTS `todo` (id INTEGER NOT NULL PRIMARY KEY, caption TEXT, description TEXT);",
        []).unwrap();

    loop {
        utils::clear();

        println!("Welcome to RusTodo!");
        println!("1 - View todos");
        println!("2 - Create new todo");
        println!("3 - Exit");

        let action = utils::get_input();
        
        match action.as_str() {
            "1" => menus::view_menu(&conn),
            "2" => menus::create_menu(&conn),
            _ => {
                std::process::exit(0);
            }
        }
    }

}

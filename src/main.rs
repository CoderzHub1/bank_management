use bank::{
    program::main_loop,
    structs::account::Account,
    task_handlers::{new_account::new_acc, prompt::prompt},
};
use sled::Db;

fn main() {
    // Initialisation
    let mut _accessed_account: Option<Account> = None;
    let db: Db = sled::open("db").expect("Cant access the path db");
    let accounts_db: sled::Tree = db
        .open_tree("accounts")
        .expect("Error: Failed to open accounts tree");
    // Main application loop

    loop {
        let command: String = prompt("Enter your command: ").unwrap();
        match command.as_str() {
            "exit" => {
                break;
            }
            _ => {
                let result = main_loop(command.as_str(), &mut _accessed_account, &accounts_db);
                match result {
                    Ok(_) => {
                        // Do noything
                    }
                    Err(x) => {
                        eprintln!("Error: {}", x);
                    }
                }
            }
        }
    }

    accounts_db.flush().expect("Cant flush the database");
}

use bank::{
    structs::account::Account,
    task_handlers::{new_account::new_acc, prompt::prompt},
    test,
};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialisation
    let mut _accessed_account: Option<Account> = None;
    let accounts_db: Db = sled::open("db/accounts")?;

    // Main application loop
    
    loop {
        let command: String = prompt("Enter your command: ")?;
        match command.as_str() {
            "new" => {
                let username: String = prompt("Enter your username: ")?;
                let password: String = prompt("Enter your password: ")?;

                _accessed_account = new_acc(accounts_db.username, password, &accounts_db);
            }

            "print" => match &_accessed_account {
                Some(acc) => {
                    println!("{:#?}", acc);
                }
                None => {
                    println!("No account accessed");
                }
            },

            _ => {
                println!("Command cant be indentified!");
            }

            "test" => {
                test::test();
            }

            "exit" => {
                accounts_db.flush()?;
                return Ok(());
            }
        }
    }
    accounts_db.flush()?;
}

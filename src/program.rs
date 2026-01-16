use sled::{Tree};
use std::error::Error;
use crate::{structs::account::Account, task_handlers::{new_account::new_acc, prompt::prompt}};

pub fn main_loop(command: &str, _accessed_account:&mut Option<Account>, accounts_db:&Tree) -> Result<(), Box<dyn Error>> {
        match command {
            "new" => {
                let username: String = prompt("Enter your username: ")?;
                let password: String = prompt("Enter your password: ")?;

                *_accessed_account = Some(new_acc(username, password, &accounts_db)?);
                Ok(())
            }

            "print" => match &_accessed_account {
                Some(acc) => {
                    println!("{:#?}", acc);
                    Ok(())
                }
                None => {
                    println!("No account accessed");
                    Ok(())
                }
            },
            
            _ => {
                println!("Command cant be indentified!");
                Ok(())
            }

        }
}

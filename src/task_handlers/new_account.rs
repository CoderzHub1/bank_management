use std::error::Error;
use crate::{Errors::AccountCreationErrors, structs::account::Account};
use sled::Tree;

pub fn new_acc(username: String, password: String, db: &Tree) -> Result<Account, Box<dyn Error>> {
    let mut id: u32 = 0;
    let last_acc_opt= db.last().ok();
    match last_acc_opt {
        Some(last_acc)=>{
            if let Some(x) = last_acc {
                let last_account: Account = bincode::deserialize(&x.1)?;
                id = last_account.id
            }
        }
        None=>{
            return Err(AccountCreationErrors::AccountCreationErrors::InternalError().into());
        }
    }


    let account = Account {
        id: id,
        username,
        password,
        balance: 0.0,
        status: true,
    };


    match db.get(&account.username) {
        Ok(_x) => {
            return Err(AccountCreationErrors::AccountCreationErrors::AlreadyExists().into());
        }
        Err(_x) => {
            let encoded = bincode::serialize(&account)?;
            db.insert(&account.username, encoded)?;

            return Ok(account);
        }
    }
    
}

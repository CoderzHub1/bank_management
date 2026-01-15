use sled::Db;
use crate::structs::account::Account;

// To add: verification if the account is unique
pub fn new_acc(username: String, password:String, db: &Db)->Option<Account>{
    let last_acc: Option<(sled::IVec, sled::IVec)> = db.last().ok()?;
    let mut id: u32 = 0;
    if let Some(x) = last_acc{
       let last_account:Account = bincode::deserialize(&x.1).ok()?;
       id = last_account.id
    }

    let account = Account{
        id:id,
        username,
        password,
        balance: 0.0,
        status: true
    };

    let encoded = bincode::serialize(&account).ok()?;
    db.insert(&account.username, encoded).ok()?;

    Some(account)
}
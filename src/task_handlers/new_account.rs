use crate::structs::account::Account;


// To add: verification if the account is unique
pub fn new_acc(username: String, password:String)->Account{
    Account{
        username,
        password,
        balance: 0.0,
    }
}
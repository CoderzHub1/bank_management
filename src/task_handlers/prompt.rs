use std::{error::Error, io::{Write, stdin, stdout}};

pub fn prompt(msg:&str)->Result<String, Box<dyn Error>>{
    print!("{}", msg);
    stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
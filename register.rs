use rand;
use rand::distributions::{Distribution,Alphanumeric};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::{Write};
use serde::{Deserialize, Serialize};




pub fn pass_gen(){

    let mut rng = rand::thread_rng();
    {
        let pass: String =Alphanumeric
            .sample_iter(&mut rng)
            .take(30)
            .map(char::from)
            .collect();

        println!("Your password is: {}", pass)
    }
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Account {
    pub account: String,
    pub pass: String,
}

pub fn read_file(path: &str) -> String{
    let path = Path::new(path);
    let display= path.display();
    let mut file = match File::open(&path) {
        Err(e) => panic!("Couldn't open {}: {}",display, e),
        Ok(file) => file,
    };
//  Read file
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(e) => panic!("Couldn't read {}:{}",display,e),
        Ok(_) => return s,
    }
    
}

pub fn write_file(path: &str,account:&str,pass:&str) {
    while account != "q"
    {
        let string = read_file(path);
        let mut accounts =parse_data(string);
        let a = Account{account:account.to_string(),pass:pass.to_string()};
        accounts.push(a);
        
        let mut string = String::new();
        for account in accounts.iter() {
            let json_str = serde_json::to_string(&account).unwrap();
            string.push_str(&json_str);
            string.push(';');
        }
        string.pop();
        let path = Path::new(path);
        let display = path.display();

        // Open a file in write-only mode
        let mut file = match File::create(&path) {
            Err(e) => panic!("Couldn't create file {}:{}",display,e),
            Ok(file) => file,
        };
        match file.write_all(string.as_bytes()) {
            Err(e) => panic!("Couldn't write to {}:{}",display,e),
            Ok(_) =>println!("Successfully write into {}",display),
        };
    }
}

pub fn parse_data(json_str: String)->Vec<Account>{
    let mut accounts :Vec<Account> = Vec::new();
    let accounts_json: Vec<&str> = json_str.trim().split(';').collect();
    // let data = accounts_json.clone().into_boxed_slice();
    for account_json in accounts_json.into_iter() {
        let account = account_json.to_string();
        let data = account.clone().into_boxed_str();
        let u: Account = serde_json::from_str(&data).unwrap();
        accounts.push(u);
    };
    
    accounts
}

pub fn check_already_created(path:&str,account_name: &str) -> bool {
    let mut a = false;
    let string = read_file(path);
    let accounts = parse_data(string);
    for account in accounts.iter() {
        if account_name == account.account{
            a = true;
        }
    };
    a
}


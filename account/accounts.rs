use crate::account::{login,register};
use std::io;
pub const PATH_FILE: &str = "/home/anhminh/Rust/rust_present/present_2/src/account/account.txt";


pub fn interface(){
    println!("==================== Manager Panel ====================");
    println!("|                                                     |");
    println!("|    1. Register: Press 1                             |");
    println!("|    2. Login: Press 2                                |");
    println!("|                                                     |");
    println!("=======================================================\n");

    loop{
        println!(" Please enter your choice");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading");
        let input: u8 = match input.trim().parse() {
            Ok(s) => s,
            Err(_) => continue,
        };
        match input{
            1 => {
                    register();
                    break;
                },
            2 => {
                    login();
                    break;
                },
            _ => {
                    println!("\n Choice not match");
                    continue;},
        }
    }
    
}
fn register(){
    println!("\nRegister:   ");
    println!("\n Please enter your username or you can press q to quit:");
    let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading");
    let input =  input.trim();
    if input == "q"{
        interface()
    }
    else if register::check_already_created(PATH_FILE,input) {
        println!("\nAccount has already been created");
        register();
    }
    else {
        let pass = register::pass_gen();
        register::write_file(PATH_FILE, input, &pass);

        println!("\nAccount has been created successfully");
        println!("Your password is {}, remember that",pass);
    }
}

fn login(){
    println!("\nLogin: ");
    println!("\n Please enter your username or you can press 'q' to quit:");
    let mut input_1 = String::new();
        io::stdin()
            .read_line(&mut input_1)
            .expect("Error reading");
    let input_1 =  input_1.trim();
    if input_1=="q"{
        interface();
    } else if !register::check_already_created(PATH_FILE, input_1){
        println!("\n Your account hasn't been created yet");
        println!(" Do you want to create a new account?\n Press 'yes'/'no':");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading");
        let input =  input.trim();
        if input == "yes" {
            register();
        } else if input == "no" {
            interface();
        } else {
            println!("Invalid syntax");
            login();
        }
    } else {
        
        check_pass(input_1);
    }
}
fn check_pass(account:&str){
    println!(" Please enter your password or you can press 'q' to quit");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading");
    let input =  input.trim();
    if input == "q" {
        interface();
    } else if !login::check(account, input) {
        println!("\n Invalid password");
        check_pass(account);
    } else{
        
    }

}




use crate::account::register;
pub const PATH_FILE: &str = "/home/anhminh/Rust/rust_present/present_2/src/account/account.txt";


pub fn check(account:&str, pass:&str) -> bool {
    let mut bool = false;
    let string = register::read_file(PATH_FILE);
    let accounts = register::parse_data(string);
    for account_data in accounts.iter() {
        if (account== account_data.account) && (pass==account_data.pass){
            bool = true;
        }
    };
    bool
}

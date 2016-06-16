extern crate oauth2;
extern crate curl;

use std::fs::File;
use std::io::Read;
use curl::http;

fn main() {
    let mut f = File::open("creds.txt").unwrap();
    let mut acc_code = String::new();
    let _ = f.read_to_string(&mut acc_code);
    acc_code.pop();
    println!("access code is: {}", acc_code);
    let tok = oauth2::Token {
        access_token: acc_code,
        scopes: vec!["repo".to_owned()],
        token_type: "bearer".to_owned()
    };


    let resp = http::handle().get("https://www.rust-lang.org").exec().unwrap();
    let respString = std::string::String::from_utf8(resp.move_body()).unwrap();
    println!("response is: {}", respString);
}

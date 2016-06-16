//! Uses the oauth library to set get an fresh access token
extern crate oauth2;
extern crate rustc_serialize;

use std;
use rustc_serialize::json;
use std::fs::File;
use std::io::Read;

#[derive(RustcDecodable, RustcEncodable)]
struct Secret {
    client_id: String,
    client_secret: String,
    auth_url: String,
    token_url: String
}

#[allow(dead_code)]
pub fn printcode () {
    let mut f = File::open("secrets.json").unwrap();
    let mut read_str = String::new();
    let _ = f.read_to_string(&mut read_str);
    let sec : Secret = json::decode(&read_str).unwrap();

    let mut conf = oauth2::Config::new(
        &sec.client_id,
        &sec.client_secret,
        &sec.auth_url,
        &sec.token_url
    );
    conf.scopes = vec!["repo".to_owned()];
    let url = conf.authorize_url("v0.0.1 gitbot".to_owned());
    println!("please visit this url: {}", url);

    let mut user_code = String::new();
    let _ = std::io::stdin().read_line(&mut user_code).unwrap();
    user_code.pop();
    let tok = conf.exchange(user_code).unwrap();
    println!("access code is: {}", tok.access_token);
}

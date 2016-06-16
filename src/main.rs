//! Main spinup for gitbot

extern crate curl;
extern crate rustc_serialize;
mod codegen;

use std::fs::File;
use std::io::Read;
use curl::http;
use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
/// Represents a Bearer token from github that is saved to disk
struct Bearer {
  access_token: String,
  scopes: Vec<String>,
  token_type: String
}

fn main() {
  let mut f = File::open("creds.json").unwrap();
  let mut tok_json = String::new();
  let _ = f.read_to_string(&mut tok_json);
  let tok : Bearer = json::decode(&tok_json).unwrap();
  println!("access token is: {}", tok.access_token);

  let mut hand = http::handle();
  let mut req = hand.get("https://api.github.com/users/allonsy");
  req = add_auth_header(req, &tok);
  let resp = req.exec().unwrap();
  let resp_string = std::string::String::from_utf8(resp.move_body()).unwrap();
  println!("response is: {}", resp_string);
}

#[allow(dead_code)]
fn add_auth_header<'a,'b>(req : http::Request<'a,'b>, tok : &Bearer) -> http::Request<'a,'b> {
  req.header("Authorization",
             &format!("token {}", tok.access_token))
    .header("User-Agent", "gitbot v0.0.1 scraper")
}

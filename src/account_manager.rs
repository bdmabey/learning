use std::path::Path;

use base64ct::Base64;
use base64ct::Encoding;
use serde::{Serialize, Deserialize};
use sha2::Sha256;
use sha2::Digest;

use crate::chat::Input;
use crate::chat::{self, read_input};
use crate::question::Questions;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    hashed_password: String,
    pub question_data: Vec<Questions>
}

impl User {

    pub fn new() -> User {
        User {
            username: String::new(),
            hashed_password: String::new(),
            question_data: vec![]
        }
    }

    pub fn login(&mut self) -> Result<&mut User, &'static str> {
        //Asks for username
        println!("Please enter your username: ");
        let tmp_username = match read_input() {
            chat::Input::Input(username) => {
                match username.trim() {
                    _ => username
                }
            }
        };

        //Read through previous users and turns them from json to objects
        let path = Path::new("./users");
        let mut holder: Vec<String> = vec![];
        if path.is_dir() {
            for entry in std::fs::read_dir(path).unwrap() {
                match entry {
                    Ok(e) => {
                        let users = std::fs::read_to_string(e.path()).unwrap();
                        holder.push(users);
                    },
                    Err(e) => panic!("{e}"),
                }
            }
        }
        let mut user_results = vec![];
        for i in holder {
            let tmp = serde_json::from_str::<User>(&i).unwrap();
            user_results.push(tmp)
        }

        //searches for user and if found then goes on to ask for password
        for user in user_results {
            if tmp_username.trim() == user.username.trim() {
                self.username = user.username;
                self.hashed_password = user.hashed_password;
                println!("Please enter your password: ");
                let tmp_password = match chat::read_input() {
                    chat::Input::Input(text) => {
                        match text.trim() {
                            _ => {
                                let mut hasher = Sha256::new();
                                hasher.update(text);
                                let result = hasher.finalize();
                                let hashed_result = Base64::encode_string(&result);
                                print!("{esc}[2J{esc}[1;1H", esc = 27 as char); //How to clear terminal screen
                                hashed_result
                            }
                        }
                    }
                };
        
                if tmp_password.ne(&self.hashed_password) {
                    return Err("Wrong Password")
                }

                self.question_data = user.question_data;
                return Ok(self)
            }
        }

        Err("No Account Found")

    }

    pub fn create(&mut self) {
        println!("Please enter the username you would like to use: ");
        match read_input() {
            Input::Input(text) => {
                match text.trim() {
                    _ => {
                        println!("Your username is: {}", text);
                        self.username = String::from(text.trim());
                        println!("Please enter your password: ");
                        match read_input() {
                            Input::Input(text) => {
                                let tmp_password = text;
                                println!("Please re-enter your passowrd: ");
                                match read_input() {
                                    Input::Input(text) => {
                                        if text.eq(&tmp_password){
                                            let mut hasher = Sha256::new();
                                            hasher.update(text);
                                            let result = hasher.finalize();
                                            let hashed_result = Base64::encode_string(&result);
                                            self.hashed_password = hashed_result
                                        } else {
                                            println!("Passwords did not match. Taking you back.");
                                            self.create()
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn logout() {

}

fn check_passwords() {

}
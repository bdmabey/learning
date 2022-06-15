use crate::account_manager::User;
use crate::{account_manager};
use crate::question;

use core::panic;
use std::fs;
use std::{thread, time::Duration};

use chrono::prelude::*;

enum Choice {
    Load,
    Save,
    Quit
}

enum TimeOfDay {
    Morning(u8),
    Afternoon(u8),
    Night(u8)
}

/*
    Ready to implement the main loop.
    OR go figure out the create user
    save is good though
*/
fn main_loop(user: &mut User) {
    loop {
        break
    }
}

pub fn start() {
    greet();

    println!("Would you like to login to your account? y/n");
    match read_input() {
        Input::Input(text) => {
            match text.trim() {
                "y" | "Y" => {
                    let mut user = account_manager::User::new();
                    match user.login() {
                        Ok(_) => {
                            println!("welcome, {}", user.username);
                            question::load_questions(&mut user);
                            /*
                                At this point all questions are loaded and we can proceed to the main loop.
                            */
                            main_loop(&mut user)
                        },
                        Err(e) => {
                            match e {
                                "Wrong Password" => {
                                    println!("Wrong password. Try again");
                                    thread::sleep(Duration::from_millis(1000));
                                    start()
                                },
                                "No Account Found" => {
                                    println!("No account was found.");
                                    account_manager::create()
                                },
                                _ => println!("{e}")
                            }
                        }
                    };
                },
                "n" | "N" => println!("OK, now I do soemthing."),
                _ => {
                    println!("You entered something I don't know. Taking you back");
                    thread::sleep(Duration::from_millis(1000));
                    start()
                }
            }
        }
    }
}

fn greet() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    let time = match Local::now().format("%H").to_string().parse::<u8>() {
        Ok(n) => {
            if n < 12 {
                TimeOfDay::Morning(n)
            } else if n < 19 {
                TimeOfDay::Afternoon(n)
            } else {
                TimeOfDay::Night(n)
            }
        },
        Err(e) => panic!("Error: {e}"),
    };
    match time {
        TimeOfDay::Morning(_) => println!("Good Morning!"),
        TimeOfDay::Afternoon(_) => println!("Good Afternoon!"),
        TimeOfDay::Night(_) => println!("Good Evening!")
    }
}

pub enum Input {
    Input(String)
}

pub fn read_input() -> Input {
    let reader = std::io::stdin();
    let mut buffer: String = String::new();
    match reader.read_line(&mut buffer) {
        Ok(_) => return Input::Input(buffer),
        Err(e) => panic!("{e}")
    }
}

fn save(user: &User) {
    let ser = serde_json::to_string_pretty(&user).unwrap();
    let mut out_path = String::from("./users/.json");
    out_path.insert_str(8, &user.username);
    fs::write(out_path, &ser).unwrap();
}

fn quit() {
    todo!()
}
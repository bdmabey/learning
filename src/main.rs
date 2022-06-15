#![allow(dead_code)]

use chat::start;
use crossbeam::channel::unbounded;
use sha2::Digest;
use sha2::Sha256;
use base64ct::{Base64, Encoding};

mod account_manager;
mod question;
mod chat;

fn main() {
    //testing_threads();
    start()
    // question::load_questions();
    // testing_json(); 
    // reading_json()
}

fn testing() {
    let mut hasher = Sha256::new();
    hasher.update(b"test");
    let result = hasher.finalize();

    let new_result = Base64::encode_string(&result);

    println!("{}", new_result)
}

//Lets play around with input threads and output threads through crossbeam.
//This gonna get ugly.
//Will also change the whole structure of the porgram.
//Well this works quite well.


fn testing_threads() {
    let (snd, rcv) = unbounded();
    let (snd2, rcv2) = unbounded();
    crossbeam::scope(|s| {
        s.spawn(|_| loop {
            let _ = match rcv2.recv().unwrap() {
                "break" => break,
                _ => (),
            };
            use std::io;
            let reader = io::stdin();
            let mut buffer: String = String::new();
            reader.read_line(&mut buffer).ok().expect("Failed to read line.");
            let send = buffer.clone();
            snd.send(send).ok();
        });

        s.spawn(|_| {
            snd2.send("Ready for input").ok();
            loop {
                if let Ok(action) = rcv.try_recv() {
                    match action.trim() {
                        "help" | "Help" => println!("Help is on the way."),
                        "quit" => {
                            snd2.send("break").ok();
                            break
                        },
                        _ => {
                            snd2.send("break").ok();
                            break
                        }
                    }
                    dbg!("msg sent to get input");
                    snd2.send("Ready for input").ok();
                }
            }   
        });
    }).unwrap();
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Fruits {
    name: String,
    apple: u8,
    oranges: u8
}

    // fn testing_json() {
    //     use std::{env, fs};
    //     use std::fs::File;

    //     let one = Fruits { name: String::from("HEather"), apple: 8, oranges: 10};
    //     let two = Fruits { name: String::from("John"), apple: 98, oranges: 109};
    //     let mut vec_fruits: Vec<Fruits> = Vec::new();

    //     vec_fruits.push(one);
    //     vec_fruits.push(two);

    //     let ser = serde_json::to_string_pretty(&vec_fruits).unwrap();

    //     std::fs::write("test.json", ser).unwrap();

    // }


    // fn reading_json() {
    //     use std::{fs, env};

    //     let text = std::fs::read_to_string("test.json").unwrap();
    //     let result = serde_json::from_str::<Vec<Fruits>>(&text).unwrap();

    //     println!("{:?}", result)
    // }
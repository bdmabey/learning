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
    start()
}

fn testing() {
    let mut hasher = Sha256::new();
    hasher.update(b"test");
    let result = hasher.finalize();

    let new_result = Base64::encode_string(&result);

    println!("{}", new_result)
}

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
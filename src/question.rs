use serde::{Serialize, Deserialize};
use std::fs;

use crate::account_manager::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Questions {
    pub question: String,
    pub answer: String,
    pub asked: bool
}

pub fn load_questions(user: &mut User) {
    let questions = fs::read_to_string("questions.txt").unwrap();
    let vec_questions: Vec<&str> = questions.split("\r\n").collect();

    let mut counter = 0;
    for _i in &user.question_data {
        counter = counter + 1;
    }

    if counter == vec_questions.len() {
        println!("no new questions")
    } else {
        for i in counter..vec_questions.len() {
            user.question_data.push(Questions { question: String::from(vec_questions[i]), answer: String::from(""), asked: false })
        }
    }

}
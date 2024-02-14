mod quiz_app;

use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;
use eframe::NativeOptions;
use serde_derive::Deserialize;
use crate::quiz_app::QuizApp;


#[derive(Debug, Deserialize)]
struct Data {
    questions: HashMap<String, String>,
}
fn load_data() {
    let file_path = "questions/questions.toml";

    let mut file_handle = File::open(file_path).expect("Could not open file");

    let mut buf: String = String::new();

    file_handle.read_to_string(&mut buf).expect("Failed to read into string");

    let data: Data = toml::from_str(&*buf).expect("Could not fit file into format");

    //  println!("{:#?}", data);
    //  for (question, _) in &data.questions {
    //      println!("{}", question);
    //      let mut userinput = String::new();
    //      io::stdin().read_line(&mut userinput).unwrap();
    //  }
    // println!("anwsers:");
    // for (_, answer) in &data.questions {
    //     println!("{}", answer);
    // }
    let mut score = 0;
    for (question, answer) in &data.questions {
        println!("{}", question);
        let mut userinput = String::new();
        io::stdin().read_line(&mut userinput).unwrap();
        let trimmedinput = userinput.trim();
        if trimmedinput == answer {
            println!("correct");
            score += 1;
        } else { print!("Inccorect! the answer is: {}", answer); }
    }
    println!("Score: {}/{}", score, data.questions.len());
}
fn main() -> eframe::Result<()> {
    let options = NativeOptions::default();

    eframe::run_native(
        "Jackfuck",
            options,
            Box::new(|cc|{
                Box::new(QuizApp::default())
            })
    )
}


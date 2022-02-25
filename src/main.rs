use std::sync::RwLock;
use my_wordle_server::is_valid_word;

use rocket::State;
#[macro_use] extern crate rocket;

#[get("/validate_word/<word>")]
async fn validate_word(word: &str) -> &'static str {
    let validate =  is_valid_word(word).await.expect("error in validating process!");
    match validate {
        true => "true", 
        false => "false",
    }
}

#[get("/set_hidden_word/<new_word>")]
async fn set_hidden_word(new_word: &str, hidden_word: &State<RwLock<String>>) -> &'static str {
    let validate =  is_valid_word(new_word).await.expect("error in validating process!");
    if validate == false {
        return "false";
    }

    {
        let mut w = hidden_word.write().unwrap();
        *w = new_word.to_string();
    }

    "true"
}

#[get("/score_word/<word>")]
async fn score_word(word: &str, hidden_word: &State<RwLock<String>>) -> String {
    let hidden_word = hidden_word.read().unwrap();
    let res = word
        .chars()
        .zip(hidden_word.chars())
        .map(|(x, y)| {
            if x == y {
                return 'G';
            }
            else if hidden_word.chars().any(|z| z == x) {
                return 'Y';
            }
            else {
                return 'W';
            }
        })
        .collect::<String>()
    ;
    
    res
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![validate_word])
        .mount("/", routes![set_hidden_word])
        .mount("/", routes![score_word])
        .manage(RwLock::new("ultra".to_string()))
}



// #[tokio::main]
// async fn main(){
//     let mut word = String::new();
//     io::stdin().read_line(&mut word).unwrap();
//     let word = word.trim();
//     println!("{}", word);
//     let x = is_valid_word(&word).await.expect("hava a problem while validating word currectness.");
//     println!("{}", x);
// }

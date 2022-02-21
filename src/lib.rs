use reqwest;
use std::error::Error;


/// validate word-correctness.
pub async fn is_valid_word(word: &str) -> Result<bool, Box<dyn Error>> {
    if word.len() != 5 {
        return Ok(false);
    }

    let url = format!("{}{}", "https://api.dictionaryapi.dev/api/v2/entries/en/", word);
    let res = reqwest::get(url)
        .await?
        .status()
    ;

    match res.as_u16() {
        200 => Ok(true), 
        _ => Ok(false)
    }
}
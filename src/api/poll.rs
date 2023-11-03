use reqwest::Client;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Poll {
    pub title: String,
    pub description: String,
    pub choices: Vec<String>
}

impl Default for Poll {
    fn default() -> Self {
        Self {
            title: "Best Programming Paradigm".to_string(),
            description: "What is the best paradigm?".to_string(),
            choices: vec![
                "OOP".to_string(),
                "FOP".to_string(),
                "ChatGPT".to_string()
                ]
        }
    }
}


pub async fn get_poll(client: &Client, id: usize) -> Result<Poll, reqwest::Error> {
    Ok(
        Poll::default()
    )
}
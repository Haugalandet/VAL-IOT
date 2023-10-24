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
            title: "Title".to_string(),
            description: "Description\nDescription\nDeez Nutz".to_string(),
            choices: vec![
                "A".to_string(),
                "B".to_string(),
                "C".to_string(),
                "D".to_string(),
                "E".to_string(),
                "F".to_string()
                ]
        }
    }
}


pub async fn get_poll(client: &Client, id: usize) -> Result<Poll, reqwest::Error> {
    Ok(
        Poll::default()
    )
}
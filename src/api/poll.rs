use bevy::prelude::default;
use reqwest::Client;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Poll {
    pub pollId: Option<usize>,
    pub title: String,
    pub description: String,
    pub choices: Vec<Choice>
}

#[derive(Serialize, Deserialize, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Choice {
    pub choiceId: Option<usize>,
    pub title: String,
    pub description: String,
    pub count: Option<usize>
}

impl Choice {
    pub fn new(title: String) -> Choice {
        Choice {
            choiceId: None,
            title,
            ..default()
        }
    }
}


impl Default for Poll {
    fn default() -> Self {
        Self {
            pollId: None,
            title: "Best Programming Paradigm".to_string(),
            description: "What is the best paradigm?".to_string(),
            choices: vec![
                Choice::new("OOP".to_string()),
                Choice::new("FOP".to_string()),
                Choice::new("ChatGPT".to_string())
                ],
        }
    }
}


pub async fn get_poll(client: &Client, id: usize) -> Result<Poll, reqwest::Error> {
    Ok(
        Poll::default()
    )
}
use reqwest::Client;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Joke {
    pub error: bool,
    pub category: String,
    pub setup: String,
    pub delivery: String,
}
impl Joke {
    pub async fn get_joke() -> Result<Joke, Error> {
        let url = "https://v2.jokeapi.dev/joke/Any?type=twopart";
        let client = Client::new();
        let response = client.get(url).send().await?.json().await?;
        Ok(response)
    }
}
#[derive(Debug, Clone)]
pub enum Error {
    FetchFailed,
    BadJoke,
}
impl From<reqwest::Error> for Error {
    fn from(_value: reqwest::Error) -> Self {
        Self::FetchFailed
    }
}

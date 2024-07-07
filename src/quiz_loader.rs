
use crate::quiz::Quiz;
use std::sync::mpsc::channel;



pub struct QuizLoader;


impl QuizLoader {


    pub async fn fetch_async(url: &str) -> Result<Quiz, String> {
        reqwest::get(url)
            .await
            .map_err(|e| e.to_string())?
            .json()
            .await
            .map_err(|e| e.to_string())
    }

}
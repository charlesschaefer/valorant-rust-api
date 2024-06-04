use reqwest::{Client, Error};
use std::env;

pub struct ValorantAPI {
    api_key: String,
    base_url: String
}

impl ValorantAPI {
    pub fn new() -> ValorantAPI {
        // gets valorant api key from environment variable
        let valorant_api = env::var("VALORANT_API_KEY").unwrap();
        ValorantAPI {
            api_key: valorant_api,
            base_url: String::from("https://br.api.riotgames.com/"),
        }
    }

    pub async fn request(&self, method: String) -> Result<String, Error> {
        let mut url = self.base_url.clone();
        url.push_str(method.as_str());
        //url.push_str("?api_key=");
        //url.push_str(&self.api_key.as_str());

        let auth_token: String = format!("{}", self.api_key);

        print!("Auth:  {}, url: {}", auth_token, url);

        let client = Client::new();
        let res = client.get(url)
            .header("X-Riot-Token", auth_token)
            .send()
            .await?
            .text()
            .await?;
    
        Ok(res)
    }

    pub async fn status(&self) -> String {
        let method = "val/status/v1/platform-data".to_string();
        let response = self.request(method).await;

        response.unwrap()
    }

    pub async fn content(&self) -> String {
        let method = "val/content/v1/contents?locale=pt-BR".to_string();
        let response = self.request(method).await;

        response.unwrap()
    }

    pub async fn ranked_by_act(&self, act_id: String) -> String {
        let mut method = "val/ranked/v1/leaderboards/by-act/".to_string();
        method.push_str(&act_id.as_str());

        let response = self.request(method).await;

        response.unwrap()
    }

    pub async fn get_match(&self, match_id: String) -> String {
        let mut method = "val/match/v1/matches/".to_string();
        method.push_str(&match_id.as_str());

        let response = self.request(method).await;

        response.unwrap()
    }

    pub async fn matches_from_user(&self, user_id: String) -> String {
        let mut method = "val/match/v1/matchlists/by-puuid/".to_string();
        method.push_str(&user_id.as_str());

        let response = self.request(method).await;

        response.unwrap()
    }

    pub async fn matches_by_queue(&self, queue_id: String) -> String {
        let mut method = "val/match/v1/recent-matches/by-queue/".to_string();
        method.push_str(&queue_id.as_str());

        let response = self.request(method).await;

        response.unwrap()
    }
}

use std::{error::Error, string::String};

use reqwest::Client;
use serde_json::{json, Value};

pub struct Openai {
    client: Client,
    host: String,
}

impl Openai {
    pub fn new(host: String, api_key: String) -> Result<Self, Box<dyn Error>> {
        let client = reqwest::Client::builder()
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    "Authorization",
                    format!("Bearer {}", api_key).parse().unwrap(),
                );
                headers.insert("Content-Type", "application/json".parse().unwrap());
                headers
            })
            .build()?;

        Ok(Self { client, host })
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.host, path)
    }

    pub async fn fetch_model_ids(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let resp = self
            .client
            .get(self.url("/v1/models"))
            .send()
            .await?
            .text()
            .await?;

        let json_value: Value = serde_json::from_str(&resp)?;

        let model_list: Vec<String> = json_value["data"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|item| item.get("id").and_then(Value::as_str))
            .map(String::from)
            .collect();

        Ok(model_list)
    }

    pub async fn fetch_response(
        &self,
        model: &String,
        input: &String,
    ) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .post(self.url("/v1/responses"))
            .json(&json!({
                "model": model,
                "input": input
            }))
            .send()
            .await?
            .json::<Value>()
            .await?;

        let output = response["output"][1]
            .as_object()
            .or_else(|| response["output"][0].as_object())
            .expect("unexpected response structure");

        let text = output["content"][0]["text"]
            .as_str()
            .expect("unexpected response structure");

        Ok(text.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[test]
    fn test_url() {
        let api = Openai::new("https://api.openai.com".to_string(), "123".to_string());
        assert!(api.is_ok());
        assert_eq!(
            api.unwrap().url("/v1/models"),
            "https://api.openai.com/v1/models"
        )
    }

    #[tokio::test]
    async fn test_fetch_model_ids() {
        let mut server = Server::new_async().await;
        let model_ids = vec!["text-davinci-003".to_string(), "gpt-3.5-turbo".to_string()];

        let _mock = server
            .mock("GET", "/v1/models")
            .with_status(200)
            .with_header("Content-Type", "application/json")
            .with_body(
                json!({"data": [{"id": "text-davinci-003"}, {"id": "gpt-3.5-turbo"}]}).to_string(),
            )
            .create();

        let openai = Openai::new(server.url(), "123".to_string()).unwrap();
        let fetched_model_ids = openai.fetch_model_ids().await.unwrap();

        assert_eq!(fetched_model_ids, model_ids);
    }
}

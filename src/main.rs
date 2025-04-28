use std::{env, io::stdin, time::Instant};

use clap::Parser;
use dialoguer::Select;
use serde_json::{json, Value};

#[derive(Parser)]
#[command(name = "gptut")]
#[command(version = "0.0.1")]
#[command(author = "Serghei Mateas <matiaspub@gmail.com>")]
#[command(about = "A simple CLI app to chat with OpenAI models")]
struct Cli {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = Cli::parse();

    let api_key = env::var("OPENAI_API_KEY").expect("env OPENAI_API_KEY");
    let url = |endpoint: &str| format!("https://api.openai.com{}", endpoint);

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

    let resp = client.get(url("/v1/models")).send().await?.text().await?;

    let json_value: serde_json::Value = serde_json::from_str(&resp)?;

    let model_list: Vec<String> = json_value["data"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|item| item.get("id").and_then(Value::as_str))
        .map(String::from)
        .collect();

    let selection = Select::new()
        .with_prompt("Please choose the model")
        .default(6)
        .items(&model_list)
        .interact()
        .expect("error in model choosing");

    let model = &model_list[selection];

    loop {
        if 0 != 0 {
            break;
        }
        let mut user_input = String::new();
        println!("");
        println!(
            "\x1b[33mPlease enter the text for model [{}]:\x1b[0m",
            model
        );
        stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        let user_input = user_input.trim();
        println!("\x1b[90m{}", "reasoning...");

        let payload = json!({
            "model": model,
            "input": user_input
        });

        let ttl = Instant::now();

        let response = client
            .post(url("/v1/responses"))
            .json(&payload)
            .send()
            .await?
            .json::<Value>()
            .await?;

        let text = response["output"][1]["content"][0]["text"]
            .as_str()
            .expect("unexpected response structure");

        let elapsed = ttl.elapsed().as_secs();
        println!("{} seconds\x1b[0m\n", elapsed);

        println!("{}", text.replace("\\n", "\n"));
    }

    Ok(())
}

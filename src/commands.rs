use std::{error::Error, io::stdin, time::Instant};

use dialoguer::Select;

use crate::api::Openai;

pub async fn chat(api: Openai) -> Result<(), Box<dyn Error>> {
    let model_list = api.fetch_model_ids().await?;

    let selection = Select::new()
        .with_prompt("Please choose the model")
        .default(6)
        .items(&model_list)
        .interact()
        .expect("error in model choosing");

    let model = &model_list[selection];

    loop {
        let mut user_input = String::new();
        println!();
        println!(
            "\x1b[33mPlease enter the text for model [{}]:\x1b[0m",
            model
        );
        stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");
        let user_input = user_input.trim();
        println!("\x1b[90mreasoning...");

        let ttl = Instant::now();

        let text = api.fetch_response(model, &user_input.to_owned()).await?;

        let elapsed = ttl.elapsed().as_secs();
        println!("{} seconds\x1b[0m\n", elapsed);

        println!("{}", text.replace("\\n", "\n"));
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_chats() {
        todo!("find out about cli testing");
    }
}

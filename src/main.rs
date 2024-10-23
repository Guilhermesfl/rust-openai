use dotenvy::dotenv;
use openai::{
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
    set_base_url, set_key,
};

use std::{
    env,
    io::{stdin, stdout, Write},
};

fn config_openai() {
    dotenv().unwrap(); // Load environt variables
    set_key(env::var("OPENAI_KEY").unwrap());
    set_base_url(env::var("OPENAI_BASE_URL").unwrap());
}

#[tokio::main]
async fn main() {
    config_openai();

    let mut messages = vec![
        ChatCompletionMessage {
            role: ChatCompletionMessageRole::System,
            content: Some("You are a LLM running as a cli interface confgiured using the rust programming language".to_string()),
            name: None,
            function_call: None
        }
    ];

    loop {
        print!("User: ");
        stdout().flush().unwrap();

        let mut user_message_content = String::new();

        stdin().read_line(&mut user_message_content).unwrap();
        messages.push(ChatCompletionMessage {
            role: ChatCompletionMessageRole::User,
            content: Some(user_message_content),
            name: None,
            function_call: None,
        });

        let chat_completion = ChatCompletion::builder("curie", messages.clone())
            .create()
            .await
            .unwrap();
        let returned_message = chat_completion.choices.first().unwrap().message.clone();

        println!(
            "{:#?}: {}",
            &returned_message.role,
            &returned_message.content.clone().unwrap().trim()
        );

        messages.push(returned_message);
    }
}

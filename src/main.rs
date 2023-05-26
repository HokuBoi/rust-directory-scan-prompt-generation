use std::env;
use std::fs;
use std::path::Path;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use serde::Serialize;

#[derive(Serialize)]
struct Prompt {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: u32,
    top_p: f32,
    frequency_penalty: f32,
    presence_penalty: f32,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

async fn create_prompt(api_key: &str, prompt: Prompt) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(AUTHORIZATION, HeaderValue::from_str(format!("Bearer {}", api_key).as_str())?);

    let res = client.post("https://api.openai.com/v1/chat/completions")
        .headers(headers)
        .json(&prompt)
        .send()
        .await?;
        
    println!("{}", res.text().await?);
        
    Ok(())
}

fn get_directories<P: AsRef<Path>>(path: P, names: &mut Vec<String>) -> std::io::Result<()> {
    for entry in fs::read_dir(path)? {
        let dir = entry?;
        if dir.file_type()?.is_dir() {
            names.push(dir.file_name().into_string().unwrap());
            get_directories(dir.path(), names)?;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run <path_to_directory>");
        return Ok(());
    }

    let api_key = match env::var("OPENAI_API_KEY") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("OPENAI_API_KEY environment variable is not set");
            return Ok(());
        }
    };

    let root_dir = &args[1];

    let mut dir_names = Vec::new();
    get_directories(root_dir, &mut dir_names)?;

    let prompt_text = format!("The project with directories named {} is about:", dir_names.join(", "));
    
    let messages = vec![
        Message {
            role: "system".to_string(),
            content: "Set the behavior".to_string(),
        },
        Message {
            role: "assistant".to_string(),
            content: "Provide examples".to_string(),
        },
        Message {
            role: "user".to_string(),
            content: prompt_text,
        },
    ];

    let prompt = Prompt {
        model: "gpt-3.5-turbo".to_string(),
        messages,
        temperature: 0.05,
        max_tokens: 256,
        top_p: 1.0,
        frequency_penalty: 0.0,
        presence_penalty: 0.0,
    };


    create_prompt(&api_key, prompt).await?;

    Ok(())
}

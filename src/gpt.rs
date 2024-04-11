const GPT_URL: &str = "127.0.0.1:4891";
const GPT_MODEL: &str = "Nous Hermes 2 Mistral DPO";

fn local_gpt_body(message: &str, tokens: usize) -> String {
    format!(
        r#"
        {{
            "model": "{GPT_MODEL}",
            "max_tokens": {tokens},
            "messages": [
                {{
                    "role": "system",
                    "content": "You are a helpful assistant."
                }},
                {{
                    "role": "user",
                    "content": "{message}"
                }}
            ]
        }}
        "#
    )
}

pub fn local_gpt_chat(message: &str, tokens: usize) -> Option<String> {
    let url = "http://".to_owned() + &GPT_URL + "/v1/chat/completions";
    let client = reqwest::blocking::Client::new();
    let body = local_gpt_body(message, tokens);
    let result = client.post(url).body(body).send();
    if result.is_err() {
        return None;
    }
    let json = serde_json::from_str(&result.unwrap().text().unwrap());
    if json.is_err() {
        return None;
    }
    let value: serde_json::Value = json.unwrap();
    let choices = value.get("choices");
    if choices.is_none() {
        return None;
    }
    let message = choices.unwrap()[0].get("message");
    if message.is_none() {
        return None;
    }
    let content = message.unwrap().get("content");
    if content.is_none() {
        return None;
    }
    Some(content.unwrap().to_string())
}
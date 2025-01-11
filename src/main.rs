use reqwest::blocking::Client;
use std::env;

const ENDPOINT: &str = "https://api.openai.com/v1/chat/completions";
const MAX_COMPLETION_TOKENS: &str = "300";
const MODEL: &str = "gpt-4o-mini";
const DEVELOPER_PROMPT: &str = "You are replying in a terminal for platform <OS>. If you can reply with a canonically known answer, you do not need anything but the snippet, no commentary such as Use the following command is needed, just the one-liner. If unsure of the answer, you should then give a short answer as to why not. There will be no possibility to follow up the conversation. There is no syntax highlighting, do not use triple backticks (```) in your response.";

const BODY: &str = "
{
    \"model\": \"<MODEL>\",
    \"max_completion_tokens\": <MAX_COMPLETION_TOKENS>,
    \"messages\": [
      {
        \"role\": \"developer\",
        \"content\": \"<DEVELOPER_PROMPT>\"
      },
      {
        \"role\": \"user\",
        \"content\": \"<CONTENT>\"
      }
    ]
}
";

fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("Usage: {} <PROMPT>", args[0]);
        return;
    }
    let prompt = &args[1..].join(" ");

    let body = BODY
        .replace("<MODEL>", MODEL)
        .replace("<MAX_COMPLETION_TOKENS>", MAX_COMPLETION_TOKENS)
        .replace("<DEVELOPER_PROMPT>", DEVELOPER_PROMPT)
        .replace("<CONTENT>", prompt)
        .replace("<OS>", std::env::consts::OS);

    let response = Client::new()
        .post(ENDPOINT)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .expect("Failed to send request");

    let msg_start_key = "\"content\": ";
    let msg_end_key = "\"refusal\":";

    let text = response.text().expect("Failed to read response body");
    if let Some(start) = text.find(msg_start_key) {
        let msg_start = start + msg_start_key.len() + 1;
        if let Some(end) = text[msg_start..].find(msg_end_key) {
            let msg_end = msg_start + end;
            let content = &text[msg_start..msg_end].trim().replace("\\n", "\n");
            println!("{}", content[0..content.len() - 2].trim());
        } else {
            println!("{}", text.trim());
        }
    } else {
        println!("{}", text.trim());
    }
}

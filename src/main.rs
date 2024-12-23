use std::io::{self};
use serde_json::Value;
use reqwest::blocking::Client;

fn translate(source_lang: &str, target_lang: &str, text: &str) -> Result<String, String> {
    let url = "https://translate.google.com/translate_a/single";

    let params = [
        ("client", "gtx"),
        ("sl", source_lang),
        ("tl", target_lang),
        ("q", text),
        ("dt", "t"),
        ("ie", "UTF-8"),
        ("oe", "UTF-8"),
    ];
   
    let client = Client::new();
    let response = client.get(url).query(&params).send();

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                match resp.json::<Value>() {
                    Ok(data) => {
                        if let Some(array) = data.get(0).and_then(|d| d.as_array()) {
                            let result: String = array
                                .iter()
                                .filter_map(|item| item.get(0).and_then(|s| s.as_str()))
                                .collect();
                            Ok(result)
                        } else {
                            Err(format!("Error: Could not extract result from the response. Structure: {:?}", data))
                        }
                    }
                    Err(err) => Err(format!("Error parsing JSON response: {}", err)),
                }
            } else {
                Err(format!("Error: HTTP {}: {}", resp.status(), resp.text().unwrap_or_default()))
            }
        }
        Err(err) => Err(format!("Error processing request: {}", err)),
    }
}

fn main() {
    let mut input = String::new();

    println!("Enter the text to translate:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let text = input.trim();

    let mut source_lang = String::new();
    println!("Enter the source language code (e.g., 'en' for English):");
    io::stdin().read_line(&mut source_lang).expect("Failed to read source language");
    let source_lang = source_lang.trim();

    let mut target_lang = String::new();
    println!("Enter the target language code (e.g., 'ru' for Russian):");
    io::stdin().read_line(&mut target_lang).expect("Failed to read target language");
    let target_lang = target_lang.trim();

    match translate(source_lang, target_lang, text) {
        Ok(result) => {
            println!("Original Text ({}): {}", source_lang, text);
            println!("Translated Text ({}): {}", target_lang, result);
        }
        Err(err) => {
            eprintln!("Translation failed: {}", err);
        }
    }
}


use std::collections::HashSet;
fn is_valid_simple_json(contents: &str) -> bool {
    let keywords: HashSet<&str> = ["true", "false", "null"].iter().cloned().collect();
    let mut stack = Vec::new();
    let mut current_token = String::new();
    let mut expecting_key = true;
    
    for ch in contents.chars() {
        match ch {
            '{' | '[' => stack.push(ch),
            '}' | ']' => {
                if stack.is_empty() || (ch == '}' && stack.pop() != Some('{')) || (ch == ']' && stack.pop() != Some('[')) {
                    return false;
                }
            },
            '"' => {
                if expecting_key {
                    expecting_key = false;
                }
                // Handle string parsing properly, skipping for simplicity here
            },
            ':' => {
                if !expecting_key || current_token.is_empty() {
                    return false;
                }
                expecting_key = false;
                if !keywords.contains(&current_token.as_str()) && !current_token.parse::<i32>().is_ok() {
                    return false;
                }
                current_token.clear();
            },
            ',' => {
                if expecting_key {
                    return false;
                }
                expecting_key = true;
                current_token.clear();
            },
            _ => {
                if expecting_key && !ch.is_whitespace() {
                    return false; // Unexpected character when expecting a key
                }
                current_token.push(ch);
            },
        }
    }

    stack.is_empty() && current_token.is_empty() && expecting_key
}

fn main() {
    let json_str = r#"{
        "key1": true,
        "key2": false,
        "key3": null,
        "key4": "value",
        "key5": 101
    }"#;
    println!("Is valid JSON? {}", is_valid_simple_json(json_str));
}

// douxiaobo@192 Rust % rustc step3_valid.rs
// douxiaobo@192 Rust % ./step3_valid   
// Is valid JSON? false
// douxiaobo@192 Rust % 

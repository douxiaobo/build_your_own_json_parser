use std::env;
use std::fs;
// use std::io::Read;
fn main(){
    let args:Vec<String>=env::args().collect();
    if args.len()<2{
        println!("Please provide a file path");
        std::process::exit(1);  //  无效输入，退出代码为 1
    }

    let file_path=&args[1];
    println!("In file {}",file_path);

    let contents=fs::read_to_string(file_path).unwrap_or_else(|err|{
        eprintln!("Failed to read file: {}",err);
        std::process::exit(1);
    });
    // let mut file = fs::File::open(file_path).unwrap();  
    // let mut contents = String::new();  
    // file.read_to_string(&mut contents).unwrap();  
    println!("{}", contents);

    if contents.is_empty(){
        println!("File is empty");
        println!("invalide");
    } else {
        println!("File is not empty");
        println!("valid");
        if is_valid_simple_json(&contents) {  
            println!("File is a valid simple JSON object");  
            std::process::exit(0);  
        } else {  
            println!("File is not a valid simple JSON object");  
            std::process::exit(1);  
        }  
    }

    
}

fn is_valid_simple_json(contents: &str) -> bool {  
    let mut stack = Vec::new();  
    for ch in contents.chars() {  
        if ch == '{' {  
            stack.push(ch);  
        } else if ch == '}' && !stack.is_empty() && stack.last() == Some(&'{') {  
            stack.pop();  
        } else {  
            // 遇到其他字符或不成对的 '}'  
            return false;  
        }  
    }  
    // 如果堆栈为空，说明所有 '{' 都有对应的 '}'  
    return stack.is_empty();  
}  

// douxiaobo@192 Rust % rustc build_your_own_json_parser.rs                    
// douxiaobo@192 Rust % ./build_your_own_json_parser ./tests/step1/invalid.json
// In file ./tests/step1/invalid.json

// File is empty
// invalide
// douxiaobo@192 Rust % ./build_your_own_json_parser ./tests/step1/valid.json  
// In file ./tests/step1/valid.json
// {}
// File is not empty
// valid
// File is a valid simple JSON object
// douxiaobo@192 Rust % 

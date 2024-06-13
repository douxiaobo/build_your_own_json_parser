use std::collections::HashSet;
use std::env;
use std::fs;

// use std::io::Read;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a file path");
        std::process::exit(1); //  无效输入，退出代码为 1
    }

    let file_path = &args[1];
    // println!("In file {}",file_path);

    let contents = fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("Failed to read file: {}", err);
        std::process::exit(1);
    });
    // let mut file = fs::File::open(file_path).unwrap();
    // let mut contents = String::new();
    // // file.read_to_string(&mut contents).unwrap();
    // println!("{}", contents);

    if contents.is_empty() {
        println!("File is empty");
        println!("invalide");
    } else {
        println!("File is not empty");
        if is_valid_simple_json(&contents) {
            println!("valid");
            println!("File is a valid simple JSON object");
            std::process::exit(0);
        } else {
            println!("invalid");
            println!("File is not a valid simple JSON object");
            std::process::exit(1);
        }
    }
}

// fn is_valid_simple_json(contents: &str) -> bool {
//     let mut in_key = false;
//     let mut in_value = false;
//     let mut expect_colon = false;
//     let mut expect_comma = false;
//     let mut valid = true;

//     let chars: Vec<char> = contents.chars().collect();
//     let mut i = 0;

//     while i < chars.len() && valid {
//         match chars[i] {
//             '{' => {
//                 if i != 0 || expect_colon || in_key || in_value || expect_comma {
//                     valid = false;
//                 }
//                 in_key = true; // 准备读取键
//             }
//             '"' => {
//                 if in_key {
//                     // 键的开始或结束
//                     in_key = false;
//                     expect_colon = true;
//                 } else if expect_colon {
//                     // 值的开始
//                     in_value = true;
//                 } else if in_value {
//                     // 值的结束
//                     in_value = false;
//                     expect_comma = true;
//                 }
//             }
//             ':' => {
//                 if expect_colon {
//                     expect_colon = false;
//                 } else {
//                     valid = false;
//                 }
//             }
//             ',' => {
//                 if expect_comma {
//                     expect_comma = false;
//                     in_key = true; // 准备下一个键
//                 } else {
//                     valid = false;
//                 }
//             }
//             '}' => {
//                 if !in_key && !in_value && !expect_colon && !expect_comma {
//                     // 结束对象，验证成功
//                 } else {
//                     valid = false;
//                 }
//             }
//             _ => {
//                 if in_key && chars[i].is_alphanumeric() || chars[i] == '_' {
//                     // 键中允许字母、数字和下划线
//                 } else if in_value && chars[i].is_alphanumeric() || chars[i] == '_' {
//                     // 值中允许字母、数字和下划线
//                 } else if chars[i] == ' ' || chars[i] == '\n' || chars[i] == '\r' || chars[i] == '\t' {
//                     // 忽略空白字符
//                 } else {
//                     valid = false;
//                 }
//             }
//         }
//         i += 1;
//     }

//     valid && i == chars.len()
// }

// fn is_valid_simple_json(contents: &str) -> bool {
//     let mut in_string = false; // 标记是否在字符串中
//     let mut in_key = false;    // 标记是否在键中
//     let mut expect_colon = false; // 标记是否期待一个冒号
//     let mut expect_value = false; // 标记是否期待一个值

//     for (_i, ch) in contents.chars().enumerate() {
//         if ch == '"' {
//             // 进入或退出字符串
//             in_string = !in_string;
//             if in_key {
//                 // 如果在键中并且找到了引号，则期待一个冒号
//                 expect_colon = true;
//                 in_key = false;
//             } else if expect_colon {
//                 // 如果期待冒号并且找到了引号，则期待一个值
//                 expect_value = true;
//                 expect_colon = false;
//             }
//         } else if !in_string {
//             if ch == '{' {
//                 // 进入 JSON 对象
//             } else if ch == '}' {
//                 // 退出 JSON 对象，如果所有都匹配，则返回 true
//                 return !in_key && !expect_colon && !expect_value && !in_string;
//             } else if ch == ':' && expect_colon {
//                 // 找到了期待的冒号
//                 expect_colon = false;
//                 expect_value = true;
//             } else if ch == ',' && !expect_colon && !expect_value {
//                 // 逗号是键值对之间的分隔符，重置期待状态
//                 in_key = false;
//                 expect_colon = false;
//                 expect_value = false;
//             } else if (ch == ' ' || ch == '\n' || ch == '\t' || ch == '\r') && !in_key && !expect_colon && !expect_value {
//                 // 忽略空白字符
//             } else if (ch == 'k' || ch == 'v') && !in_key && !expect_colon && !expect_value {
//                 // 假设键和值可以以 'k' 或 'v' 开始
//                 in_key = true;
//             } else if ch == '=' && expect_value {
//                 // 假设值可以以 '=' 开始（这不是标准的 JSON 格式，但根据您的示例似乎需要）
//                 expect_value = false;
//             } else {
//                 // 任何其他字符都会导致验证失败
//                 return false;
//             }
//         }
//     }

//     // 如果所有字符都匹配了，且没有未闭合的字符串或未完成的键值对，则返回 true
//     !in_string && !in_key && !expect_colon && !expect_value
// }

// use std::collections::VecDeque;

// fn is_valid_simple_json(contents: &str) -> bool {
//     let mut stack = VecDeque::new();
//     let mut in_string = false;

//     // 跳过首尾空白字符
//     let trimmed = contents.trim();

//     // 检查是否是对象格式，即以 { 开头并以 } 结尾
//     if trimmed.starts_with('{') && trimmed.ends_with('}') {
//         for (i, ch) in trimmed[1..trimmed.len()-1].chars().enumerate() {
//             if ch == '"' {
//                 // 进入或离开字符串
//                 in_string = !in_string;
//             } else if !in_string {
//                 if ch == '{' {
//                     stack.push_back(i);
//                 } else if ch == '}' {
//                     if let Some(top) = stack.pop_back() {
//                         if top != i {
//                             // 匹配的 { 必须在当前 } 之前
//                             return false;
//                         }
//                     } else {
//                         // 没有匹配的 {
//                         return false;
//                     }
//                 }
//             }
//         }
//     }

//     // 如果栈为空且没有未关闭的字符串，则为有效 JSON 对象
//     stack.is_empty() && !in_string
// }

// fn is_valid_simple_json(contents: &str) -> bool {
//     // 检查是否是有效的 JSON 对象
//     let mut stack = Vec::new();
//     let mut in_string = false;
//     let mut in_key = false; // 标记是否在键的范围内

//     for ch in contents.chars() {
//         if ch == '"' {
//             // 切换字符串状态
//             in_string = !in_string;
//             in_key = false; // 字符串开始，键结束
//         } else if !in_string {
//             if ch == '{' {
//                 // 进入一个新的 JSON 对象
//                 stack.push('{');
//             } else if ch == '}' {
//                 // 尝试关闭 JSON 对象
//                 if stack.last() == Some(&'{') {
//                     stack.pop();
//                 } else {
//                     return false; // 没有匹配的 '{'
//                 }
//             } else if ch == ':' && !stack.is_empty() && stack.last() == Some(&'"') {
//                 // 键后面跟冒号
//                 in_key = false;
//             } else if ch == ',' {
//                 // 逗号，分隔键值对或对象
//                 if stack.last() == Some(&'}') || stack.last() == Some(&'{') {
//                     return false; // 逗号后不能直接是 '{' 或 '}'
//                 }
//             } else if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
//                 // 忽略空白字符
//             } else {
//                 // 其他字符，如果是字母、数字或下划线开头，则可能是键的开始
//                 in_key = true;
//             }
//         }
//     }

//     // 最终检查栈是否为空且没有未关闭的字符串或键
//     stack.is_empty() && !in_string && !in_key
// }

// fn is_valid_simple_json(contents: &str) -> bool {
//     let mut stack = Vec::new();
//     let mut in_string = false;
//     let mut last_was_colon = false; // 用于跟踪上次是否是冒号

//     for ch in contents.chars() {
//         if ch == '"' {
//             in_string = !in_string; // 切换字符串状态
//         } else if !in_string {
//             if ch == '{' || ch == '[' {
//                 stack.push(ch);
//             } else if ch == '}' || ch == ']' {
//                 if stack.last() != Some(&match ch {
//                     '}' => '{',
//                     ']' => '[',
//                     _ => unreachable!(),
//                 }) {
//                     return false;
//                 }
//                 stack.pop();
//             } else if ch == ':' {
//                 if last_was_colon || stack.last() != Some(&'"') {
//                     return false;
//                 }
//                 last_was_colon = true;
//             } else if ch == ',' {
//                 if stack.last() != Some(&'{') && !last_was_colon {
//                     return false;
//                 }
//                 last_was_colon = false;
//             } else if !ch.is_whitespace() {
//                 // 非空白字符且不在字符串中，应为无效字符
//                 return false;
//             }
//         } else {
//             // 字符串中可以包含任何字符，包括 `{`、`}`、`:` 和 `,`
//         }
//     }

//     // 检查栈是否为空以及 last_was_colon 是否为 false
//     stack.is_empty() && !last_was_colon
// }

fn is_valid_simple_json(contents: &str) -> bool {
    let keywords: HashSet<&str> = ["true", "false", "null"].iter().cloned().collect();
    let mut _is_key = true; // 标记是否处于键的范围内
    let mut is_value = false; // 标记是否处于值范围内
                              // let mut is_string = false; // 标记是否在字符串中
    let mut is_quotation = false; // 标记是否在引号中
    let mut stack = Vec::new(); // 栈用于记录当前的状态，包括是否处于字符串中，是否期待冒号，是否期待逗号，是否处于数组中，是否处于对象中
    let trimmed = contents.trim(); // 去掉首尾空格
    let mut inner_content = String::new();
    let mut string = String::new();
    let mut use_quotation = false;
    if trimmed.starts_with('{') && trimmed.ends_with('}') {
        // 确保除去 "{" 和 "}" 后的内容至少包含一个键值对
        inner_content = (&trimmed[1..trimmed.len() - 1]).to_string(); // 移除首尾的大括号
    }
    for ch in inner_content.chars() {
        if is_value == true && stack.last() == Some(&':') && ch != '"' && ch != ',' {
            string.push(ch);
        } else if ch == '{' || ch == '[' {
            stack.push(ch);
        } else if (ch == '}' && stack.last() == Some(&'{'))
            || (ch == ']' && stack.last() == Some(&'['))
        {
            stack.pop();
        } else if ch == ':' {
            // if is_quotation != true && is_key == true && is_value == false {
            //     return false;
            // }
            if stack.last() == Some(&',') {
                stack.pop();
            } else {
                stack.push(ch);
            }
            // stack.push(ch);
            _is_key = false;
            is_value = true;
            // is_quotation = false;
        } else if ch == ',' {
            stack.push(ch);
            _is_key = true;
            is_value = false;
            // is_quotation = false;
            if use_quotation == true {
                use_quotation = false;
                continue;
            } else if keywords.contains(&string.trim()) {
                // string.clear(); // 清空字符串，准备接收新的键
            } else if !string.is_empty() && !string.parse::<i32>().is_ok() {
                println!("No number.");
                // 检查是否为数字
                return false;
            }
            string.clear(); // 清空字符串，准备接收新的键
                            // if !keywords.contains(&string.as_str()) {
                            //     return false;
                            // } else {
                            //     for c in string.chars() {
                            //         if !c.is_digit(10) && stack.last() != Some(&'"') {
                            //             return false;
                            //         }
                            //     }
                            // }
                            // string="".to_string();
        } else if ch == '"' {
            if is_quotation == false {
                is_quotation = true;
                // is_string = true;
                stack.push(ch);
                if is_value == true {
                    use_quotation = true;
                }
            } else {
                is_quotation = false;
                // is_string = false;
                stack.pop();
            }
        }
        // else if is_key == true && stack.last() != Some(&'"') {
        //     // ./tests/step2/valid2.json
        //     return false;
        // }
        // println!("ch:{},stack:{:?},string:{}", ch, stack, string);
    }
    return stack.is_empty() || (stack.len() == 1 && stack.last() == Some(&':'));
}

// fn is_valid_simple_json(contents: &str) -> bool {
//     let mut stack = Vec::new();
//     let mut in_string = false; // 添加一个标志来跟踪是否处于字符串中，避免逗号和冒号在字符串内被误判

//     let trimmed = contents.trim();  // 去掉首尾空格
//     let mut inner_content=String::new();
//     if trimmed.starts_with('{') && trimmed.ends_with('}') {
//         // 确保除去 "{" 和 "}" 后的内容至少包含一个键值对
//         inner_content = (&trimmed[1..trimmed.len()-1]).to_string(); // 移除首尾的大括号
//         // 简单检查是否至少包含一个 ":" 来表示键值对
//         // inner_content.contains(':')
//     }

//     for ch in inner_content.chars() {
//         if ch=='{' || ch=='[' {
//             stack.push(ch);
//         } else if ch=='}' {
//             if stack.last() != Some(&'{') {
//                 return false;
//             }
//             stack.pop();
//         } else if ch==']' {
//             if stack.last() != Some(&'[') {
//                 return false;
//             }
//             stack.pop();
//         } else if ch=='"' &&(stack.is_empty() || stack.last() != Some(&',')) {
//             in_string = !in_string; // 切换字符串状态
//             if in_string {
//                 stack.push(ch);
//             } else {
//                 if stack.last() != Some(&'"') {
//                     return false;
//                 }
//                 stack.pop();
//             }
//         } else if ch==':' && !in_string && stack.last() != Some(&',') {
//             stack.push(ch);
//         } else if ch==':' && !in_string && stack.last() == Some(&',') {
//             stack.pop();
//             stack.push(ch);
//         } else if ch==',' && !in_string && stack.last() == Some(&'"') {
//             stack.push(ch);
//         } else if ch==',' && !in_string && stack.last() == Some(&']') {
//             stack.push(ch);
//         } else{
//             // 遇到其他字符或不成对的 '}'
//             return false;
//         }
//     }
//     // 如果堆栈为空，说明所有 '{' 都有对应的 '}'
//     return stack.is_empty()||(stack.len() == 1 && stack.last() == Some(&':'));
// }

// fn is_valid_simple_json(contents: &str) -> bool {
//     let mut stack = Vec::new();
//     for ch in contents.chars() {
//         if ch == '{' || (ch == ':' && !stack.is_empty() && stack.last() == Some(&'{')) {
//             stack.push(ch);
//         } else if (ch == '}' && !stack.is_empty() && stack.last() == Some(&'{'))
//         ||(ch == ',' && !stack.is_empty() && stack.last() == Some(&':')) {
//             stack.pop();
//             stack.push(ch)
//         } else if ch=='"' && stack.last() == Some(&'{') {
//             stack.push(ch);
//         } else if ch==':' && stack.last() == Some(&',') {
//             stack.pop();
//         }else if ch=='"' && !stack.is_empty() && stack.last() == Some(&'"') {
//             stack.pop();
//         } else{
//             // 遇到其他字符或不成对的 '}'
//             return false;
//         }
//     }
//     // 如果堆栈为空，说明所有 '{' 都有对应的 '}'
//     return stack.is_empty() || (stack.len() == 1 && stack.last() == Some(&':'));
// }

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

// douxiaobo@192 build your own json parser % echo "# Build Your Own Json Parser" >> README.md
// douxiaobo@192 build your own json parser % git init
// Initialized empty Git repository in /Users/douxiaobo/Documents/Coding/Practice in Coding/build your own json parser/.git/
// douxiaobo@192 build your own json parser % ls -a
// .		.DS_Store	README.md	tests.zip
// ..		.git		Rust
// douxiaobo@192 build your own json parser % echo "*.zip ./**/tests/" >> .gitignore
// douxiaobo@192 build your own json parser % ls -a
// .		.DS_Store	.gitignore	Rust
// ..		.git		README.md	tests.zip
// douxiaobo@192 build your own json parser % git add .
// douxiaobo@192 build your own json parser % git commit -m "Build Your Onw Json Parser Step1 in Rust"
// [main (root-commit) 04bf0f3] Build Your Onw Json Parser Step1 in Rust
//  19 files changed, 118 insertions(+)
//  create mode 100644 .DS_Store
//  create mode 100644 .gitignore
//  create mode 100644 README.md
//  create mode 100644 Rust/.DS_Store
//  create mode 100755 Rust/build_your_own_json_parser
//  create mode 100644 Rust/build_your_own_json_parser.rs
//  create mode 100644 Rust/tests/.DS_Store
//  create mode 100644 Rust/tests/step1/invalid.json
//  create mode 100644 Rust/tests/step1/valid.json
//  create mode 100644 Rust/tests/step2/invalid.json
//  create mode 100644 Rust/tests/step2/invalid2.json
//  create mode 100644 Rust/tests/step2/valid.json
//  create mode 100644 Rust/tests/step2/valid2.json
//  create mode 100644 Rust/tests/step3/invalid.json
//  create mode 100644 Rust/tests/step3/valid.json
//  create mode 100644 Rust/tests/step4/invalid.json
//  create mode 100644 Rust/tests/step4/valid.json
//  create mode 100644 Rust/tests/step4/valid2.json
//  create mode 100644 tests.zip
// douxiaobo@192 build your own json parser % vim .gitignore
// douxiaobo@192 build your own json parser % git add .
// douxiaobo@192 build your own json parser % git commit -m "Build Your Onw Json Parser Step1 in Rust"
// [main 2287aff] Build Your Onw Json Parser Step1 in Rust
//  16 files changed, 4 insertions(+), 1 deletion(-)
//  delete mode 100755 Rust/build_your_own_json_parser
//  rename {Rust/tests => tests}/.DS_Store (73%)
//  rename {Rust/tests => tests}/step1/invalid.json (100%)
//  rename {Rust/tests => tests}/step1/valid.json (100%)
//  rename {Rust/tests => tests}/step2/invalid.json (100%)
//  rename {Rust/tests => tests}/step2/invalid2.json (100%)
//  rename {Rust/tests => tests}/step2/valid.json (100%)
//  rename {Rust/tests => tests}/step2/valid2.json (100%)
//  rename {Rust/tests => tests}/step3/invalid.json (100%)
//  rename {Rust/tests => tests}/step3/valid.json (100%)
//  rename {Rust/tests => tests}/step4/invalid.json (100%)
//  rename {Rust/tests => tests}/step4/valid.json (100%)
//  rename {Rust/tests => tests}/step4/valid2.json (100%)
// douxiaobo@192 build your own json parser % git add .
// douxiaobo@192 build your own json parser % git commit -m "Build Your Onw Json Parser Step1 in Rust"
// [main 73c2259] Build Your Onw Json Parser Step1 in Rust
//  14 files changed, 0 insertions(+), 0 deletions(-)
//  rename {tests => Rust/tests}/.DS_Store (100%)
//  rename {tests => Rust/tests}/step1/invalid.json (100%)
//  rename {tests => Rust/tests}/step1/valid.json (100%)
//  rename {tests => Rust/tests}/step2/invalid.json (100%)
//  rename {tests => Rust/tests}/step2/invalid2.json (100%)
//  rename {tests => Rust/tests}/step2/valid.json (100%)
//  rename {tests => Rust/tests}/step2/valid2.json (100%)
//  rename {tests => Rust/tests}/step3/invalid.json (100%)
//  rename {tests => Rust/tests}/step3/valid.json (100%)
//  rename {tests => Rust/tests}/step4/invalid.json (100%)
//  rename {tests => Rust/tests}/step4/valid.json (100%)
//  rename {tests => Rust/tests}/step4/valid2.json (100%)
// douxiaobo@192 build your own json parser % git push origin main
// fatal: 'origin' does not appear to be a git repository
// fatal: Could not read from remote repository.

// Please make sure you have the correct access rights
// and the repository exists.
// douxiaobo@192 build your own json parser % git branch -M main
// douxiaobo@192 build your own json parser % git remote add origin git@github.com/douxiaobo/build_your_own_json_parser.git
// douxiaobo@192 build your own json parser % git push -u origin main
// fatal: 'git@github.com/douxiaobo/build_your_own_json_parser.git' does not appear to be a git repository
// fatal: Could not read from remote repository.

// Please make sure you have the correct access rights
// and the repository exists.
// douxiaobo@192 build your own json parser % git remote -v
// origin	git@github.com/douxiaobo/build_your_own_json_parser.git (fetch)
// origin	git@github.com/douxiaobo/build_your_own_json_parser.git (push)
// douxiaobo@192 build your own json parser % git push origin main
// fatal: 'git@github.com/douxiaobo/build_your_own_json_parser.git' does not appear to be a git repository
// fatal: Could not read from remote repository.

// Please make sure you have the correct access rights
// and the repository exists.
// douxiaobo@192 build your own json parser % git remote add origin git@github.com:douxiaobo/build_your_own_json_parser.get
// error: remote origin already exists.
// douxiaobo@192 build your own json parser % git branch -M main
// douxiaobo@192 build your own json parser % git push -u origin main
// fatal: 'git@github.com/douxiaobo/build_your_own_json_parser.git' does not appear to be a git repository
// fatal: Could not read from remote repository.

// Please make sure you have the correct access rights
// and the repository exists.
// douxiaobo@192 build your own json parser % git remote set-url origin git@github.com:douxiaobo/build_your_own_json_parser.git
// douxiaobo@192 build your own json parser % git branch -M main
// douxiaobo@192 build your own json parser % git push -u origin main
// Enumerating objects: 40, done.
// Counting objects: 100% (40/40), done.
// Delta compression using up to 14 threads
// Compressing objects: 100% (31/31), done.
// Writing objects: 100% (40/40), 202.41 KiB | 727.00 KiB/s, done.
// Total 40 (delta 8), reused 0 (delta 0), pack-reused 0
// remote: Resolving deltas: 100% (8/8), done.
// To github.com:douxiaobo/build_your_own_json_parser.git
//  * [new branch]      main -> main
// branch 'main' set up to track 'origin/main'.
// douxiaobo@192 build your own json parser % git rm --cached tests.zip
// rm 'tests.zip'
// douxiaobo@192 build your own json parser % git rm --cached -r ./*/tests/
// rm 'Rust/tests/.DS_Store'
// rm 'Rust/tests/step1/invalid.json'
// rm 'Rust/tests/step1/valid.json'
// rm 'Rust/tests/step2/invalid.json'
// rm 'Rust/tests/step2/invalid2.json'
// rm 'Rust/tests/step2/valid.json'
// rm 'Rust/tests/step2/valid2.json'
// rm 'Rust/tests/step3/invalid.json'
// rm 'Rust/tests/step3/valid.json'
// rm 'Rust/tests/step4/invalid.json'
// rm 'Rust/tests/step4/valid.json'
// rm 'Rust/tests/step4/valid2.json'
// douxiaobo@192 build your own json parser % git commit -m "Remove unwanted files from repository"
// [main c255b9e] Remove unwanted files from repository
//  13 files changed, 47 deletions(-)
//  delete mode 100644 Rust/tests/.DS_Store
//  delete mode 100644 Rust/tests/step1/invalid.json
//  delete mode 100644 Rust/tests/step1/valid.json
//  delete mode 100644 Rust/tests/step2/invalid.json
//  delete mode 100644 Rust/tests/step2/invalid2.json
//  delete mode 100644 Rust/tests/step2/valid.json
//  delete mode 100644 Rust/tests/step2/valid2.json
//  delete mode 100644 Rust/tests/step3/invalid.json
//  delete mode 100644 Rust/tests/step3/valid.json
//  delete mode 100644 Rust/tests/step4/invalid.json
//  delete mode 100644 Rust/tests/step4/valid.json
//  delete mode 100644 Rust/tests/step4/valid2.json
//  delete mode 100644 tests.zip
// douxiaobo@192 build your own json parser % nano .gitignore
// douxiaobo@192 build your own json parser % BFG Repo-Cleaner
// zsh: command not found: BFG
// douxiaobo@192 build your own json parser % git push origin main
// Enumerating objects: 5, done.
// Counting objects: 100% (5/5), done.
// Delta compression using up to 14 threads
// Compressing objects: 100% (3/3), done.
// Writing objects: 100% (3/3), 297 bytes | 297.00 KiB/s, done.
// Total 3 (delta 2), reused 0 (delta 0), pack-reused 0
// remote: Resolving deltas: 100% (2/2), completed with 2 local objects.
// To github.com:douxiaobo/build_your_own_json_parser.git
//    73c2259..c255b9e  main -> main
// douxiaobo@192 build your own json parser %

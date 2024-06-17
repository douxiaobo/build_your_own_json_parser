// use std::collections::HashSet;
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

fn is_valid_simple_json(contents: &str) -> bool {
    // let keywords: HashSet<&str> = ["true", "false", "null"].iter().cloned().collect();
    let mut is_key = true; // 标记是否处于键的范围内
    let mut is_value = false; // 标记是否处于值范围内
                              // let mut is_string = false; // 标记是否在字符串中
    let mut is_quotation = false; // 标记是否在引号中
    let mut stack = Vec::new(); // 栈用于记录当前的状态，包括是否处于字符串中，是否期待冒号，是否期待逗号，是否处于数组中，是否处于对象中
    let trimmed = contents.trim(); // 去掉首尾空格
    let mut inner_content = String::new();
    let mut string = String::new();
    let mut use_quotation = false;  //标记是否使用引号
    if trimmed.starts_with('{') && trimmed.ends_with('}') {
        // 确保除去 "{" 和 "}" 后的内容至少包含一个键值对
        inner_content = (&trimmed[1..trimmed.len() - 1]).to_string(); // 移除首尾的大括号
    }
    for ch in inner_content.chars() {
        if ch == '\'' { // 新增：检查单引号
            return false;
        } else if is_value == true && stack.last() == Some(&':') && ch != '"' && ch != ',' {
            if ch.is_uppercase(){
                return false;
            }
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
            if use_quotation==false && is_key == true && is_value == false {
                return false;
            }
            // stack.push(ch);
            is_key = false;
            is_value = true;
            // is_quotation = false;
        } else if ch == ',' {
            stack.push(ch);
            is_key = true;
            is_value = false;
            // is_quotation = false;
            if use_quotation == true {
                use_quotation = false;
                // continue;
            } else if string!="true" || string!="false" || string!="null" {
                // else if !keywords.contains(&string.trim()) {
                return false;
                // string.clear(); // 清空字符串，准备接收新的键
            } else if !string.is_empty() && !string.parse::<i32>().is_ok() {
                println!("No number.");
                // // 检查是否为数字
                return false;
            } 
            string.clear(); // 清空字符串，准备接收新的键
        } else if ch == '"' {
            if is_quotation == false {
                is_quotation = true;
                // is_string = true;
                stack.push(ch);
                // if is_value == true {
                //     use_quotation = true;
                // }
                use_quotation = true;
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
        println!("ch:{},stack:{:?},string:{}", ch, stack, string);
    }
    return stack.is_empty() || (stack.len() == 1 && stack.last() == Some(&':'));
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


// douxiaobo@192 rust % rustc build_your_own_json_parser.rs
// douxiaobo@192 rust % ./build_your_own_json_parser ./tests/step4/invalid.json
// File is not empty
// invalid
// File is not a valid simple JSON object
// douxiaobo@192 rust % ./build_your_own_json_parser ./tests/step4/valid.json 
// File is not empty
// valid
// File is a valid simple JSON object
// douxiaobo@192 rust % ./build_your_own_json_parser ./tests/step4/valid2.json
// File is not empty
// valid
// File is a valid simple JSON object
// douxiaobo@192 rust % ./build_your_own_json_parser ./tests/step3/valid.json
// File is not empty
// valid
// File is a valid simple JSON object
// douxiaobo@192 rust % ./build_your_own_json_parser ./tests/step3/invalid.json
// File is not empty
// invalid
// File is not a valid simple JSON object
// douxiaobo@192 rust % ./build_your_own_json_parser ./tests/step2/invalid.json
// File is not empty
// invalid
// File is not a valid simple JSON object
// douxiaobo@192 rust % ./build_your_own_json_parser ./tests/step2/invalid2.json
// File is not empty
// invalid
// File is not a valid simple JSON object
// douxiaobo@192 rust % ./build_your_own_json_parser ./tests/step2/valid.json 
// File is not empty
// valid
// File is a valid simple JSON object
// douxiaobo@192 rust % ./build_your_own_json_parser ./tests/step2/valid2.json
// File is not empty
// valid
// File is a valid simple JSON object
// douxiaobo@192 rust % ./build_your_own_json_parser ./tests/step1/valid.json 
// File is not empty
// valid
// File is a valid simple JSON object
// douxiaobo@192 rust % ./build_your_own_json_parser ./tests/step1/invalid.json
// File is empty
// invalide
// douxiaobo@192 rust % 

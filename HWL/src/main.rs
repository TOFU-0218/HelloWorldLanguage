use std::io::{self, Write};

fn main() {
    // ユーザーに入力を促す
    print!("> ");
    io::stdout().flush().unwrap();  // 出力をフラッシュして表示を確実に行う

    // ユーザーの入力を取得
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("入力エラー");

    // 改行を削除
    let input = input.trim();

    // プログラムをパースして実行
    match parse_code(input) {
        Ok(program) => program.run(),
        Err(e) => eprintln!("{}", e),
    }
}


fn parse_code(code: &str) -> Result<Program, String> {
    // 入力が空でない場合はエラーを返す
    if !code.is_empty() {
        return Err(String::from("Error: DO NOT INPUT ANY CODE\n"));
    }

    // デフォルトで「Hello, World!」出力命令を追加
    let instructions = vec![Instruction::OutputHelloWorld];

    Ok(Program { instructions })
}


enum Instruction {
    OutputHelloWorld, // "Hello, World!"を出力する命令
}

struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    fn run(&self) {
        #[allow(unreachable_patterns)]
        for instruction in &self.instructions {
            match instruction {
                Instruction::OutputHelloWorld => println!("Hello, World!\n"),
                _ => (), // あとから追加する命令とかここに書いてね
            }
        }
    }
}
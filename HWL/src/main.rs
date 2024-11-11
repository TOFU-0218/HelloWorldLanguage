use std::io::{self, Write};

const TAPE_SIZE: usize = 10;

struct HelloInterpreter {
    tape: [u8; TAPE_SIZE],
    pointer: usize,
    code: String,
}

impl HelloInterpreter {
    fn new(user_code: Option<String>) -> Self {
        // 最初に必ず実行される「Hello, World!」を出力するデフォルトコード
        let default_code = "llllllHlllllllll,HHl,lllllll,llll,o,llllll,llllll,lllll,lll,oooooo,";

        // ユーザーの入力がある場合はデフォルトコードの後に追加
        let full_code = match user_code {
            Some(user_code) => format!("{}{}", default_code, user_code),
            None => default_code.to_string(),
        };

        Self {
            tape: [0; TAPE_SIZE],
            pointer: 0,
            code: full_code,
        }
    }

    fn interpret(&mut self) {
        let code_chars: Vec<char> = self.code.chars().collect();
        let mut pc = 0; // プログラムカウンタ

        while pc < code_chars.len() {
            match code_chars[pc] {
                'H' => {
                    // ポインタを右に移動
                    self.pointer = (self.pointer + 1) % TAPE_SIZE;
                }
                'e' => {
                    // ポインタを左に移動
                    if self.pointer == 0 {
                        self.pointer = TAPE_SIZE - 1;
                    } else {
                        self.pointer -= 1;
                    }
                }
                'l' => {
                    // 現在のセルの値を1増やす
                    self.tape[self.pointer] = self.tape[self.pointer].wrapping_add(1);
                }
                'o' => {
                    // 現在のセルの値を1減らす
                    self.tape[self.pointer] = self.tape[self.pointer].wrapping_sub(1);
                }
                ',' => {
                    // 現在のセルの値をASCII文字として出力
                    print!("{}", self.tape[self.pointer] as char);
                }
                'W' => {
                    // 現在のセルの値を0にリセット
                    self.tape[self.pointer] = 0;
                }
                'r' => {
                    // プログラムの開始位置に戻る（ループ効果）
                    pc = 0;
                    continue;
                }
                'd' => {
                    // プログラムを終了
                    break;
                }
                _ => {} // 無視
            }
            pc += 1;
        }
    }
}

fn main() {
    // ユーザーに入力を促す
    print!("> ");
    io::stdout().flush().unwrap();

    // ユーザーの入力を取得
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // 改行を削除
    let input = input.trim().to_string();

    // 入力があればそれを使用し、なければNoneを指定
    let user_code = if input.is_empty() { None } else { Some(input) };

    // インタプリタを初期化し、デフォルトコード + ユーザーコードを実行
    let mut interpreter = HelloInterpreter::new(user_code);
    interpreter.interpret();
}

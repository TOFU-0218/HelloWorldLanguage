use std::io::{self, Write};

const TAPE_SIZE: usize = 1000;

struct HelloInterpreter {
    tape: [u8; TAPE_SIZE],
    pointer: usize,
    code: String,
}

impl HelloInterpreter {
    fn new(user_code: Option<String>) -> Self {
        // 最初に必ず実行される「Hello,World!」を出力するデフォルトコード
        let default_code = "llllllllllllllllllllllllllllllll!HllHllHllHllHllHllHllHllHllHllHlleeeeeeeeeeeo?HlHlHlHlHlHlHlHlHlHlHleeeeeeeeeeeHlllllll,Hllll,Hlllllllllll,Hlllllllllll,Hllllllllllllll,Hooooooooooooooooooooo,Hllllllllllllllllllllll,Hllllllllllllll,Hlllllllllllllllll,Hlllllllllll,Hlll,Hllllllllllllllll!Hlleo?Hl,Hllllllllll,d";

        // ユーザーの入力があればデフォルトコードの後に追加
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
                '!' => {
                    // 現在のセルが0なら対応する`?`までジャンプ
                    if self.tape[self.pointer] == 0 {
                        let mut open_brackets = 1;
                        while open_brackets > 0 {
                            pc += 1;
                            if pc >= code_chars.len() {
                                panic!("Error: no matching `?`");
                            }
                            if code_chars[pc] == '!' {
                                open_brackets += 1;
                            } else if code_chars[pc] == '?' {
                                open_brackets -= 1;
                            }
                        }
                    }
                }
                '?' => {
                    // 現在のセルが0でなければ対応する`!`に戻る
                    if self.tape[self.pointer] != 0 {
                        let mut close_brackets = 1;
                        while close_brackets > 0 {
                            if pc == 0 {
                                panic!("Error: no matching `!`");
                            }
                            pc -= 1;
                            if code_chars[pc] == '!' {
                                close_brackets -= 1;
                            } else if code_chars[pc] == '?' {
                                close_brackets += 1;
                            }
                        }
                    }
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
    io::stdin().read_line(&mut input).expect("Error: invalid input");

    // 改行を削除
    let input = input.trim().to_string();

    // 入力があればそれを使用し、なければNoneを指定
    let user_code = if input.is_empty() { None } else { Some(input) };

    // インタプリタを初期化し、デフォルトコード + ユーザーコードを実行
    let mut interpreter = HelloInterpreter::new(user_code);
    interpreter.interpret();
}

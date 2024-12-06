use encoding_rs::GBK;
use std::{
    env,
    io::{self, BufRead},
};

fn main() {
    let input = io::stdin();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Usage: grep <pattern>");
    }
    let pattern = args[1].clone();
    let mut buffer = vec![];

    let mut stdin = input.lock();
    while let Ok(n) = stdin.read_until(b'\n', &mut buffer) {
        if n == 0 {
            break; // EOF
        }
        // 尝试将行数据解码为 UTF-8
        let line = match String::from_utf8(buffer.clone()) {
            Ok(line) => line, // 成功解码为 UTF-8
            Err(_) => {
                // 如果不是 UTF-8，尝试用 GBK 解码
                let (decoded, _, had_errors) = GBK.decode(&buffer);
                if had_errors {
                    buffer.clear();
                    continue;
                }
                decoded.to_string()
            }
        };
        if line.contains(&pattern) {
            print!("{}", line);
        }
        buffer.clear();
    }
}

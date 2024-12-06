use std::{
    fs,
    io::{self, BufRead, Read},
};

use chardetng::EncodingDetector;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    files: Vec<String>,

    #[arg(short = 'n', long)]
    number: bool,

    #[arg(short = 'E', long)]
    show_ends: bool,
}

/// 读取文件。
fn main() -> io::Result<()> {
    let cli = Cli::parse();

    if cli.files.is_empty() {
        // Read from stdin
        cat_stdin(cli.number, cli.show_ends)?;
    } else {
        for filename in cli.files {
            cat_file(&filename, cli.number, cli.show_ends)?;
        }
    }
    Ok(())
}

fn cat_file(filename: &str, show_number: bool, show_ends: bool) -> io::Result<()> {
    let mut file = fs::File::open(filename)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut detector = EncodingDetector::new();
    detector.feed(&buffer, true);
    let encoding = detector.guess(None, true);

    // 解码文件
    let (decoded, _, _) = encoding.decode(&buffer);

    for (i, line) in decoded.lines().enumerate() {
        let mut line = line.to_string();
        if show_ends {
            line.push('$');
        }
        if show_number {
            println!("{:6>} {}", i, line);
        } else {
            println!("{}", line);
        }
    }
    Ok(())
}

fn cat_stdin(show_number: bool, show_ends: bool) -> io::Result<()> {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for (i, line) in reader.lines().enumerate() {
        let mut line = line?;
        if show_ends {
            line.push('$');
        }
        if show_number {
            println!("{:>6} {}", i + 1, line);
        } else {
            println!("{}", line);
        }
    }
    Ok(())
}

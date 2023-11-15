use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Файловый подсчитатель")]
#[command(author = "Саня")]
#[command(version = "1.0")]
#[command(about = "Утилита для подсчёта строк, слов, символов и букв в файле.", long_about = None)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Использовать файл
    #[arg(short, long, value_name = "FILE")]
    filename: PathBuf,

    /// Подсчёт слов
    #[arg(long, default_value_t = false)]
    words: bool,

    /// Подсчёт строк
    #[arg(long, default_value_t = false)]
    lines: bool,

    /// Подсчёт символов
    #[arg(long, default_value_t = false)]
    chars: bool,

    /// Подсчёт букв
    #[arg(long, default_value_t = false)]
    letters: bool,
}

fn main() {
    let args = Args::parse();

    // В Rust не существует исключений -- вместо них используется структура данных Result.
    // Result это enum из двух полей: Ok(_) и Err(_). Ok содержит внутри себя результат успешного
    // выполнения функции, а Err -- содержит ошибку.
    match File::open(&args.filename) {
        // Если ошибки нет, то начинаем работать с файлом
        Ok(file) => {
            // Оборачиваем файл в буфер, чтобы читать его по частями, а не весь сразу
            let reader = BufReader::new(file);

            let mut lines = 0;
            let mut words = 0;
            let mut chars = 0;
            let mut letters = 0;

            for line in reader.lines() {
                if let Ok(line) = line {
                    // Считаем строки
                    lines += 1;

                    // Считаем символы, если надо
                    chars += if args.chars { line.chars().count() } else { 0 };

                    // Считаем буквы, если надо
                    letters += if args.letters {
                        line.chars().filter(|c| c.is_alphabetic()).count()
                    } else {
                        0
                    };

                    // Считаем слова, если надо
                    words += if args.words { count_words(line) } else { 0 };
                }
            }

            // Выводим все нужные значения в консоль
            if args.lines {
                println!("Строк: {}", lines);
            }
            if args.words {
                println!("Слов: {}", words);
            }
            if args.chars {
                println!("Символов: {}", chars);
            }
            if args.letters {
                println!("Букв: {}", letters);
            }
        }

        // Обработка ошибки
        Err(e) => {
            println!(
                "Невозможно открыть файл: {}\nПричина: {}",
                args.filename.display(),
                e
            );
        }
    };
}

#[inline]
fn count_words(line: String) -> i32 {
    let mut total = 0;
    let mut prev_is_whitespace = true;

    for c in line.chars() {
        if prev_is_whitespace {
            if c.is_alphabetic() {
                total += 1;
                prev_is_whitespace = false;
            } else {
                prev_is_whitespace = true;
            }
        // Дефис нужен для слов вроде чёрно-белый
        } else if !c.is_alphabetic() && c != '-' {
            prev_is_whitespace = true;
        }
    }

    total
}

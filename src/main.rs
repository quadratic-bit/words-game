use std::collections::{HashSet, VecDeque};
use std::io::{self, BufRead, Write};
use colored::Colorize;

mod macros;

macro_rules! next_word {
    () => {
        print!("> ");
        let _ = io::stdout().flush();
        continue;
    };
}

fn prepare_string(token: String) -> String {
    token.to_lowercase()
}

const WORD_HISTORY_BUFFER_LENGTH: usize = 10;
const CHAR_AVOID_LIST: [char; 4] = ['-', 'ы', 'ь', 'ъ'];

fn main() {
    info!("Стартуем");
    let stdin = io::stdin();

    let mut words: HashSet<String> = HashSet::new();
    let mut last_words_buffer: VecDeque<String> = VecDeque::new();
    let mut results_filname: Option<String> = None;

    let mut args = std::env::args();

    while let Some(token) = args.next() {
        if token == "--save" {
            let filename = args.next().unwrap_or_else(|| {
                err!("Не указано имя файла после --save");
                std::process::exit(1);
            });
            results_filname = Some(filename);
        } else {
            err!("Неизвестный аргумент: {}", token);
            std::process::exit(1);
        }
    }

    print!("> ");
    let _ = io::stdout().flush();

    for line in stdin.lock().lines() {
        let query = prepare_string(line.unwrap());

        if query == "" {
            next_word!();
        }

        if query == "!назад" {
            if let Some(last_word) = last_words_buffer.iter().last() {
                words.remove(last_word);
                info!("Последнее слово \"{}\" отменено", &last_word);
                last_words_buffer.pop_back();
            } else {
                err!("Дальше перемещаться назад нельзя");
            }
            next_word!();
        }

        if query == "!выход" {
            info!("Завершаем игру");
            info!("Всего было сыграно слов: {}", words.len());
            if let Some(filename) = results_filname {
                let content = words
                    .iter()
                    .map(|w| &**w)
                    .collect::<Vec<&str>>()
                    .join("\n");
                std::fs::write(filename, content).unwrap_or_else(|e| {
                    err!("Не удалось записать результат в файл: {}", e);
                    std::process::exit(0);
                });
            } else {
                for item in words {
                    println!("{}", item);
                }
            }
            std::process::exit(0);
        }

        if query.starts_with("!") {
            err!("Такой команды не существует");
            next_word!();
        }

        if !query.chars().all(|c| c.is_alphabetic() || c == '-') {
            err!("Слово может содержать только буквы и дефис");
            next_word!();
        }

        if !query.chars().any(|c| !CHAR_AVOID_LIST.contains(&c)) {
            err!("Следующее слово невозомжно");
            next_word!();
        }

        if let Some(last_word) = last_words_buffer.iter().last() {
            let target_char = last_word.chars().rev().find(|c| !CHAR_AVOID_LIST.contains(c)).unwrap();
            if query.chars().next().unwrap() != target_char {
                err!("Слово должно начинаться с буквы \"{}\"", target_char);
                next_word!();
            }
        }

        if !words.insert(query.clone()) {
            err!("Это слово уже использовано");
        } else {
            last_words_buffer.push_back(query);
            if last_words_buffer.len() > WORD_HISTORY_BUFFER_LENGTH {
                last_words_buffer.pop_front();
            }
        }
        next_word!();
    }
}

use colored::Colorize;
use std::collections::{HashSet, VecDeque};
use std::io::{self, BufRead, Write};

mod locale;
mod macros;

const WORD_HISTORY_BUFFER_LENGTH: usize = 10;

fn main() {
    let stdin = io::stdin();

    let mut words: HashSet<String> = HashSet::new();
    let mut last_words_buffer: VecDeque<String> = VecDeque::new();
    let mut results_filname: Option<String> = None;

    let mut args = std::env::args().skip(1);

    while let Some(token) = args.next() {
        if token == "--save" {
            let filename = args.next().unwrap_or_else(|| {
                err!(locale::ERR_MISSING_FILENAME);
                std::process::exit(1);
            });
            results_filname = Some(filename);
        } else if token == "--help" {
            println!("{}", locale::HELP_STRING);
            return;
        } else {
            err!(locale::ERR_INVALID_CLI_ARGUMENT, token);
            std::process::exit(1);
        }
    }

    info!(locale::INF_WELCOME);
    let mut stdin = stdin.lock().lines();

    loop {
        print!("{}", locale::PROMPT);
        let _ = io::stdout().flush();

        let line = if let Some(res) = stdin.next() {
            res
        } else {
            break;
        };

        let query = line.unwrap().to_lowercase();

        if query.is_empty() {
            continue;
        }

        if query == locale::CMD_UNDO {
            if let Some(last_word) = last_words_buffer.iter().last() {
                words.remove(last_word);
                info!(locale::INF_UNDO_WORD, &last_word);
                last_words_buffer.pop_back();
            } else {
                err!(locale::ERR_END_OF_UNDO_BUFFER);
            }
            continue;
        }

        if query == locale::CMD_EXIT {
            info!(locale::INF_EXIT, words.len());
            if let Some(filename) = results_filname {
                let content = words.iter().map(|w| &**w).collect::<Vec<&str>>().join("\n");
                std::fs::write(filename, content).unwrap_or_else(|e| {
                    err!(locale::ERR_FAILED_WRITE, e);
                    std::process::exit(0);
                });
            } else {
                for item in words {
                    println!("{}", item);
                }
            }
            std::process::exit(0);
        }

        if query.starts_with('!') {
            err!(locale::ERR_INVALID_CMD);
            continue;
        }

        if query.starts_with('?') {
            println!("{}", locale::HELP_STRING);
            continue;
        }

        if !query.chars().all(|c| c.is_alphabetic() || c == '-') {
            err!(locale::ERR_INVALID_CHR);
            continue;
        }

        if !query.chars().any(|c| !locale::ILLEGAL_STARTING_CHRS.contains(&c)) {
            err!(locale::ERR_INVALID_WORD);
            continue;
        }

        if let Some(last_word) = last_words_buffer.iter().last() {
            let target_char = last_word
                .chars()
                .rev()
                .find(|c| !locale::ILLEGAL_STARTING_CHRS.contains(c))
                .unwrap();
            if query.chars().next().unwrap() != target_char {
                err!(locale::ERR_ILLEGAL_FIRST_CHR, target_char);
                continue;
            }
        }

        if !words.insert(query.clone()) {
            err!(locale::ERR_WORD_REPEAT);
        } else {
            last_words_buffer.push_back(query);
            if last_words_buffer.len() > WORD_HISTORY_BUFFER_LENGTH {
                last_words_buffer.pop_front();
            }
        }
    }
}

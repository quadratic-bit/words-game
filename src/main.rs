mod locale;
mod macros;

use colored::Colorize;
use locale::*;
use std::collections::{HashSet, VecDeque};
use std::io::{self, BufRead, Write};
use std::{process, fs, env};

const WORD_HISTORY_BUFFER_LENGTH: usize = 10;

fn main() {
    let stdin = io::stdin();

    let mut words: HashSet<String> = HashSet::new();
    let mut last_words_buffer: VecDeque<String> = VecDeque::new();
    let mut results_filname: Option<String> = None;

    let mut args = env::args().skip(1);

    while let Some(token) = args.next() {
        if token == "--save" {
            let filename = args.next().unwrap_or_else(|| {
                err!(ERR_MISSING_FILENAME);
                process::exit(1);
            });
            results_filname = Some(filename);
        } else if token == "--help" {
            println!("{}", HELP_STRING);
            return;
        } else {
            err!(ERR_INVALID_CLI_ARGUMENT, token);
            process::exit(1);
        }
    }

    info!(INF_WELCOME);
    let mut stdin = stdin.lock().lines();

    loop {
        print!("{}", PROMPT);
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

        if query == COMMAND_UNDO {
            if let Some(last_word) = last_words_buffer.iter().last() {
                words.remove(last_word);
                info!(INF_UNDO_WORD, &last_word);
                last_words_buffer.pop_back();
            } else {
                err!(ERR_END_OF_UNDO_BUFFER);
            }
            continue;
        }

        if query == COMMAND_EXIT {
            info!(INF_EXIT, words.len());
            if let Some(filename) = results_filname {
                let content = words
                    .iter()
                    .map(|w| &**w)
                    .collect::<Vec<&str>>()
                    .join("\n");
                fs::write(filename, content).unwrap_or_else(|e| {
                    err!(ERR_FAILED_WRITE, e);
                    process::exit(1);
                });
            } else {
                for item in words {
                    println!("{}", item);
                }
            }
            break;
        }

        if query.starts_with('!') {
            err!(ERR_INVALID_COMMAND);
            continue;
        }

        if query.starts_with('?') {
            println!("{}", HELP_STRING);
            continue;
        }

        if !query.chars().all(|c| c.is_alphabetic() || c == '-') {
            err!(ERR_INVALID_CHARACTER);
            continue;
        }

        if !query.chars().any(|c| !ILLEGAL_STARTING_CHARACTERS.contains(&c)) {
            err!(ERR_INVALID_WORD);
            continue;
        }

        if let Some(last_word) = last_words_buffer.iter().last() {
            let target_char = last_word
                .chars()
                .rev()
                .find(|c| !ILLEGAL_STARTING_CHARACTERS.contains(c))
                .unwrap();
            if query.chars().next().unwrap() != target_char {
                err!(ERR_ILLEGAL_FIRST_CHARACTER, target_char);
                continue;
            }
        }

        if !words.insert(query.clone()) {
            err!(ERR_WORD_REPEAT);
        } else {
            last_words_buffer.push_back(query);
            if last_words_buffer.len() > WORD_HISTORY_BUFFER_LENGTH {
                last_words_buffer.pop_front();
            }
        }
    }
}

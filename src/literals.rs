//! Defines all string literal used in a game.

/// All characters that cannot be starters to any existing word.
pub const ILLEGAL_STARTING_CHARACTERS: [char; 4] = ['-', 'ы', 'ь', 'ъ'];

/// Sequence of prompting characters before every user input.
pub const PROMPT: &str = "> ";

/// Sequence of characters put before every error message.
pub const PREFIX_ERR: &str = "[-] ";

/// Sequence of characters put before every informational message.
pub const PREFIX_INF: &str = "[i] ";

pub const COMMAND_UNDO: &str = "!!";
pub const COMMAND_EXIT: &str = "!#";

pub const HELP_STRING: &str = "!! - отменить последнее принятое слово\n\
                               !# - завершить игру\n\
                               ?  - вывести это меню";

pub const ERR_MISSING_FILENAME: &str = "Не указано имя файла после --save";
pub const ERR_END_OF_UNDO_BUFFER: &str = "Дальше перемещаться назад нельзя";
pub const ERR_INVALID_COMMAND: &str = "Такой команды не существует";
pub const ERR_INVALID_CHARACTER: &str = "Слово может содержать только буквы и дефис";
pub const ERR_WORD_REPEAT: &str = "Это слово уже использовано";

/// Prints whenever a prompted word consists of
/// ILLEGAL_STARTING_CHARACTERS items only.
pub const ERR_INVALID_WORD: &str = "Следующее слово невозомжно";

/// Prints when an invalid argument was passed to the executable.
/// The argument in question is placed right after, separated by a whitespace.
pub const ERR_INVALID_CLI_ARGUMENT: &str = "Неизвестный аргумент:";

/// Prints when write to a file failed. The error is placed right after,
/// separated by a whitespace.
pub const ERR_FAILED_WRITE: &str = "Не удалось записать результат в файл:";

/// Prints whenever a propmted word's first character diverges from the last
/// character of a previous word. The letter is placed right after,
/// separated by a whitespace.
pub const ERR_ILLEGAL_FIRST_CHARACTER: &str = "Слово должно начинаться с буквы";

pub const INF_WELCOME: &str = "Введите ? чтобы узнать существующие команды";

/// Prints after COMMAND_EXIT was prompted. The word count is placed
/// right after, separated by a single whitespace.
pub const INF_EXIT: &str = "Завершаем игру\n\
                            Всего было сыграно слов:";

/// Prints every time last word is cancelled. The word itself is printed
/// right after, separated by a single whitespace.
pub const INF_UNDO_WORD: &str = "Последнее слово отменено:";

pub const ILLEGAL_STARTING_CHARACTERS: [char; 4] = ['-', 'ы', 'ь', 'ъ'];

pub const PROMPT: &str = "> ";
pub const COMMAND_UNDO: &str = "!!";
pub const COMMAND_EXIT: &str = "!#";

pub const HELP_STRING: &str = "!! - отменить последнее принятое слово\n\
                               !# - завершить игру\n\
                               ?  - вывести это меню";

pub const ERR_MISSING_FILENAME: &str = "Не указано имя файла после --save";
pub const ERR_END_OF_UNDO_BUFFER: &str = "Дальше перемещаться назад нельзя";
pub const ERR_INVALID_COMMAND: &str = "Такой команды не существует";
pub const ERR_INVALID_CHARACTER: &str = "Слово может содержать только буквы и дефис";
pub const ERR_INVALID_WORD: &str = "Следующее слово невозомжно";
pub const ERR_WORD_REPEAT: &str = "Это слово уже использовано";
pub const ERR_INVALID_CLI_ARGUMENT: &str = "Неизвестный аргумент:";
pub const ERR_FAILED_WRITE: &str = "Не удалось записать результат в файл:";
pub const ERR_ILLEGAL_FIRST_CHARACTER: &str = "Слово должно начинаться с буквы";

pub const INF_WELCOME: &str = "Введите ? чтобы узнать существующие команды";
pub const INF_EXIT: &str = "Завершаем игру\n\
                            Всего было сыграно слов:";
pub const INF_UNDO_WORD: &str = "Последнее слово отменено:";

//! Defines all string literal used in a game.

/// All characters that cannot be starters to any existing word.
pub const ILLEGAL_STARTING_CHARACTERS: [char; 1] = ['-'];

/// Sequence of prompting characters before every user input.
pub const PROMPT: &str = "> ";

/// Sequence of characters put before every error message.
pub const PREFIX_ERR: &str = "[-] ";

/// Sequence of characters put before every informational message.
pub const PREFIX_INF: &str = "[i] ";

pub const COMMAND_UNDO: &str = "!undo";
pub const COMMAND_EXIT: &str = "!exit";

pub const HELP_STRING: &str = "!undo - cancel the last prompted word\n\
                               !exit - exit the game and print statistics\n\
                               ?     - print this menu";

pub const ERR_MISSING_FILENAME: &str = "No filename specified after --save";
pub const ERR_END_OF_UNDO_BUFFER: &str = "Cannot undo further";
pub const ERR_INVALID_COMMAND: &str = "No such command exists";
pub const ERR_INVALID_CHARACTER: &str = "A word can only contain letters and a hyphen";
pub const ERR_WORD_REPEAT: &str = "This word is already used";

/// Prints whenever a prompted word consists of
/// ILLEGAL_STARTING_CHARACTERS items only.
pub const ERR_INVALID_WORD: &str = "This word makes the following move impossible";

/// Prints when an invalid argument was passed to the executable.
/// The argument in question is placed right after, separated by a whitespace.
pub const ERR_INVALID_CLI_ARGUMENT: &str = "Unknown argument:";

/// Prints when write to a file failed. The error is placed right after,
/// separated by a whitespace.
pub const ERR_FAILED_WRITE: &str = "Failed to write to a file:";

/// Prints whenever a propmted word's first character diverges from the last
/// character of a previous word. The letter is placed right after,
/// separated by a whitespace.
pub const ERR_ILLEGAL_FIRST_CHARACTER: &str = "The next word must start with the letter";

pub const INF_WELCOME: &str = "Prompt ? to see all existing commands";

/// Prints after COMMAND_EXIT was prompted. The word count is placed
/// right after, separated by a single whitespace.
pub const INF_EXIT: &str = "Finishing the game\n\
                            Total words played:";

/// Prints every time last word is cancelled. The word itself is printed
/// right after, separated by a single whitespace.
pub const INF_UNDO_WORD: &str = "Previous word cancelled:";

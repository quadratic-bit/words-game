# Words game

A game of [word chain](https://en.wikipedia.org/wiki/Word_chain) made by me
in 30 spare minutes because of the flight delay.

## Usage

Upon executing the binary, you'll be prompted to enter a word. A "word" is a
case-insensitive sequence of Unicode `Alphabetic` characters (or a hyphen `-`).

> [!NOTE]
> This is the only criteria for a word, as every other constraint
> limits the available languages and specific words players are allowed to use.

Every following word must start with the same letter the previous one
ended with, and all words must be unique. Here's an example of a word chain:

```console
$ words
> part
> torch
> horse
```

Usually, the game is played by multiple players, and everyone who cannot
think of the next word loses, but this varies from country to country, so
you are free to modify the rules however you wish.

If you try to enter a word that was already introduced, you'll get a warning:

```console
> torch
[-] This word has already been used
```

You can also undo a couple of last words with `!!`, in case of typos:

```console
> etherium
> !!
[i] Previous word was cancelled: etherium
> ethereum
```

All available commands can be viewed by prompting `?`
or by passing `--help` to the executable

## Building

This game is written in Rust, so you'll need to compile via `cargo`:

```console
$ git clone https://github.com/quadratic-bit/words-game
$ cd words-game
$ cargo build --release
$ ./target/release/words --help
```

## Modification

### Locali(z|s)ation

All string literals are stored in [`src/literals.rs`](src/literals.rs) with
explanations, so changing UI language should be no problem.
In there, you can also change the default prompt `> `.

There's also this array:
```rust
pub const ILLEGAL_STARTING_CHARACTERS: [char; 4] = ['-', 'ы', 'ь', 'ъ'];
```
which controls all characters that a word cannot start with
(e.g. russian ы, ь, ъ and a hyphen). The player will receive a warning if
a new word is constructed using only these characters, as no word can
be a follow-up.

### Undo buffer size

There are loads of words to come up with in such a game, so storing all of them
in a vector is not reasonable. But it is reasonable to store the last `n` words
in their original order for the "undo" feature.
By default, this number is `10`,
but it can be changed in [`src/main.rs`](src/main.rs):

```rust
const WORD_HISTORY_BUFFER_LENGTH: usize = 10;
```

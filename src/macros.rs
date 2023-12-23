#[macro_export]
macro_rules! info {
    ( $msg: literal ) => {
        println!("{} {}", "[i]".blue(), $msg.blue());
    };

    ( $msg: literal, $( $arg: expr ),* ) => {
        println!("{} {}", "[i]".blue(), format!($msg, $( $arg ),*).blue());
    };
}

#[macro_export]
macro_rules! err {
    ( $msg: literal ) => {
        println!("{} {}", "[-]".red(), $msg.red());
    };

    ( $msg: literal, $( $arg: expr ),* ) => {
        println!("{} {}", "[-]".red(), format!($msg, $( $arg ),*).red());
    };
}

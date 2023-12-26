//! Defines convenient macros for communicating with the player.

#[macro_export]
macro_rules! info {
    ( $msg: path ) => {
        println!("{} {}", "[i]".blue(), $msg.blue());
    };

    ( $msg: path, $arg: expr ) => {
        println!(
            "{} {} {}",
            "[i]".blue(),
            $msg.blue(),
            $arg.to_string().blue()
        );
    };
}

#[macro_export]
macro_rules! err {
    ( $msg: path ) => {
        println!("{} {}", "[-]".red(), $msg.red());
    };

    ( $msg: path, $arg: expr ) => {
        println!(
            "{} {} {}",
            "[-]".red(),
            $msg.red(),
            $arg.to_string().red()
        );
    };
}

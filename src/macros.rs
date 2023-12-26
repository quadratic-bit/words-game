//! Defines convenient macros for communicating with the player.

#[macro_export]
macro_rules! info {
    ( $msg: path ) => {
        println!("{}{}", PREFIX_INF.blue(), $msg.blue());
    };

    ( $msg: path, $arg: expr ) => {
        println!(
            "{}{} {}",
            PREFIX_INF.blue(),
            $msg.blue(),
            $arg.to_string().blue()
        );
    };
}

#[macro_export]
macro_rules! err {
    ( $msg: path ) => {
        println!("{}{}", PREFIX_ERR.red(), $msg.red());
    };

    ( $msg: path, $arg: expr ) => {
        println!(
            "{}{} {}",
            PREFIX_ERR.red(),
            $msg.red(),
            $arg.to_string().red()
        );
    };
}

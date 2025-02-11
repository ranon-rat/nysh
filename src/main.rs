/*
 * nysh shell by jumang
 * a cute shell
 * that written in rust lang
 */

// - objects
mod nysh_builtin;
mod tools;
// - generic input output thingy
use std::env;
use std::io::*;
use std::process::exit;
// - Emoji receiver usage
use console::Emoji;
use rand::thread_rng;
use rand::Rng;

// - main func
fn main() {
    // - say hi!
    tools::welcome_uis::say_welcome().unwrap_or_else(|err: std::io::Error| {
        eprintln!("IO error => {}", err);
        exit(1);
    });
    tools::welcome_uis::logined_as().unwrap_or_else(|err: std::io::Error| {
        eprintln!("IO error => {}", err);
        exit(1);
    });
    // shell loops
    loops();
}

pub fn loops() {
    // - vars
    // drawing emoji
    let mut rng = thread_rng();
    // - shell loops
    loop {
        // - cummand waiter
        // get current directory
        let _current_path_chr = env::current_dir().unwrap();
        let current_path: String;
        // shorted path Option<String> goes here
        match tools::command_uis::path_abbr(String::from(_current_path_chr.to_str().unwrap())) {
            Ok(path) => current_path = path,
            Err(_) => {
                eprintln!("path error => ?path");
                exit(1);
            }
        }
        let _emoji_keys: usize = rng.gen_range(0..5);
        print!(
            "{} {} ",
            current_path,
            Emoji(&tools::command_uis::get_emoji(_emoji_keys), ""),
        );
        stdout().flush().unwrap();

        // listen to the user input
        let mut waiter = String::new();
        stdin().read_line(&mut waiter).expect("input error => ?");

        // parse command into command object
        let mut commands = tools::parser::CommandParser::constructor(waiter);
        commands.parse_it();

        let mut runner = tools::runner::CommandRunner::constructor(commands).unwrap();
        runner.run_command();
    }
}

use std::io;
use crate::ui;
use crate::domain;

#[derive(Debug,PartialEq)]
enum AppCmd {
    Quit,
    None,
    AddMissingText,
    ToggleMissingIndex,
    Add(String),
    Toggle(usize),
}

pub fn app_loop() {
    ui::greetings();
    let mut domain = domain::TodoDomain::new();
    loop {
        print!("{}", domain.to_string());
        println!("q: [quit], a: [new item name], t: [toggle]");

        let mut input: String = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("error reading input"); // only os-level error

        let cmd = parse_command(&input);
        match cmd {
            AppCmd::Quit => return,
            AppCmd::None => {
                println!("missing command");
            }
            AppCmd::AddMissingText => {
                println!("missing text");
            }
            AppCmd::ToggleMissingIndex => {
                println!("missing index");
            }
            AppCmd::Add(str) => {
                domain = domain.command(domain::DomainCmd::Add(str));
            }
            AppCmd::Toggle(i) => {
                domain = domain.command(domain::DomainCmd::Toggle(i));
            }
        }
    }
}

fn parse_command(input: &String) -> AppCmd {
    if input.starts_with("q:") {
        return AppCmd::Quit;
    } else if input.starts_with("a:") {
        if input.len() == 2 {
            return AppCmd::AddMissingText;
        }
        let text = input[2..].trim().to_string();
        if text.len() > 0 {
            return AppCmd::Add(text);
        }
        return AppCmd::AddMissingText;
    } else if input.starts_with("t:") {
        if input.len() == 2 {
            return AppCmd::ToggleMissingIndex;
        } else {
            // not sure why warning / how reformat
            let _i: usize = match input[2..].trim().parse() {
                Ok(i) => {
                    return AppCmd::Toggle(i);
                }
                Err(_) => {
                    return AppCmd::ToggleMissingIndex;
                }
            };
        }
    }
    return AppCmd::None;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn quit() {
        assert_eq!(parse_command(&String::from("q:")), AppCmd::Quit);
        assert_eq!(parse_command(&String::from("q:goddammend")), AppCmd::Quit);
        assert_eq!(parse_command(&String::from("q: !!!  ")), AppCmd::Quit);
    }
    #[test]
    fn add() {
        assert_eq!(parse_command(&String::from("a:")), AppCmd::AddMissingText);
        assert_eq!(parse_command(&String::from("a:    ")), AppCmd::AddMissingText);
        assert_eq!(parse_command(&String::from("a:123ABC")), AppCmd::Add(String::from("123ABC")));
        assert_eq!(parse_command(&String::from("a:    123ABC   ")), AppCmd::Add(String::from("123ABC")));
    }
    #[test]
    fn toggle() {
        assert_eq!(parse_command(&String::from("t:")), AppCmd::ToggleMissingIndex);
        assert_eq!(parse_command(&String::from("t:    ")), AppCmd::ToggleMissingIndex);
        assert_eq!(parse_command(&String::from("t: abc")), AppCmd::ToggleMissingIndex);
        assert_eq!(parse_command(&String::from("t:0")), AppCmd::Toggle(0));
        assert_eq!(parse_command(&String::from("t: 0")), AppCmd::Toggle(0));
        assert_eq!(parse_command(&String::from("t: 21")), AppCmd::Toggle(21));
        assert_eq!(parse_command(&String::from("t: 21 ")), AppCmd::Toggle(21));
    }
}

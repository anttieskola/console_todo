use std::env;
use std::io;
use crate::application::AppCmd;
use crate::domain::DomainCmd;
use crate::domain::TodoDomain;
use crate::infra;

/// public application api
pub fn start(historylog: &str) {
    greetings();

    // "load state"
    let mut domain = play_history(historylog);
    
    // app loop
    app(&mut domain, historylog);
}

fn greetings() {
    println!("Hello 👋 {}, what shall we do today?", get_username());
}

/// history "playback"
fn play_history(historylog: &str) -> TodoDomain {
    // vanilla domain
    let mut domain = TodoDomain::new();
    // read history
    let history = infra::read(historylog);
    // replay
    for (index, event) in history.iter().enumerate() {
        let cmd = parse_event(event);
        match cmd {
            AppCmd::Add(str) => {
                println!("a: {}", str);
                domain = domain.command(DomainCmd::Add(str));
            }
            AppCmd::Toggle(i) => {
                println!("t: {}", i);
                domain = domain.command(DomainCmd::Toggle(i));
            },
            _ => {
                println!("unknown event error at index:{}, event name: {}",index, event.name);
            }
        }
    }
    return domain;
}

/// todo: windows support
fn get_username() -> String {
    for (key, value) in env::vars() {
        if key == "USER" && !value.is_empty() {
            return value;
        }
    }
    return "unknown".to_string();
}

/// event log parser
fn parse_event(event: &infra::Event) -> AppCmd {
    match event.name.as_str() {
        "AddV1" => {
            if event.data.len() > 0 {
                return AppCmd::Add(event.data[0].to_string());
            } else {
                println!("AddV1: missing data");
            }
        }
        "ToggleV1" => {
            if event.data.len() > 0 {
                match event.data[0].parse() {
                    Ok(i) => {
                        return AppCmd::Toggle(i);
                    }
                    Err(e) => {
                        println!("ToggleV1 parse error: {}", e);
                    }
                }
            } else {
                println!("ToggleV1: missing data");
            }
        }
        _ => {
            println!("error, unknown event: {}", event.name);
        }
    }
    return AppCmd::None;
}

/// app loop
fn app<'a>(domain: &'a mut TodoDomain, historylog: &str) -> &'a TodoDomain {
    loop {
        print!("{}", domain.to_string());
        println!("q: / Q: [quit], a: [new item name], t: [toggle]");
        let mut input: String = String::new();
        match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let cmd = parse_command(&input);
                    match cmd {
                        AppCmd::Quit => return domain,
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
                            let log = str.clone();
                            infra::push(historylog, infra::Event{ name: "AddV1".to_string(), data: vec![log] });
                            *domain = domain.command(DomainCmd::Add(str));
                        }
                        AppCmd::Toggle(i) => {
                            let log = i.to_string();
                            infra::push(historylog, infra::Event{ name: "ToggleV1".to_string(), data: vec![log] });
                            *domain = domain.command(DomainCmd::Toggle(i));
                        }
                    }     
                },
                Err(_) => { println!("no input"); }
            };
    }

}

/// command parser
fn parse_command(input: &String) -> AppCmd {
    if input.starts_with("q:") || input.starts_with("Q:") {
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
            
            match input[2..].trim().parse() {
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
    fn greetings() {
        crate::ui::greetings();
    }

    #[test]
    fn username() {
        let username = get_username();
        assert!(!username.is_empty());
    }
    
    #[test]
    fn parse_command_quit() {
        assert_eq!(parse_command(&String::from("q:")), AppCmd::Quit);
        assert_eq!(parse_command(&String::from("q:goddammend")), AppCmd::Quit);
        assert_eq!(parse_command(&String::from("q: !!!  ")), AppCmd::Quit);
        assert_eq!(parse_command(&String::from("Q: !!!  ")), AppCmd::Quit);
    }
    
    #[test]
    fn parse_command_add() {
        assert_eq!(parse_command(&String::from("a:")), AppCmd::AddMissingText);
        assert_eq!(parse_command(&String::from("a:    ")), AppCmd::AddMissingText);
        assert_eq!(parse_command(&String::from("a:123ABC")), AppCmd::Add(String::from("123ABC")));
        assert_eq!(parse_command(&String::from("a:    123ABC   ")), AppCmd::Add(String::from("123ABC")));
    }
    
    #[test]
    fn parse_command_toggle() {
        assert_eq!(parse_command(&String::from("t:")), AppCmd::ToggleMissingIndex);
        assert_eq!(parse_command(&String::from("t:    ")), AppCmd::ToggleMissingIndex);
        assert_eq!(parse_command(&String::from("t: abc")), AppCmd::ToggleMissingIndex);
        assert_eq!(parse_command(&String::from("t:0")), AppCmd::Toggle(0));
        assert_eq!(parse_command(&String::from("t: 0")), AppCmd::Toggle(0));
        assert_eq!(parse_command(&String::from("t: 21")), AppCmd::Toggle(21));
        assert_eq!(parse_command(&String::from("t: 21 ")), AppCmd::Toggle(21));
    }    
}

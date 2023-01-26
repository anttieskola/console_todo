use std::io::BufRead;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;

/// Basic raw event
pub struct Event {
    pub name: String,
    /// "Unlimited storage"
    pub data: Vec<String>,
}

impl Event {
    /// Convention ?
    pub fn new() -> Self {
        Event {
            name: String::new(),
            data: Vec::new(),
        }
    }
    /// Returns `true` if name defined
    pub fn valid(self: &Self) -> bool {
        !self.name.is_empty()
    }
}

/// Empty vector if no saved history
pub fn read(path: &str) -> Vec<Event> {
    let log = OpenOptions::new().read(true).open(path);
    match log {
        Ok(log_file) => read_file(log_file),
        Err(e) => {
            println!("open ({}) error: {}", path, e);
            Vec::new()
        }
    }
}

fn read_file(file: File) -> Vec<Event> {
    let mut reader = BufReader::new(file);
    let mut history = Vec::new();
    loop {
        let mut event = Event::new();
        let mut line = String::new();

        // header/name
        let result = match reader.read_line(&mut line) {
            Ok(len) => {
                if !line.trim().is_empty() {
                    event.name = line.trim().to_string();
                }
                len
            }
            Err(e) => {
                println!("read_file error: {}", e);
                0
            }
        };

        if result == 0 {
            // eof
            return history;
        }

        if event.valid() {
            loop {
                // gather data if any
                line = String::new();
                match reader.read_line(&mut line) {
                    Ok(_) => {
                        if !line.trim().is_empty() {
                            event.data.push(line.trim().to_string());
                        } else {
                            break;
                        }
                    }
                    Err(e) => {
                        println!("error: {}", e);
                        break;
                    }
                };
            }
            // add to history
            history.push(event);
        }
    }
}

/// Add new event into history
pub fn push(path: &str, event: Event) -> bool {
    let log = OpenOptions::new().create(true).append(true).open(path);
    match log {
        Ok(log_file) => append_file(log_file, &vec![event]),
        Err(e) => {
            println!("push error: {}", e);
            return false;
        }
    }
}

/// Add new events into history (only new events)
pub fn _append(path: &str, events: &Vec<Event>) -> bool {
    let log = OpenOptions::new().create(true).append(true).open(path);
    match log {
        Ok(log_file) => append_file(log_file, events),
        Err(e) => {
            println!("append error: {}", e);
            return false;
        }
    }
}

fn append_file(file: File, events: &Vec<Event>) -> bool {
    // todo: is it be better to gather all like this or write one at a time?
    // guessing... each writer call must be os call, most likely syncronized
    // so it's better to gather all and write in big batch
    let mut buffer = String::with_capacity(1024);
    for event in events.iter() {
        buffer.push_str("\n");
        buffer.push_str(&event.name);
        buffer.push_str("\n");
        for data in event.data.iter() {
            buffer.push_str(&data);
            buffer.push_str("\n");
        }
    }
    let mut writer = BufWriter::new(file);
    match writer.write_all(buffer.as_bytes()) {
        Ok(_) => return true,
        Err(e) => {
            println!("append_file error: {}", e);
            
        }
    }       
    return false;     
}

#[cfg(test)]
mod tests {
    // input should be saved
    const TEST_FILE_INPUT: &str = "test_input.txt";
    // output re-created and deleted in testcase
    const TEST_FILE_OUTPUT: &str = "test_output.txt";
    use super::*;
    #[test]
    fn read_test() {
        let events = read(TEST_FILE_INPUT);
        assert_test_input_events(&events);
    }

    #[test]
    fn read_append_test() {
        if std::path::Path::new(TEST_FILE_OUTPUT).exists() {
            std::fs::remove_file(TEST_FILE_OUTPUT).unwrap();
        }
        let events = read(TEST_FILE_INPUT);
        assert!(_append(TEST_FILE_OUTPUT, &events));
        let events = read(TEST_FILE_OUTPUT);
        assert_test_input_events(&events);
        std::fs::remove_file(TEST_FILE_OUTPUT).unwrap();
    }
    
    fn assert_test_input_events(events: &Vec<Event>) {
        // 💖
        let heart = &events[0];
        assert!(heart.data.len() == 3);

        // 🐇
        let rabbit = &events[1];
        assert!(rabbit.data.len() == 1);

        // 🏆
        let trophy = &events[2];
        assert!(trophy.data.len() == 2);

        // 🍉
        let melon = &events[3];
        assert!(melon.data.len() == 0);

        // 🎈
        let balloon = &events[4];
        assert!(balloon.data.len() == 2);
        assert_eq!("🎈 🎈 🎈", balloon.data[1]);

        //🏀
        let ball = &events[5];
        assert!(ball.data.len() == 0);        
    }    
}

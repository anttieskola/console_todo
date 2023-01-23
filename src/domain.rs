use std::fmt;

pub struct TodoDomain {
    items: Vec<TodoItem>,
}

impl TodoDomain {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    pub fn _copy(&self) -> TodoDomain {
        let mut v = Vec::new();
        for i in self.items.iter() {
            v.push(i.clone());
        }
        TodoDomain { items: v }
    }

    pub fn command(self, cmd: DomainCmd) -> TodoDomain {
        let mut v: Vec<TodoItem> = Vec::new();
        match cmd {
            DomainCmd::Add(t) => {
                for i in self.items.iter() {
                    v.push(i.clone());
                }
                v.push(TodoItem::from(t.as_str()));
            }
            DomainCmd::Toggle(index) => {
                for (x, i) in self.items.iter().enumerate() {
                    if index != x {
                        v.push(i.clone());
                    }
                }
            }
        }
        return TodoDomain { items: v };
    }

}

impl fmt::Display for TodoDomain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, item) in self.items.iter().enumerate() {
            if let Err(e) = writeln!(f, "[{}]\t{}", i, item) {
                println!("Writing error: {}", e.to_string());
            }
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum DomainCmd {
    Add(String),
    Toggle(usize),
}

#[derive(Clone)]
pub struct TodoItem {
    text: String,
    done: bool,
}

impl TodoItem {
    pub fn from(str: &str) -> Self {
        Self {
            text: str.to_string(),
            done: false,
        }
    }
}

impl fmt::Display for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]\t{}", self.done, self.text)
    }
}

#[cfg(test)]
mod tests {
    //use super::*;
    #[test]
    fn test_two() {
        assert!(true);
    }
}
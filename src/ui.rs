use std::env;

pub(crate) fn greetings() {
    println!("Hello 👋 {}, what shall we do today?", get_username());
}

fn get_username() -> String {
    for (key, value) in env::vars() {
        if key == "USER" && !value.is_empty() {
            return value;
        }
    }
    return "unknown".to_string();
}
#[cfg(test)]
mod tests {
    use super::get_username;

    #[test]
    fn greetings() {
        crate::ui::greetings();
    }

    #[test]
    fn username() {
        let username = get_username();
        assert!(!username.is_empty());
    }
}

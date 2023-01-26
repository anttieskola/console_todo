mod application;
mod domain;
mod ui;
mod infra;

const HISTORY_LOG: &str = "console_todo.log";

fn main() {
    ui::start(HISTORY_LOG);
}

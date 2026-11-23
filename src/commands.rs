/// Обработка команд. Парсинг и выполнение. Возвращает `Vec<String>` в качестве вывода в игровую консоль
pub fn handle_command(cmd: &str) -> Vec<String> {
    let cmd = cmd.trim();

    match cmd {
        "" => vec![],
        "help" => vec![
            "help - shows command list.".to_string(),
            "exit - exit from game.".to_string(),
            "clear - clear history.".to_string()
        ],
        other => vec![format!("Unknown command: {}", other)]
    }
}
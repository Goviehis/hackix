pub struct App {
    pub active_tab: ActiveTab,
    pub should_quit: bool,
    pub console: ConsoleState
}

impl App {
    pub fn new() -> Self {
        Self {
            active_tab: ActiveTab::Console,
            should_quit: false,
            console: ConsoleState {
                history: vec!["NETRUNNER v0.1 IS READY".to_string(), "type 'help' for command list".to_string()],
                input: String::new(),
                scroll: 0
            }
        }
    }
}

pub enum ActiveTab {
    Console,
    Stats,
    Shop,
    Settings
}

impl ActiveTab {
    pub fn index(&self) -> usize {
        match self {
            Self::Console => 0,
            Self::Stats => 1,
            Self::Shop => 2,
            Self::Settings => 3
        }
    }
}

pub struct ConsoleState {
    history: Vec<String>,
    input: String,
    pub scroll: u16
}
impl ConsoleState {
    pub fn push_input(&mut self, c: char) {
        self.input.push(c);
    }

    pub fn pop_input(&mut self) {
        self.input.pop();
    }

    pub fn print(&mut self, lines: Vec<String>) {
        for line in lines {
            self.history.push(line);
        }
        self.scroll = 0;
    }

    pub fn get_input(&self) -> &str {
        &self.input
    }

    pub fn get_history(&self) -> &Vec<String> {
        &self.history
    }

    pub fn enter(&mut self) -> String {
        let cmd = std::mem::take(&mut self.input);

        self.print(vec![format!("> {}", cmd)]);
        cmd
    }

    pub fn clear(&mut self) {
        self.history.clear();
    }
}
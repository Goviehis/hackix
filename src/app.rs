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
                scroll: 0,
                last_visible_height: 0
            }
        }
    }
}
#[derive(PartialEq)]
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
    scroll: u16,
    pub last_visible_height: u16
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

    pub fn enter(&mut self) -> String {
        let cmd = std::mem::take(&mut self.input);

        self.print(vec![format!("> {}", cmd)]);
        cmd
    }

    pub fn clear(&mut self) {
        self.history.clear();
    }
    
    pub fn scroll_up(&mut self) {
        let max_scroll = self.history.len().saturating_sub(self.last_visible_height as usize);

        let temp_scroll = self.scroll.saturating_add(1);
        self.scroll = max_scroll.min(temp_scroll as usize) as u16;
    }

    pub fn scroll_down(&mut self) {
        self.scroll = self.scroll.saturating_sub(1);
    }

    pub fn visible_history(&self, visible_height: usize) -> Vec<&str> {
        let len = self.history.len();

        if len <= visible_height {
            return self.history.iter().map(|line| line.as_str()).collect()
        }

        let end = len.saturating_sub(self.scroll as usize);
        let start = end.saturating_sub(visible_height);

        self.history[start..end].iter().map(|line| line.as_str()).collect()
    }
}
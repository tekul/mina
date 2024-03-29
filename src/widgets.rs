use tui::widgets::ListState;

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        let mut list = StatefulList {
            state: ListState::default(),
            items,
        };
        list.state.select(Some(0));
        list
    }

    pub fn next(&mut self, n: usize) {
        let i = match self.state.selected() {
            Some(i) => {
                if i + n > self.items.len() - 1 {
                    0
                } else {
                    i + n
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self, n: usize) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else if i < n {
                    0
                } else {
                    i - n
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

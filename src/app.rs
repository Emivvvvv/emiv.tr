use crate::TAB_TITLES;

pub struct AppState {
    pub(crate) tab_index: usize,
}

impl AppState {
    pub(crate) fn new() -> Self {
        Self { tab_index: 0 }
    }

    pub(crate) fn next_tab(&mut self) {
        self.tab_index = (self.tab_index + 1) % TAB_TITLES.len();
    }

    pub(crate) fn previous_tab(&mut self) {
        if self.tab_index > 0 {
            self.tab_index -= 1;
        } else {
            self.tab_index = TAB_TITLES.len() - 1;
        }
    }
}

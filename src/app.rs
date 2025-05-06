use crate::TAB_TITLES;

pub struct AppState {
    pub(crate) tab_index: usize,
    pub(crate) should_animate: bool,
}

impl AppState {
    pub(crate) fn new() -> Self {
        Self {
            tab_index: 0,
            should_animate: true,
        }
    }

    pub(crate) fn next_tab(&mut self) {
        self.tab_index = (self.tab_index + 1) % TAB_TITLES.len();
        self.should_animate = true; // Trigger animation on tab change
    }

    pub(crate) fn previous_tab(&mut self) {
        if self.tab_index > 0 {
            self.tab_index -= 1;
        } else {
            self.tab_index = TAB_TITLES.len() - 1;
        }
        self.should_animate = true; // Trigger animation on tab change
    }
}

use crate::TAB_TITLES;

pub struct AppState {
    pub(crate) tab_index: usize,
    pub(crate) should_animate: bool,
    pub(crate) scroll_positions: [u16; TAB_TITLES.len()], // One position for each tab
    pub(crate) max_scroll: [u16; TAB_TITLES.len()],       // Track content heights for each tab
}

impl AppState {
    pub(crate) fn new() -> Self {
        Self {
            tab_index: 0,
            should_animate: true,
            scroll_positions: [0, 0, 0, 0],
            max_scroll: [0, 0, 0, 0],
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

    pub(crate) fn scroll_down(&mut self) {
        if self.scroll_positions[self.tab_index] < self.max_scroll[self.tab_index] {
            self.scroll_positions[self.tab_index] += 1;
        }
    }

    pub(crate) fn scroll_up(&mut self) {
        if self.scroll_positions[self.tab_index] > 0 {
            self.scroll_positions[self.tab_index] -= 1;
        }
    }

    pub(crate) fn update_max_scroll(&mut self, tab: usize, new_max: u16) {
        self.max_scroll[tab] = new_max;
    }
}

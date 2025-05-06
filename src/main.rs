use gridlife::{CellState, Grid};
use ratatui::prelude::{Direction, Span};
use ratatui::widgets::Tabs;
use ratzilla::event::KeyCode;
use ratzilla::ratatui::layout::{Constraint, Flex, Layout, Offset, Rect};
use ratzilla::ratatui::style::{Style, Stylize};
use ratzilla::ratatui::text::{Line, Text};
use ratzilla::ratatui::widgets::{BorderType, Clear, Wrap};
use ratzilla::ratatui::Frame;
use ratzilla::ratatui::{
    layout::Alignment,
    style::Color,
    widgets::{Block, Paragraph},
    Terminal,
};
use ratzilla::utils::is_mobile;
use ratzilla::widgets::Hyperlink;
use ratzilla::{DomBackend, WebRenderer};
use std::cell::RefCell;
use std::io;
use std::rc::Rc;

const BANNER: &str = r#"
88888888ba   88888888ba   88        88  88        88
88      "8b  88      "8b  88        88  88        88
88      ,8P  88      ,8P  88        88  88        88
88aaaaaa8P'  88aaaaaa8P'  88        88  88aaaaaaaa88
88""""""8b,  88""""88'    88        88  88""""""""88
88      `8b  88    `8b    88        88  88        88
88      a8P  88     `8b   Y8a.    .a8P  88        88
88888888P"   88      `8b   `"Y8888Y"'   88        88
"#;

const DESCRIPTION: &str = "Bruh";

const EDUCATION: &[(&str, &str, &str)] = &[
    ("Rizz University", "B.Sc. in Rizz Engineering", "DROPPED"),
    (
        "Ohio State University",
        "M.Sc. in Brainrot Concent Consumption",
        "Score: Congratulations",
    ),
];

const LINKS: &[(&str, &str)] = &[
    ("Free Robux", "https://github.com/Emivvvvv"),
    ("Fun Video", "https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
];

const TAB_TITLES: &[&str] = &["About Me", "Buggy Hyperlinks"];

struct AppState {
    tab_index: usize,
}

impl AppState {
    fn new() -> Self {
        Self { tab_index: 0 }
    }

    fn next_tab(&mut self) {
        self.tab_index = (self.tab_index + 1) % TAB_TITLES.len();
    }

    fn previous_tab(&mut self) {
        if self.tab_index > 0 {
            self.tab_index -= 1;
        } else {
            self.tab_index = TAB_TITLES.len() - 1;
        }
    }
}

fn main() -> io::Result<()> {
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;
    let size = terminal.size()?;
    let mut grid = Grid::new_random(size.width.into(), size.height.into());
    let app_state = Rc::new(RefCell::new(AppState::new()));

    // Set up key event handling
    terminal.on_key_event({
        let app_state = app_state.clone();
        move |event| {
            let mut state = app_state.borrow_mut();
            match event.code {
                KeyCode::Right => {
                    state.next_tab();
                }
                KeyCode::Left => {
                    state.previous_tab();
                }
                _ => {}
            }
        }
    });

    terminal.draw_web(move |frame| {
        render_game_of_life(&mut grid, frame);

        let app_state = app_state.clone();
        let borrowed_state = app_state.borrow();
        let area = if is_mobile() {
            let vertical = Layout::vertical([Constraint::Percentage(30)]).flex(Flex::Center);
            let horizontal = Layout::horizontal([Constraint::Percentage(80)]).flex(Flex::Center);
            let [area] = vertical.areas(frame.area());
            let [area] = horizontal.areas(area);
            area
        } else {
            let vertical = Layout::vertical([Constraint::Percentage(80)]).flex(Flex::Center);
            let horizontal = Layout::horizontal([Constraint::Percentage(60)]).flex(Flex::Center);
            let [area] = vertical.areas(frame.area());
            let [area] = horizontal.areas(area);
            area
        };

        // Render view
        let constraints = [
            Constraint::Length(BANNER.lines().count() as u16 + 1), // Banner
            Constraint::Length(3),                                 // Tabs
            Constraint::Length(20),                                // Contents
        ];
        render_background(frame, area, None, &constraints);

        // Split the area into sections
        let [banner_area, tabs_area, content_area] = Layout::vertical(constraints).areas(area);

        render_banner(frame, banner_area);
        // Render tabs
        let tabs = Tabs::new(
            TAB_TITLES
                .iter()
                .map(|t| Line::from(*t))
                .collect::<Vec<Line>>(),
        )
        .block(Block::bordered())
        .select(borrowed_state.tab_index)
        .highlight_style(Style::default().fg(Color::Gray));

        frame.render_widget(tabs, tabs_area);

        // Render content based on selected tab
        match borrowed_state.tab_index {
            0 => render_about_me_and_education(frame, content_area),
            1 => render_links(frame, content_area),
            _ => {}
        }
    });

    Ok(())
}

fn render_about_me_and_education(frame: &mut Frame, content_area: Rect) {
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(14), // whoami
            Constraint::Length(6),  // Education
        ])
        .split(content_area);

    render_whoami(frame, content_chunks[0]);
    render_education(frame, content_chunks[1]);
}

fn render_whoami(frame: &mut Frame<'_>, description_area: Rect) {
    frame.render_widget(
        Paragraph::new(DESCRIPTION)
            .wrap(Wrap { trim: true })
            .left_aligned()
            .block(Block::bordered().title("whoami".bold().underlined())),
        description_area,
    );
}

fn render_education(frame: &mut Frame<'_>, education_area: Rect) {
    let mut lines = Vec::new();

    for (university, field, gpa) in EDUCATION.iter() {
        let mut line = Line::default();
        line.spans
            .push(Span::styled(*university, Style::default().bold()));
        line.spans.push(Span::raw(", "));
        line.spans.push(Span::raw(*field));
        line.spans.push(Span::raw(", "));
        line.spans.push(Span::raw(*gpa));
        lines.push(line);
    }

    let text = Text::from(lines);

    frame.render_widget(
        Paragraph::new(text)
            .wrap(Wrap { trim: true })
            .left_aligned()
            .block(Block::bordered().title("Education".bold().underlined())),
        education_area,
    );
}

fn render_banner(frame: &mut Frame<'_>, banner_area: Rect) {
    frame.render_widget(
        Paragraph::new(BANNER).alignment(Alignment::Center),
        banner_area,
    );
}

fn render_links(frame: &mut Frame<'_>, content_area: Rect) {
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(14), // NOTHING
            Constraint::Length(6),  // Buggy Hyperlinks
        ])
        .split(content_area);

    frame.render_widget(
        Block::bordered().title("Links".bold().underlined()),
        content_chunks[1],
    );
    for (i, (_, url)) in LINKS.iter().enumerate() {
        let link = Hyperlink::new(*url);
        frame.render_widget(
            link,
            content_chunks[1].offset(Offset {
                x: 1,
                y: i as i32 + 1,
            }),
        );
    }
}

fn render_game_of_life(grid: &mut Grid<CellState>, frame: &mut Frame<'_>) {
    grid.update_states();
    let grid_out = grid.to_string();
    let lines: Vec<Line> = grid_out.lines().map(Line::from).collect();
    let grid_text = Text::from(lines).fg(Color::Rgb(100, 100, 100));
    frame.render_widget(Paragraph::new(grid_text), frame.area());
}

fn render_background(
    frame: &mut Frame<'_>,
    area: Rect,
    title: Option<String>,
    constraints: &[Constraint],
) {
    let mut area = Rect::new(
        area.x - 2,
        area.y - 1,
        area.width + 4,
        constraints
            .iter()
            .map(|c| match *c {
                Constraint::Min(v) | Constraint::Max(v) | Constraint::Length(v) => v,
                _ => 0,
            })
            .sum::<u16>()
            + 3,
    );
    area = area.clamp(frame.area());
    let mut block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Color::Rgb(220, 40, 50))
        .style(
            Style::default()
                .fg(Color::Rgb(220, 40, 50))
                .bg(Color::Rgb(16, 24, 39)),
        )
        .title_bottom("|built with Ratzilla|")
        .title_alignment(Alignment::Center);
    if let Some(title) = title {
        block = block.title_top(Line::from(title).alignment(Alignment::Center).bold());
    }
    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
}

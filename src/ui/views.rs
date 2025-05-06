use crate::data::FERRIS_RATATUI_AND_UNSAFE_FERRIS;
use crate::ui::{
    render_contributions, render_education, render_experiences, render_projects,
    render_publications, render_whoami,
};
use gridlife::{CellState, Grid};
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Line, Style, Stylize, Text};
use ratatui::widgets::{Block, BorderType, Clear, Paragraph};
use ratatui::Frame;

pub const BG_COLOR: Color = Color::Rgb(16, 24, 39);

pub fn render_about_me_and_education(frame: &mut Frame, content_area: Rect, scroll: u16) {
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(14), // whoami
            Constraint::Length(6),  // Education
        ])
        .split(content_area);

    render_whoami(frame, content_chunks[0], scroll);
    render_education(frame, content_chunks[1]);
}

pub fn render_projects_and_contributions(frame: &mut Frame, content_area: Rect, scroll: u16) {
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(14), // Projects
            Constraint::Length(6),  // Publications
        ])
        .split(content_area);

    render_projects(frame, content_chunks[0], scroll);
    render_contributions(frame, content_chunks[1]);
}

pub fn render_experiences_and_publications(frame: &mut Frame, content_area: Rect, scroll: u16) {
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(14), // Experience
            Constraint::Length(6),  // Publications
        ])
        .split(content_area);

    render_experiences(frame, content_chunks[0], scroll);
    render_publications(frame, content_chunks[1]);
}

pub fn render_ferris_ratatui_and_unsafe_ferris(
    frame: &mut Frame,
    long_live_ferris: Rect,
    scroll: u16,
) {
    frame.render_widget(
        Paragraph::new(FERRIS_RATATUI_AND_UNSAFE_FERRIS)
            .left_aligned()
            .scroll((0, scroll))
            .block(
                Block::bordered().title("Ferris, Ratatui and Unsafe Ferris".bold().underlined()),
            ),
        long_live_ferris,
    );
}

pub fn render_game_of_life(grid: &mut Grid<CellState>, frame: &mut Frame<'_>) {
    grid.update_states();
    let grid_out = grid.to_string();
    let lines: Vec<Line> = grid_out.lines().map(Line::from).collect();
    let grid_text = Text::from(lines).fg(Color::Rgb(100, 100, 100));
    frame.render_widget(Paragraph::new(grid_text), frame.area());
}

pub fn render_background(
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

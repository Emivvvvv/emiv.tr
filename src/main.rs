use gridlife::Grid;
use ratatui::widgets::Tabs;
use ratzilla::event::KeyCode;
use ratzilla::ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratzilla::ratatui::style::Style;
use ratzilla::ratatui::text::Line;
use ratzilla::ratatui::widgets::Wrap;
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

mod data;
use data::*;

mod app;
use app::*;

mod ui;
use ui::*;

const TAB_TITLES: &[&str] = &["About Me", "Projects", "Experiences", ":)"];

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
                KeyCode::Right | KeyCode::Char('l') | KeyCode::Char('L') => {
                    state.next_tab();
                }
                KeyCode::Left | KeyCode::Char('h') | KeyCode::Char('H') => {
                    state.previous_tab();
                }
                _ => {}
            }
        }
    });

    terminal.draw_web(move |frame| {
        render_game_of_life(&mut grid, frame);

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

        // Render appropriate view based on device
        if is_mobile() {
            render_mobile_view(frame, area);
        } else {
            let tab_index = app_state.borrow().tab_index;
            render_desktop_view(frame, area, tab_index);
        }
    });

    Ok(())
}

fn render_mobile_view(frame: &mut Frame, area: Rect) {
    let constraints = [
        Constraint::Length(MOBILE_INFO.lines().count() as u16 + 2),
        Constraint::Length(1),
    ];

    render_background(frame, area, None, &constraints);

    let [info_area, links_area] = Layout::vertical(constraints).areas(area);

    // Render mobile info message
    frame.render_widget(
        Paragraph::new(MOBILE_INFO)
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center),
        info_area,
    );

    // Render main website link
    frame.render_widget(Hyperlink::new("https://dev.emiv.online"), links_area);
}

fn render_desktop_view(frame: &mut Frame, area: Rect, tab_index: usize) {
    let constraints = [
        Constraint::Length(BANNER.lines().count() as u16 + 1), // Banner
        Constraint::Length(3),                                 // Tabs
        Constraint::Length(16),                                // Contents
        Constraint::Length(LINKS.len() as u16 + 2),            // Links
    ];
    render_background(frame, area, None, &constraints);

    // Split the area into sections
    let [banner_area, tabs_area, content_area, links_area] =
        Layout::vertical(constraints).areas(area);

    render_banner(frame, banner_area);
    // Render tabs
    let tabs = Tabs::new(
        TAB_TITLES
            .iter()
            .map(|t| Line::from(*t))
            .collect::<Vec<Line>>(),
    )
    .block(Block::bordered().title_bottom("< h|l >")
               .title_alignment(Alignment::Right))
    .select(tab_index)
    .highlight_style(Style::default().fg(Color::Gray));

    frame.render_widget(tabs, tabs_area);

    // Render content based on selected tab
    match tab_index {
        0 => render_about_me_and_education(frame, content_area),
        1 => render_projects_and_contributions(frame, content_area),
        2 => render_experiences_and_publications(frame, content_area),
        3 => render_ferris_ratatui_and_unsafe_ferris(frame, content_area),
        _ => {}
    }

    render_links(frame, links_area);
}

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
use tachyonfx::{Duration, EffectRenderer, Shader};

mod data;
use data::*;

mod app;
use app::*;

mod ui;
use ui::*;

mod animations;
use animations::*;

const TAB_TITLES: &[&str] = &["About Me", "Projects", "Experiences", ":)"];

fn main() -> io::Result<()> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;
    let size = terminal.size()?;
    let mut grid = Grid::new_random(size.width.into(), size.height.into());
    let app_state = Rc::new(RefCell::new(AppState::new()));

    // Define our effects
    let mut content_effect = CREATE_CONTENT_EFFECT();
    let mut banner_effect = CREATE_BANNER_EFFECT();

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
                KeyCode::Down | KeyCode::Char('j') | KeyCode::Char('J') => {
                    state.scroll_down();
                }
                KeyCode::Up | KeyCode::Char('k') | KeyCode::Char('K') => {
                    state.scroll_up();
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
            let mut state = app_state.borrow_mut();
            let tab_index = state.tab_index;

            // Changing max scroll depending on the area.width
            let tab_title = match area.width {
                0..=50 => {
                    for &i in &[0, 1] {
                        state.update_max_scroll(i, 2);
                    }
                    state.update_max_scroll(2, 20);
                    "<← h|l →, ↓ j|k ↑>"
                }
                51..=64 => {
                    for &i in &[0, 1, 2, 3] {
                        state.update_max_scroll(i, 0);
                    }
                    "<← h|l →>"
                }
                65..=100 => {
                    for &i in &[0, 1] {
                        state.update_max_scroll(i, 0);
                    }
                    state.update_max_scroll(2, 15);
                    state.update_max_scroll(3, 116 - area.width);
                    match tab_index {
                        2 | 3 => "<← h|l →, ↓ j|k ↑>",
                        _ => "<← h|l →>",
                    }
                }
                101..=115 => {
                    for &i in &[0, 1, 2] {
                        state.update_max_scroll(i, 0);
                    }
                    state.update_max_scroll(2, 3);
                    state.update_max_scroll(3, 116 - area.width);
                    match tab_index {
                        2 | 3 => "<← h|l →, ↓ j|k ↑>",
                        _ => "<← h|l →>",
                    }
                }
                _ => {
                    for &i in &[0, 1, 3] {
                        state.update_max_scroll(i, 0);
                    }
                    state.update_max_scroll(2, 3);
                    if tab_index == 2 {
                        "<← h|l →, ↓ j|k ↑>"
                    } else {
                        "<← h|l →>"
                    }
                }
            };

            // Check if we need to reset the animation
            if state.should_animate {
                content_effect = CREATE_CONTENT_EFFECT();
                state.should_animate = false;
            }

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
            if banner_effect.running() {
                frame.render_effect(&mut banner_effect, banner_area, Duration::from_millis(100));
            }

            // Render tabs
            let tabs = Tabs::new(
                TAB_TITLES
                    .iter()
                    .map(|t| Line::from(*t))
                    .collect::<Vec<Line>>(),
            )
            .block(
                Block::bordered()
                    .title_bottom(tab_title)
                    .title_alignment(Alignment::Right),
            )
            .select(tab_index)
            .highlight_style(Style::default().fg(Color::Gray));
            frame.render_widget(tabs, tabs_area);

            // Render content based on selected tab
            match tab_index {
                0 => render_about_me_and_education(frame, content_area, state.scroll_positions[0]),
                1 => render_projects_and_contributions(
                    frame,
                    content_area,
                    state.scroll_positions[1],
                ),
                2 => render_experiences_and_publications(
                    frame,
                    content_area,
                    state.scroll_positions[2],
                ),
                3 => render_ferris_ratatui_and_unsafe_ferris(
                    frame,
                    content_area,
                    state.scroll_positions[3],
                ),
                _ => {}
            }
            frame.render_effect(
                &mut content_effect,
                content_area,
                Duration::from_millis(100),
            );

            render_links(frame, links_area);
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

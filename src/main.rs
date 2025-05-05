use gridlife::{CellState, Grid};
use ratatui::prelude::Span;
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
use std::io;

const BANNER: &str = r#"
░        ░░  ░░░░  ░░        ░░  ░░░░  ░
▒  ▒▒▒▒▒▒▒▒   ▒▒   ▒▒▒▒▒  ▒▒▒▒▒  ▒▒▒▒  ▒
▓      ▓▓▓▓        ▓▓▓▓▓  ▓▓▓▓▓▓  ▓▓  ▓▓
█  ████████  █  █  █████  ███████    ███
█        ██  ████  ██        █████  ████
"#;

const DESCRIPTION: &str = r#"
Hi y’all, I'm Emirhan — a Rustacean v_(°v°)_v and senior Computer Engineering student at Yeditepe University, currently interning at smartPulse.

I’m into systems-level programming, distributed systems, P2P, and cryptography. Also a fan of F1, chess, photography, and cycling.
"#;

const EDUCATION: &[(&str, &str, &str)] = &[
    (
        "Yeditepe University",
        "B.Sc. in Computer Engineering",
        "GPA: 3.87",
    ),
    ("Yeditepe University", "Minor in Economics", "GPA: 3.70"),
    (
        "Hogeschool Utrecht",
        "Exchange Program in Social Robotics",
        "Score: 10/10",
    ),
];

const PROJECTS: &[(&str, &str)] = &[
    (
        "btc-vanity",
        "A Bitcoin vanity address generator library + CLI.",
    ),
    (
        "deloxide",
        "Scrubs your threads clean by detecting deadlocks in real time.",
    ),
    ("rlox-ast", "Lox language interpreter."),
];

const PUBLICATIONS: &[(&str, &str)] = &[(
    "Ethnical Anthropomorphism in Human-Robot Interaction: Personalized Robot Tutors",
    "37th Bled eConference, 2024",
)];

const LINKS: &[(&str, &str)] = &[
    ("GitHub", "https://github.com/Emivvvvv"),
    ("Website", "https://dev.emiv.online"),
];

const MOBILE_INFO: &str = r#"
See in desktop,
or go to main site
"#;

fn main() -> io::Result<()> {
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;
    let size = terminal.size()?;
    let mut grid = Grid::new_random(size.width.into(), size.height.into());

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
            let horizontal = Layout::horizontal([Constraint::Percentage(80)]).flex(Flex::Center);
            let [area] = vertical.areas(frame.area());
            let [area] = horizontal.areas(area);
            area
        };

        if is_mobile() {
            let constraints = [
                Constraint::Length(MOBILE_INFO.lines().count() as u16 + 2),
                Constraint::Length(1),
            ];

            render_background(
                frame,
                area,
                None,
                &constraints,
            );

            let [info_area, links_area] = Layout::vertical(constraints).areas(area);

            // Render mobile info message
            frame.render_widget(
                Paragraph::new(MOBILE_INFO)
                    .wrap(Wrap { trim: true })
                    .alignment(Alignment::Center),
                info_area,
            );

            // Render main website link
            frame.render_widget(
                Hyperlink::new("https://dev.emiv.online"),
                links_area,
            );
        } else {
            let description = textwrap::wrap(DESCRIPTION.trim(), area.width as usize - 15)
                .iter()
                .map(|line| line.to_string())
                .collect::<Vec<String>>()
                .join("\n");

            let constraints = [
                Constraint::Length(BANNER.lines().count() as u16 + 1),
                Constraint::Length(description.lines().count() as u16 + 2),
                Constraint::Length(EDUCATION.len() as u16 + 2),
                Constraint::Length(PROJECTS.len() as u16 + 2),
                Constraint::Length((PUBLICATIONS.len() * 2) as u16 + 2),
                Constraint::Length(LINKS.len() as u16 + 2),
            ];
            render_background(frame, area, None, &constraints);
            let [banner_area, description_area, education_area, projects_area, publications_area, links_area] =
                Layout::vertical(constraints).areas(area);
            render_banner(frame, banner_area);
            render_whoami(frame, description, description_area);
            render_education(frame, education_area);
            render_projects(frame, projects_area);
            render_publications(frame, publications_area);
            render_links(frame, links_area);
        }
    });

    Ok(())
}

fn render_game_of_life(grid: &mut Grid<CellState>, frame: &mut Frame<'_>) {
    grid.update_states();
    let grid_out = grid.to_string();
    let lines: Vec<Line> = grid_out.lines().map(Line::from).collect();
    let grid_text = Text::from(lines).fg(Color::Rgb(100, 100, 100));
    frame.render_widget(Paragraph::new(grid_text), frame.area());
}
fn render_banner(frame: &mut Frame<'_>, banner_area: Rect) {
    frame.render_widget(
        Paragraph::new(BANNER).alignment(Alignment::Center),
        banner_area,
    );
}

fn render_whoami(frame: &mut Frame<'_>, whoami_string: String, description_area: Rect) {
    frame.render_widget(
        Paragraph::new(whoami_string)
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

fn render_projects(frame: &mut Frame<'_>, projects_area: Rect) {
    let mut lines = Vec::new();

    for (name, description) in PROJECTS.iter() {
        let mut line = Line::default();
        line.spans
            .push(Span::styled(*name, Style::default().bold()));
        line.spans.push(Span::raw(": "));
        line.spans.push(Span::raw(*description));
        lines.push(line);
    }

    let text = Text::from(lines);

    frame.render_widget(
        Paragraph::new(text)
            .wrap(Wrap { trim: true })
            .left_aligned()
            .block(Block::bordered().title("Projects".bold().underlined())),
        projects_area,
    );
}

fn render_publications(frame: &mut Frame<'_>, publications_area: Rect) {
    let mut lines = Vec::new();

    for (paper, info) in PUBLICATIONS.iter() {
        // First line with paper title in bold
        let paper_line = Line::from(vec![Span::styled(*paper, Style::default().bold())]);
        lines.push(paper_line);

        // Second line with paper info
        let conference_line = Line::from(vec![Span::raw(*info)]);
        lines.push(conference_line);
    }

    let text = Text::from(lines);

    frame.render_widget(
        Paragraph::new(text)
            .wrap(Wrap { trim: true })
            .left_aligned()
            .block(Block::bordered().title("Publications".bold().underlined())),
        publications_area,
    );
}

fn render_links(frame: &mut Frame<'_>, links_area: Rect) {
    frame.render_widget(
        Block::bordered().title("Links".bold().underlined()),
        links_area,
    );
    for (i, (_, url)) in LINKS.iter().enumerate() {
        let link = Hyperlink::new(*url);
        frame.render_widget(
            link,
            links_area.offset(Offset {
                x: 1,
                y: i as i32 + 1,
            }),
        );
    }
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

use ratatui::prelude::Span;
use ratzilla::ratatui::layout::{Offset, Rect};
use ratzilla::ratatui::style::{Style, Stylize};
use ratzilla::ratatui::text::{Line, Text};
use ratzilla::ratatui::widgets::Wrap;
use ratzilla::ratatui::Frame;
use ratzilla::ratatui::{
    layout::Alignment,
    widgets::{Block, Paragraph},
};
use ratzilla::widgets::Hyperlink;

use crate::data::*;

pub fn render_whoami(frame: &mut Frame<'_>, description_area: Rect, scroll: u16) {
    frame.render_widget(
        Paragraph::new(DESCRIPTION)
            .wrap(Wrap { trim: true })
            .left_aligned()
            .scroll((scroll, 0)) // Add scrolling using the scroll position
            .block(Block::bordered().title("whoami".bold().underlined())),
        description_area,
    );
}

pub fn render_education(frame: &mut Frame<'_>, education_area: Rect) {
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

pub fn render_projects(frame: &mut Frame<'_>, projects_area: Rect, scroll: u16) {
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
            .scroll((scroll, 0))
            .block(Block::bordered().title("Projects".bold().underlined())),
        projects_area,
    );
}

pub fn render_contributions(frame: &mut Frame<'_>, contributions_area: Rect) {
    let mut lines = Vec::new();

    for (project, description, commits) in CONTRIBUTIONS.iter() {
        let mut line = Line::default();
        line.spans
            .push(Span::styled(*project, Style::default().bold()));
        line.spans.push(Span::raw(", "));
        line.spans.push(Span::raw(*description));
        line.spans.push(Span::raw(", "));
        line.spans.push(Span::raw(*commits));
        lines.push(line);
    }

    let text = Text::from(lines);

    frame.render_widget(
        Paragraph::new(text)
            .wrap(Wrap { trim: true })
            .left_aligned()
            .block(Block::bordered().title("Contributions".bold().underlined())),
        contributions_area,
    );
}

pub fn render_experiences(frame: &mut Frame<'_>, projects_area: Rect, scroll: u16) {
    let mut lines = Vec::new();

    for (company, title, duration, description) in EXPERIENCES.iter() {
        let mut line = Line::default();
        line.spans
            .push(Span::styled(*company, Style::default().bold()));
        line.spans.push(Span::raw(", "));
        line.spans.push(Span::raw(*title));
        line.spans.push(Span::raw(", "));
        line.spans.push(Span::raw(*duration));
        lines.push(line);

        lines.push(Line::from(format!("• {}", description)));
        lines.push(Line::default());
    }
    let text = Text::from(lines);

    frame.render_widget(
        Paragraph::new(text)
            .wrap(Wrap { trim: true })
            .left_aligned()
            .scroll((scroll, 0))
            .block(Block::bordered().title("Experiences".bold().underlined())),
        projects_area,
    );
}

pub fn render_publications(frame: &mut Frame<'_>, publications_area: Rect) {
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

pub fn render_banner(frame: &mut Frame<'_>, banner_area: Rect) {
    frame.render_widget(
        Paragraph::new(BANNER).alignment(Alignment::Center),
        banner_area,
    );
}

pub fn render_links(frame: &mut Frame<'_>, links_area: Rect) {
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

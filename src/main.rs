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
░        ░░  ░░░░  ░░        ░░  ░░░░  ░
▒  ▒▒▒▒▒▒▒▒   ▒▒   ▒▒▒▒▒  ▒▒▒▒▒  ▒▒▒▒  ▒
▓      ▓▓▓▓        ▓▓▓▓▓  ▓▓▓▓▓▓  ▓▓  ▓▓
█  ████████  █  █  █████  ███████    ███
█        ██  ████  ██        █████  ████
"#;

const DESCRIPTION: &str = r#"Hi y’all, I'm Emirhan — a Rustacean v_(°v°)_v and senior Computer Engineering student at Yeditepe University, currently interning at smartPulse.

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
        "Bitcoin vanity address generator library + CLI. (+25,000 downloads!!)",
    ),
    (
        "deloxide",
        "Scrubs your threads clean with real-time deadlock detection and built-in log insights.",
    ),
    ("rlox-ast", "Lox language interpreter."),
    ("RustyChain", "Basic blockchain implementation."),
    ("AmongOS", "Small sussy operating system."),
];

const CONTRIBUTIONS: &[(&str, &str, &str)] = &[(
    "Ratatui",
    "A Rust crate for cooking up terminal user interfaces (TUIs)",
    "13 Commits",
)];

const EXPERIENCES: &[(&str, &str, &str, &str)] = &[
    (
        "smartPulse Technology",
        "Connectivity Intern",
        "March 2025 – Still",

            "Implemented a internal device setup tool from start to finish using Rust and Ratatui.",
    ),
    (
        "HyperHawk Hyperloop Team",
        "Co-Head of SWE Department",
        "October 2024 – Still",

            "Directed the project's development strategy and structure, driving key decision-making processes.",
    ),

];

const PUBLICATIONS: &[(&str, &str)] = &[(
    "Ethnical Anthropomorphism in Human-Robot Interaction: Personalized Robot Tutors",
    "37th Bled eConference, 2024",
)];

const LINKS: &[(&str, &str)] = &[
    ("GitHub", "https://github.com/Emivvvvv"),
    ("Website", "https://dev.emiv.online"),
];

const MOBILE_INFO: &str = r#"Not mobile-friendly

please use a desktop or
visit the mobile-friendly site.
"#;

const FERRIS_RATATUI_AND_UNSAFE_FERRIS: &str = r#"                    +++ ++++++                           .+%@@%.                   .    .. :  .. .             
               + +++++++++++++++++                     .+@@@@@@.                 ..   =. .---.=. +..:.=           
              +++++++++++++++++++++++        ++       -%@@@@@@@:              .   .=...=.:+-=--:+--=.=..--:.      
 ++++++    ++++++++++++++++++++++++++++    ++++ ++   .*@@@@@@@@@=   ......     .=.: :=:==:===++*+++*++=.-=--.     
++++++++ +++++++++++++++++++++++++++++++++++++++++    ....#@@@@%*#%@@@@@@*     ..:=+:+==*++*=+*+*****++.=...      
++++++++ ++++++++++++++++++++++++++++++++*+++++++           =#-%= +@@@@@@-  .-::-==+=+*+*****+*****=*+**++=+..:.   I can also add scrolling here
 +++++++++++++++++++++++++++++++++++++++++++++++        .#@@@@@@@@@@@@#:   :==..=*+++*****************=****+:.     for smaller screens.
    ++++++++++++++++=.+@#++*=.%#++++++++++++++         .-%@@@@@@@@@@#:       .-+**++***********************+*+=..  Not everyone might want to
      ++++++++++++++#@@@@++#@@@@++++++++++++         .*=.%: =@@@@@@#:     .:-==-=+************************==+-:    see Unsafe Ferris though :)
     ++++***+++++++++*%#*+++*##+++++++****++++     .++ .%: %+@@@@@@#:      -=-:-****************************+::.
      +++*####*****++++++#@#*+++****#####*+++    .=+   :-   .%*@@@@@@@%.     .:=++*****@*%**@+%*************-..
       ++++##     ################    ###+++   .=*      +=    .##%@#+*@:    ..:=++***********#***************+=-:.
         +++ #                        # ++*    %=               :#=..+%.       ..-**********************-+***+-.  
           ++                          ++       .*-               -**#:             .. .-==-=:-......      ..     
"#;

const TAB_TITLES: &[&str] = &["About Me", "Projects", "Experiences", ":)"];

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
    .block(Block::bordered())
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

fn render_projects_and_contributions(frame: &mut Frame, content_area: Rect) {
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(14), // Projects
            Constraint::Length(6),  // Publications
        ])
        .split(content_area);

    render_projects(frame, content_chunks[0]);
    render_contributions(frame, content_chunks[1]);
}

fn render_experiences_and_publications(frame: &mut Frame, content_area: Rect) {
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(14), // Experience
            Constraint::Length(6),  // Publications
        ])
        .split(content_area);

    render_experiences(frame, content_chunks[0]);
    render_publications(frame, content_chunks[1]);
}

fn render_ferris_ratatui_and_unsafe_ferris(frame: &mut Frame, long_live_ferris: Rect) {
    frame.render_widget(
        Paragraph::new(FERRIS_RATATUI_AND_UNSAFE_FERRIS)
            .left_aligned()
            .block(
                Block::bordered().title("Ferris, Ratatui and Unsafe Ferris".bold().underlined()),
            ),
        long_live_ferris,
    );
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

fn render_contributions(frame: &mut Frame<'_>, contributions_area: Rect) {
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

fn render_experiences(frame: &mut Frame<'_>, projects_area: Rect) {
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
    lines.push(Line::from("Imma add scrolling here. There's just not enough space man.").bold());

    let text = Text::from(lines);

    frame.render_widget(
        Paragraph::new(text)
            .wrap(Wrap { trim: true })
            .left_aligned()
            .block(Block::bordered().title("Experiences".bold().underlined())),
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

fn render_banner(frame: &mut Frame<'_>, banner_area: Rect) {
    frame.render_widget(
        Paragraph::new(BANNER).alignment(Alignment::Center),
        banner_area,
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

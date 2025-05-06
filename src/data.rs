pub const BANNER: &str = r#"
░        ░░  ░░░░  ░░        ░░  ░░░░  ░
▒  ▒▒▒▒▒▒▒▒   ▒▒   ▒▒▒▒▒  ▒▒▒▒▒  ▒▒▒▒  ▒
▓      ▓▓▓▓        ▓▓▓▓▓  ▓▓▓▓▓▓  ▓▓  ▓▓
█  ████████  █  █  █████  ███████    ███
█        ██  ████  ██        █████  ████
"#;

pub const DESCRIPTION: &str = r#"Hi y’all, I'm Emirhan — a Rustacean v_(°v°)_v and senior Computer Engineering student at Yeditepe University, currently interning at smartPulse.

I’m into systems-level programming, distributed systems, P2P, and cryptography. Also a fan of F1, chess, photography, and cycling.
"#;

pub const EDUCATION: &[(&str, &str, &str)] = &[
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

pub const PROJECTS: &[(&str, &str)] = &[
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

pub const CONTRIBUTIONS: &[(&str, &str, &str)] = &[(
    "Ratatui",
    "A Rust crate for cooking up terminal user interfaces (TUIs)",
    "13 Commits",
)];

pub const EXPERIENCES: &[(&str, &str, &str, &str)] = &[
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

pub const PUBLICATIONS: &[(&str, &str)] = &[(
    "Ethnical Anthropomorphism in Human-Robot Interaction: Personalized Robot Tutors",
    "37th Bled eConference, 2024",
)];

pub const LINKS: &[(&str, &str)] = &[
    ("GitHub", "https://github.com/Emivvvvv"),
    ("Website", "https://dev.emiv.online"),
];

pub const MOBILE_INFO: &str = r#"Not mobile-friendly

please use a desktop or
visit the mobile-friendly site.
"#;

pub const FERRIS_RATATUI_AND_UNSAFE_FERRIS: &str = r#"                    +++ ++++++                           .+%@@%.                   .    .. :  .. .
               + +++++++++++++++++                     .+@@@@@@.                 ..   =. .---.=. +..:.=
              +++++++++++++++++++++++        ++       -%@@@@@@@:              .   .=...=.:+-=--:+--=.=..--:.
 ++++++    ++++++++++++++++++++++++++++    ++++ ++   .*@@@@@@@@@=   ......     .=.: :=:==:===++*+++*++=.-=--.
++++++++ +++++++++++++++++++++++++++++++++++++++++    ....#@@@@%*#%@@@@@@*     ..:=+:+==*++*=+*+*****++.=...
++++++++ ++++++++++++++++++++++++++++++++*+++++++           =#-%= +@@@@@@-  .-::-==+=+*+*****+*****=*+**++=+..:. 
 +++++++++++++++++++++++++++++++++++++++++++++++        .#@@@@@@@@@@@@#:   :==..=*+++*****************=****+:.   
    ++++++++++++++++=.+@#++*=.%#++++++++++++++         .-%@@@@@@@@@@#:       .-+**++***********************+*+=..
      ++++++++++++++#@@@@++#@@@@++++++++++++         .*=.%: =@@@@@@#:     .:-==-=+************************==+-:  
     ++++***+++++++++*%#*+++*##+++++++****++++     .++ .%: %+@@@@@@#:      -=-:-****************************+::.
      +++*####*****++++++#@#*+++****#####*+++    .=+   :-   .%*@@@@@@@%.     .:=++*****@*%**@+%*************-..
       ++++##     ################    ###+++   .=*      +=    .##%@#+*@:    ..:=++***********#***************+=-:.
         +++ #                        # ++*    %=               :#=..+%.       ..-**********************-+***+-.
           ++                          ++       .*-               -**#:             .. .-==-=:-......      ..
"#;

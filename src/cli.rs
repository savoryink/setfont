use clap::{App, AppSettings, Arg, SubCommand};

const NAME: &str = env!("CARGO_PKG_NAME");

const AUTHOR: &str = "SavoryInk (https://savory.ink)";

const VERSION: &str = concat!("v", env!("CARGO_PKG_VERSION"));

const ABOUT: &str = "
Sets the font of editors and terminals\
";

pub fn app() -> App<'static, 'static> {
    App::new(NAME)
        .author(AUTHOR)
        .version(VERSION)
        .about(ABOUT)
        .settings(&[
            AppSettings::ArgRequiredElseHelp,
            AppSettings::DeriveDisplayOrder,
            AppSettings::SubcommandsNegateReqs,
        ])
        .arg(
            Arg::with_name("apps")
                .help("The applications to set the primary font of")
                .value_name("APP")
                .short("a")
                .long("apps")
                .takes_value(true)
                .multiple(true)
                .required(true),
        )
        .arg(
            Arg::with_name("font")
                .help("The primary font to set")
                .value_name("FONT")
                .short("f")
                .long("font")
                .takes_value(true)
                .required_unless_one(&["ligatures", "no-ligatures"]),
        )
        .arg(
            Arg::with_name("ligatures")
                .help("Enables orthographic ligatures")
                .short("l")
                .long("ligatures"),
        )
        .arg(
            Arg::with_name("no-ligatures")
                .long("no-ligatures")
                .overrides_with("ligatures")
                .hidden(true),
        )
        .subcommand(
            SubCommand::with_name("list")
                .alias("ls")
                .about("List all supported apps"),
        )
}

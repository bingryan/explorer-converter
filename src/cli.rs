use clap::{crate_authors, crate_description, crate_version, App, AppSettings, Arg, SubCommand};


pub fn build_cli() -> App<'static, 'static> {
    App::new("explorer")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .subcommands(vec![
            SubCommand::with_name("producer")
                .about("explorer producer command"),
            SubCommand::with_name("consumer")
                .about("explorer consumer command"),
        ])
}
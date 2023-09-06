use clap::{arg, Arg, Command};

extern crate cr8s;

fn main() {
    let matches = Command::new("Cr8s")
        .about("Cr8s CLI")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("users")
                .about("Cr8s user management")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("create")
                        .about("Create user with multiple roles attached")
                        .arg_required_else_help(true)
                        .arg(Arg::new("username").required(true))
                        .arg(Arg::new("password").required(true))
                        .arg(
                            Arg::new("roles")
                                .required(true)
                                .num_args(1..)
                                .value_delimiter(','),
                        ),
                )
                .subcommand(Command::new("list").about("List all available users"))
                .subcommand(
                    Command::new("delete").about("Delete user by ID").arg(
                        Arg::new("id")
                            .required(true)
                            .value_parser(clap::value_parser!(i32)),
                    ),
                ),
        )
        .subcommand(
            Command::new("digest-send")
                .about("Send an email with the  newest crates")
                .arg(
                    Arg::new("to")
                        .required(true)
                        .num_args(1..2)
                        .value_delimiter(','),
                )
                .arg(
                    Arg::new("hours_since")
                        .required(true)
                        .value_parser(clap::value_parser!(i32)),
                )
                .arg(
                    arg!([subject])
                        .required(false)
                        .default_value(None)
                        .value_parser(clap::value_parser!(Option<String>)),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("users", sub_matches)) => match sub_matches.subcommand() {
            Some(("create", sub_matches)) => cr8s::commands::create_user(
                sub_matches
                    .get_one::<String>("username")
                    .unwrap()
                    .to_owned(),
                sub_matches
                    .get_one::<String>("password")
                    .unwrap()
                    .to_owned(),
                sub_matches
                    .get_many::<String>("roles")
                    .unwrap()
                    .map(|v| v.to_string())
                    .collect(),
            ),
            Some(("list", _)) => cr8s::commands::list_users(),
            Some(("delete", sub_matches)) => {
                cr8s::commands::delete_user(sub_matches.get_one::<i32>("id").unwrap().to_owned())
            }
            _ => {}
        },
        Some(("digest-send", sub_matches)) => cr8s::commands::send_digest(
            sub_matches
                .get_many::<String>("to")
                .unwrap()
                .map(|v| v.to_string())
                .collect(),
            sub_matches
                .get_one::<i32>("hours_since")
                .unwrap()
                .to_owned(),
            sub_matches.get_one::<String>("subject").cloned(),
        ),
        _ => {}
    }
}

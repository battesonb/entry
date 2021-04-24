mod args;
mod errors;
mod schema;

use args::{Command, Config, Entry, SchemaCommand};
use schema::Schema;

use exitfailure::ExitFailure;
use failure::{err_msg, ResultExt};
use std::{
    fs,
    io::{BufRead, BufReader},
};
use structopt::StructOpt;

fn main() -> Result<(), ExitFailure> {
    let mut config: Config = confy::load("entry")?;
    let args = Entry::from_args();
    match args.cmd {
        Command::Schema { cmd: subcommand } => match subcommand {
            SchemaCommand::List => {}
            _ => todo!("schema command"),
        },
        _ => todo!("command"),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;

    #[test]
    fn invalid_minute() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("entry").unwrap();
        cmd.arg("new");
        cmd.arg("-t 12:60");
        cmd.assert()
            .failure()
            .stderr(predicates::str::contains("invalid minute input"));
        Ok(())
    }

    #[test]
    fn invalid_hour() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("entry").unwrap();
        cmd.arg("new");
        cmd.arg("-t 24:00");
        cmd.assert()
            .failure()
            .stderr(predicates::str::contains("invalid hour input"));
        Ok(())
    }

    #[test]
    fn invalid_hour_for_meridian() -> Result<(), Box<dyn std::error::Error>> {
        let mut cmd = Command::cargo_bin("entry").unwrap();
        cmd.arg("new");
        cmd.arg("-t 13:00am");
        cmd.assert().failure().stderr(predicates::str::contains(
            "invalid hour input for meridiem time",
        ));
        Ok(())
    }
}

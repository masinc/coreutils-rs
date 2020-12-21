mod lib;

use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg};
use lib::LsEntry;

const CURRENT_DIR: &str = ".";
const ARG_DIRS: &str = "DIRS";

fn main() {
    let matches = app_from_crate!()
        .arg(Arg::with_name(ARG_DIRS).multiple(true))
        .get_matches();

    let arg_dirs: Vec<_> = matches
        .values_of(ARG_DIRS)
        .map_or_else(|| vec![CURRENT_DIR], |x| x.collect());

    let entries = arg_dirs.into_iter().map(LsEntry::new);

    for entry in entries {
        let children = entry.entries();

        match children {
            Err(err) => eprintln!("{}", err),
            Ok(children) => {
                for child_entry in children {
                    match child_entry {
                        Err(err) => eprintln!("{}", err),
                        Ok(child_entry) => {
                            println!("{}", child_entry.to_string());
                        }
                    }
                }
            }
        }
    }
}

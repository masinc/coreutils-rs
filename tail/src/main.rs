mod lib;

use anyhow::Result;

use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg};
use lib::InputContent;
use std::iter::once;

const ARG_FILES: &str = "FILES";

fn main() -> Result<()> {
    let matches = app_from_crate!()
        .arg(Arg::with_name(ARG_FILES).multiple(true))
        .get_matches();

    let arg_files: Vec<_> = matches
        .values_of(ARG_FILES)
        .map_or_else(|| vec![], |x| x.collect());

    let contents: Vec<_> = match arg_files.len() {
        // stdin
        0 => once(InputContent::from_stdin())
            .map(|x| x.unwrap().read_lines_default())
            .collect(),
        // files
        _ => arg_files
            .into_iter()
            .map(InputContent::from_file)
            .map(|x| x.unwrap().read_lines_default())
            .collect(),
    };

    for content in contents {
        content?.into_iter().for_each(|x| println!("{}", x));
    }

    Ok(())
}

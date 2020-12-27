mod lib;

use std::iter::once;

use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg};
use lib::InputContent;

const DEFAULT_LINE_COUNT: usize = 10;

const ARG_FILES: &str = "FILES";

fn main() {
    let matches = app_from_crate!()
        .arg(Arg::with_name(ARG_FILES).multiple(true))
        .get_matches();

    let arg_files: Vec<_> = matches
        .values_of(ARG_FILES)
        .map_or_else(|| vec![], |x| x.collect());

    let contents: Vec<_> = match arg_files.len() {
        // stdin
        0 => once(InputContent::from_stdin()).collect(),
        // files
        _ => arg_files
            .into_iter()
            .map(InputContent::from_file_name)
            .collect(),
    };

    for content in contents {
        match content.read_lines(DEFAULT_LINE_COUNT) {
            Err(e) => {
                eprintln!("{:?}: {:?} {}", content, e.kind(), e.to_string())
            }
            Ok(lines) => {
                for line in lines {
                    match line {
                        Err(e) => eprintln!("{}", e),
                        Ok(line) => {
                            print!("{}", line);
                        }
                    }
                }
            }
        }
    }
}

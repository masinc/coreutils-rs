mod lib;

use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg};
use lib::FileContent;
use std::iter::once;

const ARG_FILES: &str = "FILES";

fn main() {
    let matches = app_from_crate!()
        .arg(Arg::with_name(ARG_FILES).multiple(true))
        .get_matches();

    let arg_files: Vec<_> = matches
        .values_of(ARG_FILES)
        .map_or_else(|| vec![], |x| x.collect());

    let contents: Vec<FileContent> = match arg_files.len() {
        // stdin
        0 => once(FileContent::from_stdin()).collect(),
        // files
        _ => arg_files
            .into_iter()
            .map(|f| FileContent::from_file_name(f))
            .collect(),
    };

    for content in contents {
        match &content.read() {
            Ok(body) => print!("{}", String::from_utf8_lossy(body)),
            Err(err) => eprintln!("{:?}: {:?} {}", content, err.kind(), err.to_string()),
        }
    }
}

mod lib;

use clap::{app_from_crate, crate_authors, crate_description, crate_name, crate_version, Arg};
use lib::parse_strings;

const ARG_STRING: &str = "STRING";

fn main() {
    let matches = app_from_crate!()
        .arg(Arg::with_name(ARG_STRING).multiple(true))
        .get_matches();

    let arg_strings = matches
        .values_of(ARG_STRING)
        .map_or(Vec::new(), |x| x.collect());

    println!("{}", parse_strings(&mut arg_strings.into_iter()));
}

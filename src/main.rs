extern crate clap;

use clap::{App, Arg};

fn main() {
    let app = App::new("bulkro")
        .version("1.0")
        .author("notkild <notkild@gmail.com>")
        .about("Bulk renamer and organizer")
        .arg(Arg::with_name("files")
            .value_name("FILE")
            .takes_value(true)
            .multiple(true)
            .required(true)
            .help("Set the file to rename"))
        .arg(Arg::with_name("format")
            .value_name("FORMAT")
            .takes_value(true)
            .required(true)
            .help("Set files format"))
        .get_matches();

    let files = app.values_of("files").unwrap();
    let format = app.value_of("format").unwrap();
}

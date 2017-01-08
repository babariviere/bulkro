extern crate bulkro;
extern crate clap;

use bulkro::Formatter;
use clap::{App, Arg};
use std::fs::{create_dir_all, rename};

fn main() {
    let app = App::new("bulkro")
        .version("1.0")
        .author("notkild <notkild@gmail.com>")
        .about("Bulk renamer and organizer")
        .arg(Arg::with_name("format")
            .value_name("FORMAT")
            .takes_value(true)
            .required(true)
            .help("Set files format"))
        .arg(Arg::with_name("files")
            .value_name("FILE")
            .takes_value(true)
            .multiple(true)
            .required(true)
            .help("Set the file to rename"))
        .get_matches();

    let files = app.values_of("files").unwrap().collect::<Vec<&str>>();
    let format = app.value_of("format").unwrap();
    let formatter = Formatter::new(&format, &files);
    if format.contains('/') {
        let idx = format.rfind('/').unwrap();
        create_dir_all(&format[0..idx]).expect(&format!("Failed to create {}", &format[0..idx]));
    }
    for (i, f) in formatter.enumerate() {
        let file = files[i].clone();
        println!("{} => {}", file, f);
        rename(file, f).expect(&format!("Failed to rename {}", file));
    }
}

use chrono::Datelike;
use chrono::Local;
use clap::{App, Arg};

fn main() {
    let matches = App::new("The Amazing Greeter")
        .version("0.1.0")
        .author("Tom Hartman")
        .about("Setup new esqurie site")
        .arg(
            Arg::with_name("name")
                .short("n")
                .long("name")
                .value_name("NAME")
                .help("Client Name")
                .takes_value(true)
                .default_value("esquire"),
        )
        .get_matches();

    let name = matches.value_of("name").unwrap();
    let year = Local::now().year().to_string();

    let name = format!("{}{}", name, year);

    println!("Name: {}", name);
}

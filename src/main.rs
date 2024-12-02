use clap::{value_parser, Arg, Command};

mod days;

fn main() {
    let matches = Command::new("Advent of Code 2024")
        .version("1.0")
        .about("Run specific day solutions")
        .arg(
            Arg::new("day")
                .short('d')
                .long("day")
                .help("The day number to run")
                .value_parser(value_parser!(u32))
                .required(true),
        )
        .get_matches();

    let day: u32 = match matches.get_one::<u32>("day") {
        Some(&day_num) => day_num,
        None => {
            eprintln!("Day argument is missing.");
            std::process::exit(1);
        }
    };

    println!("🎄 Advent of Code 2024: day {} 🎄", day);
    match day {
        1 => days::day01::run(),
        2 => days::day02::run(),
        _ => println!("Day not implemented yet!"),
    }
}

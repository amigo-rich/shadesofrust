use clap::{App, Arg, SubCommand};
use shadesofrust::{operation::Operation, run};

fn main() {
    let app = App::new("Shades of rust")
        .version("0.1")
        .author("Richard Bradshaw")
        .about("Set the display brightness")
        .subcommand(
            SubCommand::with_name("get")
                .about("Retrieve the current brightness")
                .arg(
                    Arg::with_name("path")
                        .long("path")
                        .required(true)
                        .help("The sysfs path of the device. e.g. /sys/class/backlight/amdgpu_bl0/")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("set")
                .about("Set the current brightness")
                .arg(
                    Arg::with_name("path")
                        .long("path")
                        .required(true)
                        .help("The sysfs path of the device. e.g. /sys/class/backlight/amdgpu_bl0/")
                        .takes_value(true),


                )
                .arg(
                    Arg::with_name("brightness")
                    .long("brightness")
                    .required(true)
                    .help("The new brightness value. This should be between 0 and the maximum brightness")
                    .takes_value(true),
                )
        );
    let arg_matches = app.get_matches();

    let operation = match arg_matches.subcommand() {
        ("get", Some(sc_matches)) => {
            Operation::Get(std::path::Path::new(sc_matches.value_of("path").unwrap()))
        }
        ("set", Some(sc_matches)) => Operation::Set(
            std::path::Path::new(sc_matches.value_of("path").unwrap()),
            sc_matches.value_of("brightness").unwrap(),
        ),
        _ => {
            eprintln!("For usage, see shadesofrust --help");
            return;
        }
    };

    if let Err(e) = run(operation) {
        eprintln!("{}", e);
    }
}

use clap::{Command, Arg, ArgAction};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let m = Command::new("echor")
        .version("0.1.0")
        .author("Katsunori Kanda, potix2@gmail.com")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
                .num_args(0)
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let ending = if m.get_flag("omit_newline") { "" } else { "\n" };
    let text = m
        .get_many::<String>("text")
        .unwrap()
        .cloned()
        .collect::<Vec<_>>()
        .join(" ");
    print!("{}{}", text, ending);
    Ok(())
}

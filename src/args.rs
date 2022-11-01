use clap::{arg, Command, ArgMatches};

use crate::commands::{encode, decode, remove};

pub fn cli() -> Command {
    Command::new("spng")
        .about("Hide your message with PNG image")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(encode_cli())
        .subcommand(decode_cli())
        .subcommand(remove_cli())
        // .subcommand(print_cli())
}

fn encode_cli() -> Command {
    Command::new("encode")
        .about("Encode a message to PNG file")
        .arg(arg!(<FILEPATH> "Path to PNG file"))
        .arg(arg!(<SIG> "PNG's signature"))
        .arg(arg!(<MESSAGE> "Message"))
}

fn decode_cli() -> Command {
    Command::new("decode")
        .about("Decode a PNG file to get the message")
        .arg(arg!(<FILEPATH> "Path to PNG file"))
        .arg(arg!(<SIG> "PNG's signature"))
}

fn remove_cli() -> Command{
    Command::new("remove")
        .about("Remove a message from PNG file")
        .arg(arg!(<FILEPATH> "Path to PNG file"))
        .arg(arg!(<SIG> "PNG's signature"))
}

// fn print_cli() -> Command {
//     Command::new("print")
//         .about("Print a list of secret messages")
//         .arg(arg!(<FILEPATH> "Path to PNG file"))
// }

pub fn matches() -> ArgMatches {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("encode", sub_matches)) => {
            let path = sub_matches
                .get_one::<String>("FILEPATH").expect("required");
            let sig = sub_matches
                .get_one::<String>("SIG").expect("required");
            let message = sub_matches
                .get_one::<String>("MESSAGE").expect("required");
            println!(
                "Encoding {} with a message {} ({})",
                path,
                message,
                sig
            );
            encode(path, sig, message);
        }
        Some(("decode", sub_matches)) => {
            let path = sub_matches
                .get_one::<String>("FILEPATH").expect("required");
            let sig = sub_matches
                .get_one::<String>("SIG").expect("required");
            println!(
                "Decoding {} ({})",
                path,
                sig
            );

            let message = decode(path, sig).unwrap();
            println!("Message: {}", message)
        }
        Some(("remove", sub_matches)) => {
            let path = sub_matches
                .get_one::<String>("FILEPATH").expect("required");
            let sig = sub_matches
                .get_one::<String>("SIG").expect("required");
            println!(
                "Removing {} ({})",
                path,
                sig
            );

            let message = remove(path, sig).unwrap();
            println!("Removed: {}", message)
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }

    matches
}

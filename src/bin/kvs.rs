use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    long_about = None
)]
struct Args {
    commands: Vec<String>,
}

fn main() {
    let args = Args::parse();

    if args.commands.len() == 0 {
        panic!()
    } else {
        // means there is a command
        let action = args.commands[0].clone();

        if action == "get" {
            // get must have ONE value
            panic!("unimplemented")
        } else if action == "set" {
            panic!("unimplemented")
        } else if action == "rm" {
            panic!("unimplemented")
        } else {
            panic!("unknown subcommand")
        }
    }

    for (_, command) in args.commands.iter().enumerate() {
        println!("command: {}", command);
    }

    println!("Hello, world!");
}

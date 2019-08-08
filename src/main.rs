mod state;

extern crate clap;
use std::io::Read;
use std::fs::File;

const DEFAULT_TAPE_SIZE: usize = 30000;

fn main()
{
    // use clap to get cli arguments
    let matches = clap::App::new("brainfuck_rust")
                    .version(clap::crate_version!())
                    .author(clap::crate_authors!())
                    .about("Yet yet another brainfuck interpreter, now in Rust")
                    .arg(clap::Arg::with_name("program")
                        .short("p")
                        .long("program")
                        .value_name("PROGRAM")
                        .help("Body of program to interpret")
                        .takes_value(true))
                    .arg(clap::Arg::with_name("inputfile")
                        .short("f")
                        .long("file")
                        .value_name("INPUT FILE")
                        .help("Reads program from file")
                        .takes_value(true))
                    .group(clap::ArgGroup::with_name("input").required(true)
                        .args(&["program", "inputfile"]))
                    .arg(clap::Arg::with_name("tapesize")
                        .short("t")
                        .long("tapesize")
                        .value_name("TAPE SIZE")
                        .help("Sets size of brainfuck tape")
                        .takes_value(true))
                    .get_matches();

    // read program from file or from arguments
    let program: String = match matches.value_of("inputfile")
    {
        // read from file
        Some(filename) =>
        {
            let mut file = File::open(filename).expect("Unable to open file");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("Unable to read file");
            contents
        }

        // read from argument
        None => String::from(matches.value_of("program").unwrap()),
    };

    // get tape size
    let tape_size: usize = match matches.value_of("tapesize")
    {
        Some(tape_size) => tape_size.parse::<usize>().unwrap(),
        None => DEFAULT_TAPE_SIZE,
    };

    // create brainfuck state
    let mut bfstate = state::BrainfuckState::new(&program, tape_size);

    // interpret program
    bfstate.interpret();
}

mod interpreter;

extern crate clap;

fn main()
{
    // use clap to get cli arguments
    let matches = clap::App::new("brainfuck_rust")
                    .version(clap::crate_version!())
                    .author(clap::crate_authors!())
                    .about("Yet yet another brainfuck interpreter, now in Rust")
                    .arg(clap::Arg::with_name("PROGRAM")
                        .help("Body of program to interpret")
                        .index(1))
                    .arg(clap::Arg::with_name("inputfile")
                        .short("f")
                        .long("file")
                        .value_name("INPUT FILE")
                        .help("Reads program from file")
                        .takes_value(true))
                    .group(clap::ArgGroup::with_name("input").required(true)
                        .args(&["PROGRAM", "inputfile"]))
                    .arg(clap::Arg::with_name("tapesize")
                        .short("t")
                        .long("tapesize")
                        .value_name("TAPE SIZE")
                        .help("Sets size of brainfuck tape")
                        .takes_value(true))
                    .get_matches();

    // read program from file or from arguments
    let program: Vec<u8> = match matches.value_of("inputfile")
    {
        // read from file
        Some(filename) => Vec::new(),

        // read from argument
        None => matches.value_of("PROGRAM").unwrap().as_bytes().iter().map(|&x| x as u8).collect(),
    };

    // get tape size
    let tape_size: usize = match matches.value_of("tapesize")
    {
        Some(tape_size) => tape_size.parse::<usize>().unwrap(),
        None => 30000,
    };

    // interpret program
    interpreter::interpret(program, tape_size);
}

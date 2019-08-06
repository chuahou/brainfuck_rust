mod interpreter;

fn main()
{
    // get configuration
    let tape_size: usize = 30000;
    let helloworld = b"++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    let mut program: Vec<u8> = Vec::new();
    for byte in helloworld.iter()
    {
        program.push(*byte);
    }
    let program = program;

    // interpret program
    interpreter::interpret(program, tape_size);
}

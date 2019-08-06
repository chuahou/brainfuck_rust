use std::io::Read;

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

    // create tape and pointers
    let mut tape = vec![0u8; tape_size];
    let mut tape_ptr: usize = 0;
    let mut program_ptr: usize = 0;

    // main loop
    while program_ptr < program.len()
    {
        match program[program_ptr]
        {
            // increment data cell
            b'+' => tape[tape_ptr] += 1,

            // decrement data cell
            b'-' => tape[tape_ptr] -= 1,

            // increment tape pointer
            b'>' =>
            {
                if tape_ptr < tape_size - 1
                {
                    tape_ptr += 1;
                }
                else
                {
                    panic!("Exceeded tape size");
                }
            }

            // decrement tape pointer
            b'<' =>
            {
                if tape_ptr > 0
                {
                    tape_ptr -= 1;
                }
                else
                {
                    panic!("Tape pointer moved left of 0");
                }
            }

            // print current data
            b'.' => print!("{}", tape[tape_ptr] as char),

            // input character to current position on data
            b',' => tape[tape_ptr] =
                std::io::stdin().bytes().next().unwrap().unwrap(),

            // handle [
            b'[' =>
            {
                // jump to ]
                if tape[tape_ptr] == 0
                {
                    while program[program_ptr] != b']'
                    {
                        program_ptr += 1;
                    }
                }
            }

            // handle ]
            b']' =>
            {
                // jump to the matching [ if not zero
                if tape[tape_ptr] != 0
                {
                    // count represents the number of excess ] we've encountered
                    // minus the number of [ encountered
                    let mut count = 1;
                    while count > 0
                    {
                        program_ptr -= 1;
                        match program[program_ptr] 
                        {
                            b'[' => count -= 1,
                            b']' => count += 1,
                            _ => (),
                        }
                    }
                }
            }

            // comment
            _ => println!("..."),
        }

        // go to next character
        program_ptr += 1;
    }
}

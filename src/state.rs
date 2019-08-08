use std::io::Read;
use std::num::Wrapping;

#[derive(Debug)]
pub struct BrainfuckState
{
    tape: Vec<Wrapping<u8>>,
    tape_ptr: usize,
    program: Vec<u8>,
    program_ptr: usize,
}

impl BrainfuckState
{
    pub fn new(program_str: &str, tape_size: usize) -> BrainfuckState
    {
        let tape = vec![Wrapping(0u8); tape_size];
        let tape_ptr = 0;
        let program = program_str.as_bytes().iter().map(|&x| x as u8).collect();
        let program_ptr = 0;

        BrainfuckState{ tape, tape_ptr, program, program_ptr }
    }

    pub fn interpret(&mut self)
    {
        while self.program_ptr < self.program.len()
        {
            match self.program[self.program_ptr]
            {
                // increment data cell
                b'+' => self.tape[self.tape_ptr] += Wrapping(1),

                // decrement data cell
                b'-' => self.tape[self.tape_ptr] -= Wrapping(1),

                // increment self.tape pointer
                b'>' =>
                {
                    if self.tape_ptr < self.tape.len() - 1
                    {
                        self.tape_ptr += 1;
                    }
                    else
                    {
                        panic!("Exceeded self.tape size");
                    }
                }

                // decrement self.tape pointer
                b'<' =>
                {
                    if self.tape_ptr > 0
                    {
                        self.tape_ptr -= 1;
                    }
                    else
                    {
                        panic!("Tape pointer moved left of 0");
                    }
                }

                // print current data
                b'.' => match self.tape[self.tape_ptr] { Wrapping(c) => print!("{}", c as char) },

                // input character to current position on data
                b',' => self.tape[self.tape_ptr] =
                    Wrapping(std::io::stdin().bytes().next().unwrap().unwrap() as u8),

                // handle [
                b'[' =>
                {
                    // jump to ]
                    if self.tape[self.tape_ptr] == Wrapping(0)
                    {
                        // count represents the number of excess [ we've encountered
                        // minus the number of ] encountered
                        let mut count = 1;
                        while count > 0
                        {
                            self.program_ptr += 1;
                            match self.program[self.program_ptr]
                            {
                                b']' => count -= 1,
                                b'[' => count += 1,
                                _ => (),
                            }
                        }
                    }
                }

                // handle ]
                b']' =>
                {
                    // jump to the matching [ if not zero
                    if self.tape[self.tape_ptr] != Wrapping(0)
                    {
                        // count represents the number of excess ] we've encountered
                        // minus the number of [ encountered
                        let mut count = 1;
                        while count > 0
                        {
                            self.program_ptr -= 1;
                            match self.program[self.program_ptr] 
                            {
                                b'[' => count -= 1,
                                b']' => count += 1,
                                _ => (),
                            }
                        }
                    }
                }

                // comment
                _ => (),
            }

            // go to next character
            self.program_ptr += 1;
        }
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_construct_bf_state()
    {
        let mut bfstate = BrainfuckState::new("++++++++[->+++++++++<]", 30000);
        assert_eq!(bfstate.tape.len(), 30000, "Testing tape size");
        assert_eq!(bfstate.tape_ptr, 0, "Testing tape pointer set to 0");
        assert_eq!(bfstate.program, vec!(b'+', b'+', b'+', b'+', b'+', b'+',
            b'+', b'+', b'[', b'-', b'>', b'+', b'+', b'+', b'+', b'+', b'+',
            b'+', b'+', b'+', b'<', b']'), "Testing program converted to Vec<u8>");
        assert_eq!(bfstate.program_ptr, 0, "Testing program pointer set to 0");

        // check no panic
        bfstate.interpret();
    }

    #[test]
    fn test_overflow()
    {
        let mut bfstate = BrainfuckState::new("-", 1);
        bfstate.tape[0] = Wrapping(0);
        bfstate.interpret();
        assert_eq!(bfstate.tape[0], Wrapping(255), "Testing underflow");

        // change program to increment
        bfstate.program = vec!(b'+');
        bfstate.program_ptr = 0;
        bfstate.interpret();
        assert_eq!(bfstate.tape[0], Wrapping(0), "Testing overflow");
    }

    #[test]
    #[should_panic]
    fn test_program_ptr_out_of_bounds_left()
    {
        let mut bfstate = BrainfuckState::new("<", 100);
        bfstate.interpret();
    }

    #[test]
    #[should_panic]
    fn test_program_ptr_out_of_bounds_right()
    {
        let mut program = String::new();
        for _ in 1..101
        {
            program.push('>');
        }
        let mut bfstate = BrainfuckState::new(&program, 100);
        bfstate.interpret();
    }
}

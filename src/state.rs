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
}

#[cfg(test)]
mod tests
{
    use super::*;

    fn construct_bf_state() -> BrainfuckState
    {
        BrainfuckState::new("++++++++[->+++++++++<]", 30000)
    }

    #[test]
    fn test_construct_bf_state()
    {
        let bfstate = construct_bf_state();
        assert_eq!(bfstate.tape.len(), 30000, "Testing tape size");
        assert_eq!(bfstate.tape_ptr, 0, "Testing tape pointer set to 0");
        assert_eq!(bfstate.program, vec!(b'+', b'+', b'+', b'+', b'+', b'+',
            b'+', b'+', b'[', b'-', b'>', b'+', b'+', b'+', b'+', b'+', b'+',
            b'+', b'+', b'+', b'<', b']'), "Testing program converted to Vec<u8>");
        assert_eq!(bfstate.program_ptr, 0, "Testing program pointer set to 0");
    }

    #[test]
    fn test_overflow()
    {
        let mut bfstate = construct_bf_state();
        bfstate.tape[0] = Wrapping(0);
        bfstate.tape[0] -= Wrapping(1);
        assert_eq!(bfstate.tape[0], Wrapping(255), "Testing underflow");
        bfstate.tape[0] += Wrapping(1);
        assert_eq!(bfstate.tape[0], Wrapping(0), "Testing overflow");
    }

    #[test]
    #[should_panic]
    fn test_program_ptr_out_of_bounds()
    {
        let mut bfstate = construct_bf_state();
        bfstate.program_ptr -= 1;
    }
}

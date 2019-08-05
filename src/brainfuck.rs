// brainfuck tape struct representing tape
#[derive(Debug)]
pub struct BrainfuckTape
{
    pub tape: Vec<i8>,
    pub ptr_idx: usize
}

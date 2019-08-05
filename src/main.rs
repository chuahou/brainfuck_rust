mod brainfuck;

use crate::brainfuck::BrainfuckTape;

fn main()
{
    let bftape = BrainfuckTape { tape: Vec::new(), ptr_idx: 0};

    println!("{:?}", bftape);
}

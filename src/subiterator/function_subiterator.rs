//! SubIterator for a Function

/// Sub-iterator for a Function. Keeps track of current location in a Function.
pub struct FuncSubIterator {
    /// The current instruction the SubIterator is at
    pub(crate) curr_instr: usize,
    /// Number of instructions in this function
    num_instr: usize,
}

#[allow(dead_code)]
impl FuncSubIterator {
    /// Creates a new FunctionSubIterator
    pub fn new(num_instr: usize) -> Self {
        FuncSubIterator {
            curr_instr: 0,
            num_instr,
        }
    }

    /// Resets the FunctionSubIterator
    pub(crate) fn reset(&mut self, num_instr: usize) {
        self.curr_instr = 0;
        self.num_instr = num_instr;
    }

    /// Checks if there are instructions left to visit
    pub(crate) fn has_next(&self) -> bool {
        self.curr_instr + 1 < self.num_instr
    }

    /// Goes to the next instruction
    pub(crate) fn next(&mut self) -> bool {
        if !self.has_next() {
            false
        } else {
            self.curr_instr += 1;
            true
        }
    }

    /// Checks if the SubIterator has finished traversing all the instructions
    pub fn end(&mut self) -> bool {
        self.curr_instr == self.num_instr
    }
}

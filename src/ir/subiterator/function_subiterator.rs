/// Sub-iterator for a Function. Keeps track of current location in a Function.
pub struct FuncSubIterator {
    pub(crate) curr_instr: usize,
    num_instr: usize,
}

#[allow(dead_code)]
impl FuncSubIterator {
    pub fn new(num_instr: usize) -> Self {
        FuncSubIterator {
            curr_instr: 0,
            num_instr,
        }
    }

    pub(crate) fn reset(&mut self, num_instr: usize) {
        self.curr_instr = 0;
        self.num_instr = num_instr;
    }

    pub(crate) fn has_next(&self) -> bool {
        self.curr_instr + 1 < self.num_instr
    }

    pub(crate) fn next(&mut self) -> bool {
        if !self.has_next() {
            false
        } else {
            self.curr_instr += 1;
            true
        }
    }

    pub fn end(&mut self) -> bool {
        self.curr_instr == self.num_instr
    }
}

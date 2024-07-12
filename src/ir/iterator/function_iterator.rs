pub struct FuncIterator {
    pub(crate) curr_instr: usize,
    num_instr: usize,
}

impl FuncIterator {
    pub fn new(num_instr: usize) -> Self {
        FuncIterator {
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
}

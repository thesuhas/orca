use crate::ir::types::InstrumentType::{InstrumentAfter, InstrumentAlternate, InstrumentBefore};
use crate::ir::types::{Body, InstrumentType};
use wasmparser::Operator;

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

    pub(crate) fn next<'a>(&'a mut self, body: &'a Body) -> Option<&Operator> {
        if !self.has_next() {
            None
        } else {
            self.curr_instr += 1;
            let instr = &body.instructions[self.curr_instr];
            Some(&instr.0)
        }
    }

    pub(crate) fn curr_op<'a>(&'a self, body: &'a Body) -> Option<&Operator> {
        if self.curr_instr < self.num_instr {
            let instr = &body.instructions[self.curr_instr];
            Some(&instr.0)
        } else {
            None
        }
    }

    pub(crate) fn get_instrument_type<'a>(&'a self, body: &'a Body) -> &InstrumentType {
        &body.instructions[self.curr_instr].1
    }

    pub(crate) fn before(&self, body: &mut Body) {
        body.instructions[self.curr_instr].1 = InstrumentBefore(vec![])
    }

    pub(crate) fn after(&self, body: &mut Body) {
        body.instructions[self.curr_instr].1 = InstrumentAfter(vec![])
    }

    pub(crate) fn alternate(&self, body: &mut Body) {
        body.instructions[self.curr_instr].1 = InstrumentAlternate(vec![])
    }

    pub fn inject<'a>(&mut self, body: &mut Body<'a>, instr: Operator<'a>) {
        body.instructions[self.curr_instr].1.add_instr(instr);
    }

    pub fn get_injected_val<'a>(&'a self, idx: usize, body: &'a Body) -> &Operator {
        body.instructions[self.curr_instr].1.get_instr(idx)
    }
}

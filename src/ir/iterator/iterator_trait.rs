//! Trait that needs to be satisfied by all iterators

use crate::ir::types::{InstrumentType, Location};
use wasmparser::Operator;

#[allow(dead_code)]
/// Iterator trait that must be satisfied by all Iterators. Enables code injection and traversal.
/// Instructions as defined [here].
///
/// [here]: https://webassembly.github.io/spec/core/binary/instructions.html
pub trait Iterator<'a> {
    /// Inject an operator at the current location
    fn inject(&mut self, instr: Operator<'a>);

    /// Mark the current location to InstrumentBefore
    fn before(&mut self) -> &mut Self;

    /// Mark the current location to InstrumentAfter
    fn after(&mut self) -> &mut Self;

    /// Mark the current location to InstrumentAlternate
    fn alternate(&mut self) -> &mut Self;

    /// Reset the Iterator and all Child Iterators and SubIterators
    fn reset(&mut self);

    /// Go to the next Instruction
    fn next(&mut self) -> Option<&Operator>;

    /// Get the current location
    fn curr_loc(&self) -> Location;

    /// Get the InstrumentType of the current location
    fn curr_instrument_type(&self) -> &InstrumentType;

    /// Get the current instruction
    fn curr_op(&self) -> Option<&Operator>;

    /// Get the instruction injected at index idx
    fn get_injected_val(&self, idx: usize) -> &Operator;

    // Control Flow
    /// Inject a call instruction
    fn call(&mut self, idx: u32) -> &mut Self {
        self.inject(Operator::Call {
            function_index: idx,
        });
        self
    }

    /// Inject a return statement
    fn return_stmt(&mut self) -> &mut Self {
        self.inject(Operator::Return);
        self
    }

    /// Inject a no op instruction
    fn nop(&mut self) -> &mut Self {
        self.inject(Operator::Nop);
        self
    }

    /// Inject an unreachable instruction
    fn unreachable(&mut self) -> &mut Self {
        self.inject(Operator::Unreachable);
        self
    }

    /// Inject an if statement
    fn if_stmt(&mut self, block_type: wasmparser::ValType) -> &mut Self {
        self.inject(Operator::If {
            blockty: wasmparser::BlockType::Type(block_type),
        });
        self
    }

    /// Inject an else statement
    fn else_stmt(&mut self) -> &mut Self {
        self.inject(Operator::Else);
        self
    }

    /// Inject an end statement. Indicates the end of the current scope
    fn end(&mut self) -> &mut Self {
        self.inject(Operator::End);
        self
    }

    /// Inject a block statement. Indicates the start of a block
    fn block(&mut self, block_type: wasmparser::BlockType) -> &mut Self {
        self.inject(Operator::Block {
            blockty: block_type,
        });
        self
    }

    /// Inject a loop statement
    fn loop_stmt(&mut self, block_type: wasmparser::BlockType) -> &mut Self {
        self.inject(Operator::Loop {
            blockty: block_type,
        });
        self
    }

    /// Inject a break statement
    fn br(&mut self, relative_depth: u32) -> &mut Self {
        self.inject(Operator::Br { relative_depth });
        self
    }

    /// Inject a conditional break statement
    fn br_if(&mut self, relative_depth: u32) -> &mut Self {
        self.inject(Operator::BrIf { relative_depth });
        self
    }

    // Numerics
    /// Inject a local.get
    fn local_get(&mut self, idx: u32) -> &mut Self {
        self.inject(Operator::LocalGet { local_index: idx });
        self
    }

    /// Inject a local.set
    fn local_set(&mut self, idx: u32) -> &mut Self {
        self.inject(Operator::LocalSet { local_index: idx });
        self
    }

    // Integers
    /// Inject an i32.const instruction
    fn i32(&mut self, value: i32) -> &mut Self {
        self.inject(Operator::I32Const { value });
        self
    }

    /// Inject an i32.add instruction
    fn i32_add(&mut self) -> &mut Self {
        self.inject(Operator::I32Add);
        self
    }

    /// Inject an i32.sub instruction
    fn i32_sub(&mut self) -> &mut Self {
        self.inject(Operator::I32Sub);
        self
    }

    /// Inject an i32.mul instruction
    fn i32_mul(&mut self) -> &mut Self {
        self.inject(Operator::I32Mul);
        self
    }

    /// Inject an i32.divs instruction
    fn i32_div_signed(&mut self) -> &mut Self {
        self.inject(Operator::I32DivS);
        self
    }

    /// Inject an i32.divu instruction
    fn i32_div_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I32DivU);
        self
    }

    /// Inject an i32.remu instruction
    fn i32_rem_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I32RemU);
        self
    }

    /// Inject an i32.rems instruction
    fn i32_rem_signed(&mut self) -> &mut Self {
        self.inject(Operator::I32RemS);
        self
    }

    /// Inject an i32.and instruction
    fn i32_and(&mut self) -> &mut Self {
        self.inject(Operator::I32And);
        self
    }

    /// Inject an i32.or instruction
    fn i32_or(&mut self) -> &mut Self {
        self.inject(Operator::I32Or);
        self
    }

    /// Inject an i32.xor instruction
    fn i32_xor(&mut self) -> &mut Self {
        self.inject(Operator::I32Xor);
        self
    }

    /// Inject an i32.shl instruction
    fn i32_shl(&mut self) -> &mut Self {
        self.inject(Operator::I32Shl);
        self
    }

    /// Inject an i32.shrs instruction
    fn i32_shr_signed(&mut self) -> &mut Self {
        self.inject(Operator::I32ShrS);
        self
    }

    /// Inject an i32.shru instruction
    fn i32_shr_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I32ShrU);
        self
    }

    /// Inject an i32.rotl instruction
    fn i32_rotl(&mut self) -> &mut Self {
        self.inject(Operator::I32Rotl);
        self
    }

    /// Inject and i32.rotr instruction
    fn i32_rotr(&mut self) -> &mut Self {
        self.inject(Operator::I32Rotr);
        self
    }

    /// Inject an i32.eq instruction
    fn i32_eq(&mut self) -> &mut Self {
        self.inject(Operator::I32Eq);
        self
    }

    /// Inject an i32.eqz instruction
    fn i32_eqz(&mut self) -> &mut Self {
        self.inject(Operator::I32Eqz);
        self
    }

    /// Inject an i32.ne instruction
    fn i32_ne(&mut self) -> &mut Self {
        self.inject(Operator::I32Ne);
        self
    }

    /// Inject an i32.ltu instruction
    fn i32_lt_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I32LtU);
        self
    }

    /// Inject an i32.lts instruction
    fn i32_lt_signed(&mut self) -> &mut Self {
        self.inject(Operator::I32LtS);
        self
    }

    /// Inject an i32.gtu instruction
    fn i32_gt_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I32GtU);
        self
    }

    /// Inject an i32.gts instruction
    fn i32_gt_signed(&mut self) -> &mut Self {
        self.inject(Operator::I32GtS);
        self
    }

    /// Inject an i32.lteu instruction
    fn i32_lte_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I32LeU);
        self
    }

    /// Inject an i32.ltes instruction
    fn i32_lte_signed(&mut self) -> &mut Self {
        self.inject(Operator::I32LeS);
        self
    }

    /// Inject an i32.gteu instruction
    fn i32_gte_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I32GeU);
        self
    }

    /// Inject an i32.gtes instruction
    fn i32_gte_signed(&mut self) -> &mut Self {
        self.inject(Operator::I32GeS);
        self
    }

    /// Inject an i64.const
    fn i64(&mut self, value: i64) -> &mut Self {
        self.inject(Operator::I64Const { value });
        self
    }

    /// Inject an i64.add instruction
    fn i64_add(&mut self) -> &mut Self {
        self.inject(Operator::I64Add);
        self
    }

    /// Inject an i64.sub instruction
    fn i64_sub(&mut self) -> &mut Self {
        self.inject(Operator::I64Sub);
        self
    }

    /// Inject an i64.mul instruction
    fn i64_mul(&mut self) -> &mut Self {
        self.inject(Operator::I64Mul);
        self
    }

    /// Inject an i64.divs instruction
    fn i64_div_signed(&mut self) -> &mut Self {
        self.inject(Operator::I64DivS);
        self
    }

    /// Inject an i64.divu instruction
    fn i64_div_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I64DivU);
        self
    }

    /// Inject an i64.remu instruction
    fn i64_rem_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I64RemU);
        self
    }

    /// Inject an i64.rems instruction
    fn i64_rem_signed(&mut self) -> &mut Self {
        self.inject(Operator::I64RemS);
        self
    }

    /// Inject an i64.and instruction
    fn i64_and(&mut self) -> &mut Self {
        self.inject(Operator::I64And);
        self
    }

    /// Inject an i64.or instruction
    fn i64_or(&mut self) -> &mut Self {
        self.inject(Operator::I64Or);
        self
    }

    /// Inject an i64.xor instruction
    fn i64_xor(&mut self) -> &mut Self {
        self.inject(Operator::I64Xor);
        self
    }

    /// Inject an i64.shl instruction
    fn i64_shl(&mut self) -> &mut Self {
        self.inject(Operator::I64Shl);
        self
    }

    /// Inject an i64.shrs instruction
    fn i64_shr_signed(&mut self) -> &mut Self {
        self.inject(Operator::I64ShrS);
        self
    }

    /// Inject an i64.shru instruction
    fn i64_shr_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I64ShrU);
        self
    }

    /// Inject an i64.rotl instruction
    fn i64_rotl(&mut self) -> &mut Self {
        self.inject(Operator::I64Rotl);
        self
    }

    /// Inject an i64.rotr instruction
    fn i64_rotr(&mut self) -> &mut Self {
        self.inject(Operator::I64Rotr);
        self
    }

    /// Inject an i64.eq instruction
    fn i64_eq(&mut self) -> &mut Self {
        self.inject(Operator::I64Eq);
        self
    }

    /// Inject an i64.eqz instruction
    fn i64_eqz(&mut self) -> &mut Self {
        self.inject(Operator::I64Eqz);
        self
    }

    /// Inject an i64.ne instruction
    fn i64_ne(&mut self) -> &mut Self {
        self.inject(Operator::I64Ne);
        self
    }

    /// Inject an i64.ltu instruction
    fn i64_lt_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I64LtU);
        self
    }

    /// Inject an i64.lts instruction
    fn i64_lt_signed(&mut self) -> &mut Self {
        self.inject(Operator::I64LtS);
        self
    }

    /// Inject an i64.gtu instruction
    fn i64_gt_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I64GtU);
        self
    }

    /// Inject an i64.gts instruction
    fn i64_gt_signed(&mut self) -> &mut Self {
        self.inject(Operator::I64GtS);
        self
    }

    /// Inject an i64.lteu instruction
    fn i64_lte_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I64LeU);
        self
    }

    /// Inject an i64.ltes instruction
    fn i64_lte_signed(&mut self) -> &mut Self {
        self.inject(Operator::I64LeS);
        self
    }

    /// Inject an i64.gteu instruction
    fn i64_gte_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I64GeU);
        self
    }

    /// Inject an i64.gtes instruction
    fn i64_gte_signed(&mut self) -> &mut Self {
        self.inject(Operator::I64GeS);
        self
    }

    // Floating point
    /// Inject a f32.const instruction
    fn f32(&mut self, val: f32) -> &mut Self {
        self.inject(Operator::F32Const {
            value: wasmparser::Ieee32::from(val),
        });
        self
    }

    /// Inject a f32.abs instruction
    fn f32_abs(&mut self) -> &mut Self {
        self.inject(Operator::F32Abs);
        self
    }

    /// Inject a f32.ceil instruction
    fn f32_ceil(&mut self) -> &mut Self {
        self.inject(Operator::F32Ceil);
        self
    }

    /// Inject a f32.floor instruction
    fn f32_floor(&mut self) -> &mut Self {
        self.inject(Operator::F32Floor);
        self
    }

    /// Inject a f32.trunc instruction
    fn f32_trunc(&mut self) -> &mut Self {
        self.inject(Operator::F32Trunc);
        self
    }

    /// Inject a f32.sqrt instruction
    fn f32_sqrt(&mut self) -> &mut Self {
        self.inject(Operator::F32Sqrt);
        self
    }

    /// Inject a f32.add instruction
    fn f32_add(&mut self) -> &mut Self {
        self.inject(Operator::F32Add);
        self
    }

    /// Inject a f32.sub instruction
    fn f32_sub(&mut self) -> &mut Self {
        self.inject(Operator::F32Sub);
        self
    }

    /// Inject a f32.mul instruction
    fn f32_mul(&mut self) -> &mut Self {
        self.inject(Operator::F32Mul);
        self
    }

    /// Inject a f32.div instruction
    fn f32_div(&mut self) -> &mut Self {
        self.inject(Operator::F32Div);
        self
    }

    /// Inject a f32.min instruction
    fn f32_min(&mut self) -> &mut Self {
        self.inject(Operator::F32Min);
        self
    }

    /// Inject a f32.max instruction
    fn f32_max(&mut self) -> &mut Self {
        self.inject(Operator::F32Max);
        self
    }

    /// Inject a f32.eq instruction
    fn f32_eq(&mut self) -> &mut Self {
        self.inject(Operator::F32Eq);
        self
    }

    /// Inject a f32.ne instruction
    fn f32_ne(&mut self) -> &mut Self {
        self.inject(Operator::F32Ne);
        self
    }

    /// Inject a f32.gt instruction
    fn f32_gt(&mut self) -> &mut Self {
        self.inject(Operator::F32Gt);
        self
    }

    /// Inject a f32.ge instruction
    fn f32_ge(&mut self) -> &mut Self {
        self.inject(Operator::F32Ge);
        self
    }

    /// Inject a f32.lt instruction
    fn f32_lt(&mut self) -> &mut Self {
        self.inject(Operator::F32Lt);
        self
    }

    /// Inject a f32.le instruction
    fn f32_le(&mut self) -> &mut Self {
        self.inject(Operator::F32Le);
        self
    }

    /// Inject a f64.const instruction
    fn f64(&mut self, val: f64) -> &mut Self {
        self.inject(Operator::F64Const {
            value: wasmparser::Ieee64::from(val),
        });
        self
    }

    /// Inject a f64.abs instruction
    fn f64_abs(&mut self) -> &mut Self {
        self.inject(Operator::F64Abs);
        self
    }

    /// Inject a f64.ceil instruction
    fn f64_ceil(&mut self) -> &mut Self {
        self.inject(Operator::F64Ceil);
        self
    }

    /// Inject a f64.floor instruction
    fn f64_floor(&mut self) -> &mut Self {
        self.inject(Operator::F64Floor);
        self
    }

    /// Inject a f64.trunc instruction
    fn f64_trunc(&mut self) -> &mut Self {
        self.inject(Operator::F64Trunc);
        self
    }

    /// Inject a f64.sqrt instruction
    fn f64_sqrt(&mut self) -> &mut Self {
        self.inject(Operator::F64Sqrt);
        self
    }

    /// Inject a f64.add instruction
    fn f64_add(&mut self) -> &mut Self {
        self.inject(Operator::F64Add);
        self
    }

    /// Inject a f64.sub instruction
    fn f64_sub(&mut self) -> &mut Self {
        self.inject(Operator::F64Sub);
        self
    }

    /// Inject a f64.mul instruction
    fn f64_mul(&mut self) -> &mut Self {
        self.inject(Operator::F64Mul);
        self
    }

    /// Inject a f64.div instruction
    fn f64_div(&mut self) -> &mut Self {
        self.inject(Operator::F64Div);
        self
    }

    /// Inject a f64.min instruction
    fn f64_min(&mut self) -> &mut Self {
        self.inject(Operator::F64Min);
        self
    }

    /// Inject a f64.max instruction
    fn f64_max(&mut self) -> &mut Self {
        self.inject(Operator::F64Max);
        self
    }

    /// Inject a f64.eq instruction
    fn f64_eq(&mut self) -> &mut Self {
        self.inject(Operator::F64Eq);
        self
    }

    /// Inject a f64.ne instruction
    fn f64_ne(&mut self) -> &mut Self {
        self.inject(Operator::F64Ne);
        self
    }

    /// Inject a f64.gt instruction
    fn f64_gt(&mut self) -> &mut Self {
        self.inject(Operator::F64Gt);
        self
    }

    /// Inject a f64.ge instruction
    fn f64_ge(&mut self) -> &mut Self {
        self.inject(Operator::F64Ge);
        self
    }

    /// Inject a f64.lt instruction
    fn f64_lt(&mut self) -> &mut Self {
        self.inject(Operator::F64Lt);
        self
    }

    /// Inject a f64.le instruction
    fn f64_le(&mut self) -> &mut Self {
        self.inject(Operator::F64Le);
        self
    }

    // Memory Instructions
    /// Inject a memory.init instruction
    fn memory_init(&mut self, data_index: u32, mem: u32) -> &mut Self {
        self.inject(Operator::MemoryInit { data_index, mem });
        self
    }

    /// Inject a memory.size instruction
    fn memory_size(&mut self, mem: u32) -> &mut Self {
        self.inject(Operator::MemorySize { mem });
        self
    }

    /// Inject a memory.grow instruction
    fn memory_grow(&mut self, mem: u32) -> &mut Self {
        self.inject(Operator::MemoryGrow { mem });
        self
    }

    /// Inject a memory.fill instruction
    fn memory_fill(&mut self, mem: u32) -> &mut Self {
        self.inject(Operator::MemoryFill { mem });
        self
    }

    /// Inject a memory.copy instruction
    fn memory_copy(&mut self, dst_mem: u32, src_mem: u32) -> &mut Self {
        self.inject(Operator::MemoryCopy { dst_mem, src_mem });
        self
    }

    /// Inject a memory.discard instruction
    fn memory_discard(&mut self, mem: u32) -> &mut Self {
        self.inject(Operator::MemoryDiscard { mem });
        self
    }
    /// Inject a data drop instruction
    fn data_drop(&mut self, data_index: u32) -> &mut Self {
        self.inject(Operator::DataDrop { data_index });
        self
    }

    // Parametric Instructions
    /// Inject a drop instruction
    fn drop(&mut self) -> &mut Self {
        self.inject(Operator::Drop);
        self
    }
}

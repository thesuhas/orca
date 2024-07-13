use crate::ir::types::{InstrumentType, Location};
use wasmparser::Operator;

#[allow(dead_code)]
pub trait Iterator<'a> {
    fn inject(&mut self, instr: Operator<'a>);

    fn before(&mut self) -> &mut Self;

    fn after(&mut self) -> &mut Self;

    fn alternate(&mut self) -> &mut Self;

    fn reset(&mut self);

    fn next(&mut self) -> Option<&Operator>;

    fn curr_loc(&self) -> Location;

    fn curr_instrument_type(&self) -> &InstrumentType;

    fn curr_op(&self) -> Option<&Operator>;

    fn get_injected_val(&self, idx: usize) -> &Operator;

    // Control Flow
    fn call(&mut self, idx: u32) -> &mut Self {
        self.inject(Operator::Call {
            function_index: idx,
        });
        self
    }

    fn return_stmt(&mut self) -> &mut Self {
        self.inject(Operator::Return);
        self
    }

    fn nop(&mut self) -> &mut Self {
        self.inject(Operator::Nop);
        self
    }

    fn unreachable(&mut self) -> &mut Self {
        self.inject(Operator::Unreachable);
        self
    }

    fn if_stmt(&mut self, block_type: wasmparser::ValType) -> &mut Self {
        self.inject(Operator::If {
            blockty: wasmparser::BlockType::Type(block_type),
        });
        self
    }

    fn else_stmt(&mut self) -> &mut Self {
        self.inject(Operator::Else);
        self
    }

    fn end(&mut self) -> &mut Self {
        self.inject(Operator::End);
        self
    }

    fn block(&mut self, block_type: wasmparser::BlockType) -> &mut Self {
        self.inject(Operator::Block {
            blockty: block_type,
        });
        self
    }

    fn loop_stmt(&mut self, block_type: wasmparser::BlockType) -> &mut Self {
        self.inject(Operator::Loop {
            blockty: block_type,
        });
        self
    }

    fn br(&mut self, relative_depth: u32) -> &mut Self {
        self.inject(Operator::Br { relative_depth });
        self
    }

    fn br_if(&mut self, relative_depth: u32) -> &mut Self {
        self.inject(Operator::BrIf { relative_depth });
        self
    }

    // Numerics
    fn local_get(&mut self, idx: u32) -> &mut Self {
        self.inject(Operator::LocalGet { local_index: idx });
        self
    }

    fn local_set(&mut self, idx: u32) -> &mut Self {
        self.inject(Operator::LocalSet { local_index: idx });
        self
    }

    // Integers
    fn i32(&mut self, value: i32) -> &mut Self {
        self.inject(Operator::I32Const { value });
        self
    }

    fn i32_add(&mut self) -> &mut Self {
        self.inject(Operator::I32Add);
        self
    }

    fn i32_sub(&mut self) -> &mut Self {
        self.inject(Operator::I32Sub);
        self
    }

    fn i32_mul(&mut self) -> &mut Self {
        self.inject(Operator::I32Mul);
        self
    }

    fn i32_div_signed(&mut self) -> &mut Self {
        self.inject(Operator::I32DivS);
        self
    }

    fn i32_div_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I32DivU);
        self
    }

    fn i32_rem_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I32RemU);
        self
    }

    fn i32_rem_signed(&mut self) -> &mut Self {
        self.inject(Operator::I32RemS);
        self
    }

    fn i32_and(&mut self) -> &mut Self {
        self.inject(Operator::I32And);
        self
    }

    fn i32_or(&mut self) -> &mut Self {
        self.inject(Operator::I32Or);
        self
    }

    fn i32_xor(&mut self) -> &mut Self {
        self.inject(Operator::I32Xor);
        self
    }

    fn i32_shl(&mut self) -> &mut Self {
        self.inject(Operator::I32Shl);
        self
    }

    fn i32_shr_signed(&mut self) -> &mut Self {
        self.inject(Operator::I32ShrS);
        self
    }

    fn i32_shr_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I32ShrU);
        self
    }

    fn i32_rotl(&mut self) -> &mut Self {
        self.inject(Operator::I32Rotl);
        self
    }

    fn i32_rotr(&mut self) -> &mut Self {
        self.inject(Operator::I32Rotr);
        self
    }

    fn i32_eq(&mut self) -> &mut Self {
        self.inject(Operator::I32Eq);
        self
    }

    fn i32_eqz(&mut self) -> &mut Self {
        self.inject(Operator::I32Eqz);
        self
    }

    fn i32_ne(&mut self) -> &mut Self {
        self.inject(Operator::I32Ne);
        self
    }

    fn i32_lt_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I32LtU);
        self
    }

    fn i32_lt_signed(&mut self) -> &mut Self {
        self.inject(Operator::I32LtS);
        self
    }

    fn i32_gt_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I32GtU);
        self
    }

    fn i32_gt_signed(&mut self) -> &mut Self {
        self.inject(Operator::I32GtS);
        self
    }

    fn i32_lte_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I32LeU);
        self
    }

    fn i32_lte_signed(&mut self) -> &mut Self {
        self.inject(Operator::I32LeS);
        self
    }

    fn i32_gte_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I32GeU);
        self
    }

    fn i32_gte_signed(&mut self) -> &mut Self {
        self.inject(Operator::I32GeS);
        self
    }

    fn i64(&mut self, value: i64) -> &mut Self {
        self.inject(Operator::I64Const { value });
        self
    }

    fn i64_add(&mut self) -> &mut Self {
        self.inject(Operator::I64Add);
        self
    }

    fn i64_sub(&mut self) -> &mut Self {
        self.inject(Operator::I64Sub);
        self
    }

    fn i64_mul(&mut self) -> &mut Self {
        self.inject(Operator::I64Mul);
        self
    }

    fn i64_div_signed(&mut self) -> &mut Self {
        self.inject(Operator::I64DivS);
        self
    }

    fn i64_div_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I64DivU);
        self
    }

    fn i64_rem_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I64RemU);
        self
    }

    fn i64_rem_signed(&mut self) -> &mut Self {
        self.inject(Operator::I64RemS);
        self
    }

    fn i64_and(&mut self) -> &mut Self {
        self.inject(Operator::I64And);
        self
    }

    fn i64_or(&mut self) -> &mut Self {
        self.inject(Operator::I64Or);
        self
    }

    fn i64_xor(&mut self) -> &mut Self {
        self.inject(Operator::I64Xor);
        self
    }

    fn i64_shl(&mut self) -> &mut Self {
        self.inject(Operator::I64Shl);
        self
    }

    fn i64_shr_signed(&mut self) -> &mut Self {
        self.inject(Operator::I64ShrS);
        self
    }

    fn i64_shr_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I64ShrU);
        self
    }

    fn i64_rotl(&mut self) -> &mut Self {
        self.inject(Operator::I64Rotl);
        self
    }

    fn i64_rotr(&mut self) -> &mut Self {
        self.inject(Operator::I64Rotr);
        self
    }

    fn i64_eq(&mut self) -> &mut Self {
        self.inject(Operator::I64Eq);
        self
    }

    fn i64_eqz(&mut self) -> &mut Self {
        self.inject(Operator::I64Eqz);
        self
    }

    fn i64_ne(&mut self) -> &mut Self {
        self.inject(Operator::I64Ne);
        self
    }

    fn i64_lt_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I64LtU);
        self
    }

    fn i64_lt_signed(&mut self) -> &mut Self {
        self.inject(Operator::I64LtS);
        self
    }

    fn i64_gt_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I64GtU);
        self
    }

    fn i64_gt_signed(&mut self) -> &mut Self {
        self.inject(Operator::I64GtS);
        self
    }

    fn i64_lte_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I64LeU);
        self
    }

    fn i64_lte_signed(&mut self) -> &mut Self {
        self.inject(Operator::I64LeS);
        self
    }

    fn i64_gte_unsigned(&mut self) -> &mut Self {
        self.inject(Operator::I64GeU);
        self
    }

    fn i64_gte_signed(&mut self) -> &mut Self {
        self.inject(Operator::I64GeS);
        self
    }

    // Floating point
    fn f32(&mut self, val: f32) -> &mut Self {
        self.inject(Operator::F32Const {
            value: wasmparser::Ieee32::from(val),
        });
        self
    }

    fn f32_abs(&mut self) -> &mut Self {
        self.inject(Operator::F32Abs);
        self
    }

    fn f32_ceil(&mut self) -> &mut Self {
        self.inject(Operator::F32Ceil);
        self
    }

    fn f32_floor(&mut self) -> &mut Self {
        self.inject(Operator::F32Floor);
        self
    }

    fn f32_trunc(&mut self) -> &mut Self {
        self.inject(Operator::F32Trunc);
        self
    }

    fn f32_sqrt(&mut self) -> &mut Self {
        self.inject(Operator::F32Sqrt);
        self
    }

    fn f32_add(&mut self) -> &mut Self {
        self.inject(Operator::F32Add);
        self
    }

    fn f32_sub(&mut self) -> &mut Self {
        self.inject(Operator::F32Sub);
        self
    }

    fn f32_mul(&mut self) -> &mut Self {
        self.inject(Operator::F32Mul);
        self
    }

    fn f32_div(&mut self) -> &mut Self {
        self.inject(Operator::F32Div);
        self
    }

    fn f32_min(&mut self) -> &mut Self {
        self.inject(Operator::F32Min);
        self
    }

    fn f32_max(&mut self) -> &mut Self {
        self.inject(Operator::F32Max);
        self
    }

    fn f32_eq(&mut self) -> &mut Self {
        self.inject(Operator::F32Eq);
        self
    }

    fn f32_ne(&mut self) -> &mut Self {
        self.inject(Operator::F32Ne);
        self
    }

    fn f32_gt(&mut self) -> &mut Self {
        self.inject(Operator::F32Gt);
        self
    }

    fn f32_ge(&mut self) -> &mut Self {
        self.inject(Operator::F32Ge);
        self
    }

    fn f32_lt(&mut self) -> &mut Self {
        self.inject(Operator::F32Lt);
        self
    }

    fn f32_le(&mut self) -> &mut Self {
        self.inject(Operator::F32Le);
        self
    }

    fn f64(&mut self, val: f64) -> &mut Self {
        self.inject(Operator::F64Const {
            value: wasmparser::Ieee64::from(val),
        });
        self
    }

    fn f64_abs(&mut self) -> &mut Self {
        self.inject(Operator::F64Abs);
        self
    }

    fn f64_ceil(&mut self) -> &mut Self {
        self.inject(Operator::F64Ceil);
        self
    }

    fn f64_floor(&mut self) -> &mut Self {
        self.inject(Operator::F64Floor);
        self
    }

    fn f64_trunc(&mut self) -> &mut Self {
        self.inject(Operator::F64Trunc);
        self
    }

    fn f64_sqrt(&mut self) -> &mut Self {
        self.inject(Operator::F64Sqrt);
        self
    }

    fn f64_add(&mut self) -> &mut Self {
        self.inject(Operator::F64Add);
        self
    }

    fn f64_sub(&mut self) -> &mut Self {
        self.inject(Operator::F64Sub);
        self
    }

    fn f64_mul(&mut self) -> &mut Self {
        self.inject(Operator::F64Mul);
        self
    }

    fn f64_div(&mut self) -> &mut Self {
        self.inject(Operator::F64Div);
        self
    }

    fn f64_min(&mut self) -> &mut Self {
        self.inject(Operator::F64Min);
        self
    }

    fn f64_max(&mut self) -> &mut Self {
        self.inject(Operator::F64Max);
        self
    }

    fn f64_eq(&mut self) -> &mut Self {
        self.inject(Operator::F64Eq);
        self
    }

    fn f64_ne(&mut self) -> &mut Self {
        self.inject(Operator::F64Ne);
        self
    }

    fn f64_gt(&mut self) -> &mut Self {
        self.inject(Operator::F64Gt);
        self
    }

    fn f64_ge(&mut self) -> &mut Self {
        self.inject(Operator::F64Ge);
        self
    }

    fn f64_lt(&mut self) -> &mut Self {
        self.inject(Operator::F64Lt);
        self
    }

    fn f64_le(&mut self) -> &mut Self {
        self.inject(Operator::F64Le);
        self
    }

    // Memory Instructions
    fn memory_init(&mut self, data_index: u32, mem: u32) -> &mut Self {
        self.inject(Operator::MemoryInit { data_index, mem });
        self
    }

    fn memory_size(&mut self, mem: u32) -> &mut Self {
        self.inject(Operator::MemorySize { mem });
        self
    }

    fn memory_grow(&mut self, mem: u32) -> &mut Self {
        self.inject(Operator::MemoryGrow { mem });
        self
    }

    fn memory_fill(&mut self, mem: u32) -> &mut Self {
        self.inject(Operator::MemoryFill { mem });
        self
    }

    fn memory_copy(&mut self, dst_mem: u32, src_mem: u32) -> &mut Self {
        self.inject(Operator::MemoryCopy { dst_mem, src_mem });
        self
    }

    fn memory_discard(&mut self, mem: u32) -> &mut Self {
        self.inject(Operator::MemoryDiscard { mem });
        self
    }

    fn data_drop(&mut self, data_index: u32) -> &mut Self {
        self.inject(Operator::DataDrop { data_index });
        self
    }

    // Parametric Instructions
    fn drop(&mut self) -> &mut Self {
        self.inject(Operator::Drop);
        self
    }
}

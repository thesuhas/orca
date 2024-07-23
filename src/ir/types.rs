//! Intermediate representation of sections in a wasm module.
use crate::error::Error;
use std::fmt::Formatter;
use std::fmt::{self};
use std::mem::discriminant;
use wasm_encoder::reencode::Reencode;
use wasm_encoder::AbstractHeapType;
use wasmparser::{ConstExpr, GlobalType, Operator, RefType, ValType};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
/// Globals in a wasm module.
pub struct Global {
    pub ty: GlobalType,
    pub init_expr: InitExpr,
}

impl Global {
    pub(crate) fn from_wasmparser(global: wasmparser::Global) -> Result<Global> {
        let ty = global.ty;
        let init_expr = InitExpr::eval(&global.init_expr);
        Ok(Global { ty, init_expr })
    }
}

/// Orca's representation of ValType
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DataType {
    I32,
    I64,
    F32,
    F64,
    V128,
    FuncRef,
    ExternRef,
    Any,
    None,
    NoExtern,
    NoFunc,
    Eq,
    Struct,
    Array,
    I31,
    Exn,
    NoExn,
    Module(u32),
    RecGroup(u32),
    CoreTypeId(u32), // TODO: Look at this
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            DataType::I32 => write!(f, "DataType: I32"),
            DataType::I64 => write!(f, "DataType: I64"),
            DataType::F32 => write!(f, "DataType: F32"),
            DataType::F64 => write!(f, "DataType: F64"),
            DataType::V128 => write!(f, "DataType: V128"),
            DataType::FuncRef => write!(f, "DataType: FuncRef"),
            DataType::ExternRef => write!(f, "DataType: ExternRef"),
            DataType::Any => write!(f, "DataType: Any"),
            DataType::None => write!(f, "DataType: None"),
            DataType::NoExtern => write!(f, "DataType: NoExtern"),
            DataType::NoFunc => write!(f, "DataType: NoFunc"),
            DataType::Eq => write!(f, "DataType: Eq"),
            DataType::Struct => write!(f, "DataType: Struct"),
            DataType::Array => write!(f, "DataType: Array"),
            DataType::I31 => write!(f, "DataType: I31"),
            DataType::Exn => write!(f, "DataType: Exn"),
            DataType::NoExn => write!(f, "DataType: NoExn"),
            DataType::Module(idx) => write!(f, "DataType: Module {:?}", idx),
            DataType::RecGroup(idx) => write!(f, "DataType: RecGroup {:?}", idx),
            DataType::CoreTypeId(idx) => write!(f, "DataType: CoreTypeId {:?}", idx),
        }
    }
}

impl From<ValType> for DataType {
    fn from(value: ValType) -> Self {
        match value {
            ValType::I32 => DataType::I32,
            ValType::I64 => DataType::I64,
            ValType::F32 => DataType::F32,
            ValType::F64 => DataType::F64,
            ValType::V128 => DataType::V128,
            ValType::Ref(reftype) => match reftype.heap_type() {
                wasmparser::HeapType::Abstract { shared: _, ty } => match ty {
                    wasmparser::AbstractHeapType::Func => DataType::FuncRef,
                    wasmparser::AbstractHeapType::Extern => DataType::ExternRef,
                    wasmparser::AbstractHeapType::Any => DataType::Any,
                    wasmparser::AbstractHeapType::None => DataType::None,
                    wasmparser::AbstractHeapType::NoExtern => DataType::NoExtern,
                    wasmparser::AbstractHeapType::NoFunc => DataType::NoFunc,
                    wasmparser::AbstractHeapType::Eq => DataType::Eq,
                    wasmparser::AbstractHeapType::Struct => DataType::Struct,
                    wasmparser::AbstractHeapType::Array => DataType::Array,
                    wasmparser::AbstractHeapType::I31 => DataType::I31,
                    wasmparser::AbstractHeapType::Exn => DataType::Exn,
                    wasmparser::AbstractHeapType::NoExn => DataType::NoExn,
                },
                wasmparser::HeapType::Concrete(u) => match u {
                    wasmparser::UnpackedIndex::Module(idx) => DataType::Module(idx),
                    wasmparser::UnpackedIndex::RecGroup(idx) => DataType::RecGroup(idx),
                    wasmparser::UnpackedIndex::Id(_id) => panic!("Not supported yet!"),
                },
            },
        }
    }
}

/// Converts from Orca's DataType to wasm_encoder::ValType
impl From<&DataType> for wasm_encoder::ValType {
    fn from(ty: &DataType) -> Self {
        match ty {
            DataType::I32 => wasm_encoder::ValType::I32,
            DataType::I64 => wasm_encoder::ValType::I64,
            DataType::F32 => wasm_encoder::ValType::F32,
            DataType::F64 => wasm_encoder::ValType::F64,
            DataType::V128 => wasm_encoder::ValType::V128,
            DataType::FuncRef => wasm_encoder::ValType::FUNCREF,
            DataType::ExternRef => wasm_encoder::ValType::EXTERNREF,
            DataType::Any => wasm_encoder::ValType::Ref(wasm_encoder::RefType {
                nullable: false,
                heap_type: wasm_encoder::HeapType::Abstract {
                    shared: false,
                    ty: AbstractHeapType::Any,
                },
            }),
            DataType::None => wasm_encoder::ValType::Ref(wasm_encoder::RefType {
                nullable: false,
                heap_type: wasm_encoder::HeapType::Abstract {
                    shared: false,
                    ty: AbstractHeapType::None,
                },
            }),
            DataType::NoExtern => wasm_encoder::ValType::Ref(wasm_encoder::RefType {
                nullable: false,
                heap_type: wasm_encoder::HeapType::Abstract {
                    shared: false,
                    ty: AbstractHeapType::NoExtern,
                },
            }),
            DataType::NoFunc => wasm_encoder::ValType::Ref(wasm_encoder::RefType {
                nullable: false,
                heap_type: wasm_encoder::HeapType::Abstract {
                    shared: false,
                    ty: AbstractHeapType::NoFunc,
                },
            }),
            DataType::Eq => wasm_encoder::ValType::Ref(wasm_encoder::RefType {
                nullable: false,
                heap_type: wasm_encoder::HeapType::Abstract {
                    shared: false,
                    ty: AbstractHeapType::Eq,
                },
            }),
            DataType::Struct => wasm_encoder::ValType::Ref(wasm_encoder::RefType {
                nullable: false,
                heap_type: wasm_encoder::HeapType::Abstract {
                    shared: false,
                    ty: AbstractHeapType::Struct,
                },
            }),
            DataType::Array => wasm_encoder::ValType::Ref(wasm_encoder::RefType {
                nullable: false,
                heap_type: wasm_encoder::HeapType::Abstract {
                    shared: false,
                    ty: AbstractHeapType::Array,
                },
            }),
            DataType::I31 => wasm_encoder::ValType::Ref(wasm_encoder::RefType {
                nullable: false,
                heap_type: wasm_encoder::HeapType::Abstract {
                    shared: false,
                    ty: AbstractHeapType::I31,
                },
            }),
            DataType::Exn => wasm_encoder::ValType::Ref(wasm_encoder::RefType {
                nullable: false,
                heap_type: wasm_encoder::HeapType::Abstract {
                    shared: false,
                    ty: AbstractHeapType::Exn,
                },
            }),
            DataType::NoExn => wasm_encoder::ValType::Ref(wasm_encoder::RefType {
                nullable: false,
                heap_type: wasm_encoder::HeapType::Abstract {
                    shared: false,
                    ty: AbstractHeapType::NoExn,
                },
            }),
            DataType::Module(idx) => wasm_encoder::ValType::Ref(wasm_encoder::RefType {
                nullable: false,
                heap_type: wasm_encoder::HeapType::Concrete(*idx),
            }),
            DataType::RecGroup(idx) => wasm_encoder::ValType::Ref(wasm_encoder::RefType {
                nullable: false,
                heap_type: wasm_encoder::HeapType::Concrete(*idx),
            }),
            DataType::CoreTypeId(idx) => wasm_encoder::ValType::Ref(wasm_encoder::RefType {
                nullable: false,
                heap_type: wasm_encoder::HeapType::Concrete(*idx),
            }),
        }
    }
}

/// Converts wasmparser::ValType to wasm_encoder::ValType
pub fn valtype_to_wasmencoder_type(val_type: &ValType) -> wasm_encoder::ValType {
    let mut reencoder = wasm_encoder::reencode::RoundtripReencoder;
    reencoder.val_type(*val_type).unwrap()
}

/// orca's representation of function types, shortened from walrus
/// https://docs.rs/walrus/latest/walrus/struct.Type.html
#[derive(Debug, Clone)]
pub struct FuncType {
    pub params: Box<[DataType]>,
    pub results: Box<[DataType]>,
}

impl FuncType {
    pub fn new(params: Box<[DataType]>, results: Box<[DataType]>) -> Self {
        Self { params, results }
    }
}

#[derive(Debug, Clone)]
/// Data Segment in a wasm module.
pub struct DataSegment {
    /// The kind of data segment.
    pub kind: DataSegmentKind,
    /// The data of the data segment.
    pub data: Vec<u8>,
}

impl DataSegment {
    pub fn from_wasmparser(data: wasmparser::Data) -> Result<DataSegment> {
        Ok(DataSegment {
            kind: DataSegmentKind::from_wasmparser(data.kind)?,
            data: data.data.to_vec(),
        })
    }
}

/// The kind of data segment.
#[derive(Debug, Clone)]
pub enum DataSegmentKind {
    /// The data segment is passive.
    Passive,
    /// The data segment is active.
    Active {
        /// The memory index for the data segment.
        memory_index: u32,
        /// The memory offset where this active data segment will be automatically
        /// initialized.
        offset_expr: InitExpr,
    },
}

impl DataSegmentKind {
    pub(crate) fn from_wasmparser(kind: wasmparser::DataKind) -> Result<DataSegmentKind> {
        Ok(match kind {
            wasmparser::DataKind::Passive => DataSegmentKind::Passive,
            wasmparser::DataKind::Active {
                memory_index,
                offset_expr,
            } => DataSegmentKind::Active {
                memory_index,
                offset_expr: InitExpr::eval(&offset_expr),
            },
        })
    }
}

#[derive(Debug, Clone)]
/// Kind of Element
pub enum ElementKind<'a> {
    Passive,
    Active {
        table_index: Option<u32>,
        offset_expr: ConstExpr<'a>,
    },
    Declared,
}

impl ElementKind<'_> {
    pub(crate) fn from_wasmparser(kind: wasmparser::ElementKind) -> Result<ElementKind> {
        match kind {
            wasmparser::ElementKind::Passive => Ok(ElementKind::Passive),
            wasmparser::ElementKind::Declared => Ok(ElementKind::Declared),
            wasmparser::ElementKind::Active {
                table_index,
                offset_expr,
            } => Ok(ElementKind::Active {
                table_index,
                offset_expr,
            }),
        }
    }
}

#[derive(Debug, Clone)]
/// Type of element
pub enum ElementItems<'a> {
    Functions(Vec<u32>),
    ConstExprs {
        ty: RefType,
        exprs: Vec<ConstExpr<'a>>,
    },
}

impl ElementItems<'_> {
    pub(crate) fn from_wasmparser(items: wasmparser::ElementItems) -> Result<ElementItems> {
        match items {
            wasmparser::ElementItems::Functions(reader) => {
                let functions = reader
                    .into_iter()
                    .collect::<std::result::Result<Vec<_>, _>>()?;
                Ok(ElementItems::Functions(functions))
            }
            wasmparser::ElementItems::Expressions(ref_type, reader) => {
                let exprs = reader
                    .into_iter()
                    .collect::<std::result::Result<Vec<_>, _>>()?;
                Ok(ElementItems::ConstExprs {
                    ty: ref_type,
                    exprs,
                })
            }
        }
    }
}

#[derive(Debug, Clone)]
/// Instrumentation Data that is stored with every instruction
pub enum Instrument<'a> {
    NotInstrumented,
    Instrumented {
        before: Vec<Operator<'a>>,
        after: Vec<Operator<'a>>,
        alternate: Vec<Operator<'a>>,
        current: InstrumentationMode,
    },
}

#[derive(Debug, Clone)]
/// Mode of Instruction in case the instruction is marked as Instrumented
pub enum InstrumentationMode {
    Before,
    After,
    Alternate,
}

impl fmt::Display for Instrument<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Instrument::NotInstrumented => write!(f, "Not Instrumented"),
            Instrument::Instrumented {before, after, alternate, current: _current} => write!(f, "Instrumented:\n Before: {:?} instructions\n After: {:?} instructions\n Alternate: {:?} instructions\n", before.len(), after.len(), alternate.len())
        }
    }
}

impl PartialEq for Instrument<'_> {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl Eq for Instrument<'_> {}

impl<'a> Instrument<'a> {
    /// Add an instruction to the current InstrumentationMode's list
    pub fn add_instr(&mut self, val: Operator<'a>) {
        match self {
            Instrument::NotInstrumented => {
                panic!("Cannot inject code into locations that are not instrumented!")
            }
            Instrument::Instrumented {
                before,
                after,
                alternate,
                current,
            } => match current {
                InstrumentationMode::Before => before.push(val),
                InstrumentationMode::After => after.push(val),
                InstrumentationMode::Alternate => alternate.push(val),
            },
        }
    }

    /// Get an instruction to the current InstrumentationMode's list
    pub fn get_instr(&self, idx: usize) -> &Operator {
        match self {
            Instrument::NotInstrumented => {
                panic!("Cannot retrieve code from Instruction that was not instrumented!")
            }
            Instrument::Instrumented {
                before,
                after,
                alternate,
                current,
            } => match current {
                InstrumentationMode::Before => before.get(idx).unwrap(),
                InstrumentationMode::After => after.get(idx).unwrap(),
                InstrumentationMode::Alternate => alternate.get(idx).unwrap(),
            },
        }
    }

    /// Set the current Instrumentation Mode on instrumented instructions. Can also be used to switch Instrumentation Modes at the current location
    pub fn set_curr(&mut self, ty: InstrumentationMode) {
        match self {
            Instrument::NotInstrumented => panic!(
                "Cannot set current instrumentation type on instruction that are not instrumented!"
            ),
            Instrument::Instrumented {
                before: _before,
                after: _after,
                alternate: _alternate,
                current,
            } => *current = ty,
        }
    }

    /// Get the current Instrumentation Mode
    pub fn get_curr(&self) -> InstrumentType {
        match self {
            Instrument::NotInstrumented => InstrumentType::NotInstrumented,
            Instrument::Instrumented {
                before: _before,
                after: _after,
                alternate: _alternate,
                current,
            } => match current {
                InstrumentationMode::Before => InstrumentType::InstrumentBefore,
                InstrumentationMode::After => InstrumentType::InstrumentAfter,
                InstrumentationMode::Alternate => InstrumentType::InstrumentAlternate,
            },
        }
    }
}

#[derive(Debug, Clone)]
/// Type of instrumentation to be applied to an instruction.
pub enum InstrumentType {
    InstrumentBefore,
    InstrumentAfter,
    InstrumentAlternate,
    NotInstrumented,
}

impl fmt::Display for InstrumentType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            InstrumentType::InstrumentBefore => write!(f, "Instrument Before"),
            InstrumentType::InstrumentAfter => write!(f, "Instrument After"),
            InstrumentType::InstrumentAlternate => write!(f, "Instrument Alternate"),
            InstrumentType::NotInstrumented => write!(f, "Not Instrumented"),
        }
    }
}

impl PartialEq for InstrumentType {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

impl Eq for InstrumentType {}

/// Used to represent a unique location in a wasm component or module.
#[derive(Debug, Clone, Copy)]
pub enum Location {
    Component {
        mod_idx: usize,
        func_idx: usize,
        instr_idx: usize,
    },
    Module {
        func_idx: usize,
        instr_idx: usize,
    },
}

/// Represents an import in a WebAssembly module.
#[derive(Debug, Clone)]
pub struct Import<'a> {
    /// The module being imported from.
    pub module: &'a str,
    /// The name of the imported item.
    pub name: &'a str,
    /// The type of the imported item.
    pub ty: wasmparser::TypeRef,
    /// The name (in the custom section) of the imported item.
    pub import_name: Option<String>,
}

impl Import<'_> {
    pub(crate) fn from_wasmparser(import: wasmparser::Import) -> Import {
        Import {
            module: import.module,
            name: import.name,
            ty: import.ty,
            import_name: None,
        }
    }

    pub fn is_function(&self) -> bool {
        matches!(self.ty, wasmparser::TypeRef::Func(_))
    }
}

#[derive(Debug, Clone)]
/// Body of a function in a wasm module
pub struct Body<'a> {
    /// Local variables of the function, given as tuples of (# of locals, type).
    /// Note that these do not include the function parameters which are given
    /// indices before the locals. So if a function has 2 parameters and a local
    /// defined here then local indices 0 and 1 will refer to the parameters and
    /// index 2 will refer to the local here.
    // TODO: this representation is a bit weird, why it is like this?
    pub locals: Vec<(u32, DataType)>,
    pub num_locals: usize,
    // accessing operators by .0 is not very clear
    pub instructions: Vec<(Operator<'a>, Instrument<'a>)>,
    pub num_instructions: usize,
    pub name: Option<String>,
}

#[allow(clippy::new_without_default)]
// 'b should outlive 'a
impl<'a, 'b> Body<'a>
where
    'b: 'a,
{
    pub fn new() -> Self {
        Self {
            locals: Vec::new(),
            num_locals: 0,
            instructions: Vec::new(),
            num_instructions: 0,
            name: None,
        }
    }

    pub fn push_instr(&mut self, instr: Operator<'b>) {
        self.instructions.push((instr, Instrument::NotInstrumented));
        self.num_instructions += 1;
    }

    pub fn get_instr(&self, idx: usize) -> &Operator {
        &self.instructions[idx].0
    }

    pub fn get_instr_type(&self, idx: usize) -> &Instrument {
        &self.instructions[idx].1
    }

    pub fn end(&mut self) {
        self.push_instr(Operator::End);
    }
}

/// A constant which is produced in WebAssembly, typically used in global
/// initializers or element/data offsets.
#[derive(Debug, Copy, Clone)]
pub enum InitExpr {
    /// An immediate constant value
    Value(Value),
    /// A constant value referenced by the global specified
    Global(u32),
    /// A null reference
    RefNull(RefType),
    /// A function initializer
    RefFunc(u32),
}

impl InitExpr {
    pub(crate) fn eval(init: &ConstExpr) -> InitExpr {
        use wasmparser::Operator::*;
        let mut reader = init.get_operators_reader();
        let val = match reader.read().unwrap() {
            I32Const { value } => InitExpr::Value(Value::I32(value)),
            I64Const { value } => InitExpr::Value(Value::I64(value)),
            F32Const { value } => InitExpr::Value(Value::F32(f32::from_bits(value.bits()))),
            F64Const { value } => InitExpr::Value(Value::F64(f64::from_bits(value.bits()))),
            V128Const { value } => InitExpr::Value(Value::V128(v128_to_u128(&value))),
            GlobalGet { global_index } => InitExpr::Global(global_index),
            // Marking nullable as true as it's a null reference
            RefNull { hty } => InitExpr::RefNull(RefType::new(true, hty).unwrap()),
            RefFunc { function_index } => InitExpr::RefFunc(function_index),
            _ => panic!("invalid constant expression"),
        };
        match reader.read().unwrap() {
            End => {}
            _ => panic!("invalid constant expression"),
        }
        reader.ensure_end().unwrap();
        val
    }

    pub(crate) fn to_wasmencoder_type(self) -> wasm_encoder::ConstExpr {
        match self {
            InitExpr::Value(v) => match v {
                Value::I32(v) => wasm_encoder::ConstExpr::i32_const(v),
                Value::I64(v) => wasm_encoder::ConstExpr::i64_const(v),
                Value::F32(v) => wasm_encoder::ConstExpr::f32_const(v),
                Value::F64(v) => wasm_encoder::ConstExpr::f64_const(v),
                Value::V128(v) => wasm_encoder::ConstExpr::v128_const(v as i128),
            },
            InitExpr::Global(g) => wasm_encoder::ConstExpr::global_get(g),
            InitExpr::RefNull(ty) => wasm_encoder::ConstExpr::ref_null(if ty.is_func_ref() {
                wasm_encoder::HeapType::Abstract {
                    shared: false,
                    ty: AbstractHeapType::Func,
                }
            } else if ty.is_extern_ref() {
                wasm_encoder::HeapType::Abstract {
                    shared: false,
                    ty: AbstractHeapType::Extern,
                }
            } else {
                unreachable!()
            }),
            InitExpr::RefFunc(f) => wasm_encoder::ConstExpr::ref_func(f),
        }
    }
}

/// Constant values that can show up in WebAssembly
#[derive(Debug, Clone, Copy)]
pub enum Value {
    /// A constant 32-bit integer
    I32(i32),
    /// A constant 64-bit integer
    I64(i64),
    /// A constant 32-bit float
    F32(f32),
    /// A constant 64-bit float
    F64(f64),
    /// A constant 128-bit vector register
    V128(u128),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Value::I32(i) => i.fmt(f),
            Value::I64(i) => i.fmt(f),
            Value::F32(i) => i.fmt(f),
            Value::F64(i) => i.fmt(f),
            Value::V128(i) => i.fmt(f),
        }
    }
}

/// LocalID in a function
#[derive(Debug, Clone, Copy)]
pub struct LocalID {
    pub id: usize,
}

impl fmt::Display for LocalID {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Local ID: {:?}", self.id)
    }
}

/// TypeID in a module
#[derive(Debug, Clone, Copy)]
pub struct TypeID {
    pub id: usize,
}

impl fmt::Display for TypeID {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Type ID: {:?}", self.id)
    }
}

/// ModuleID in a Component
#[derive(Debug, Clone, Copy)]
pub struct ModuleID {
    pub id: usize,
}

impl fmt::Display for ModuleID {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Module ID: {:?}", self.id)
    }
}

/// FunctionID in a module
#[derive(Debug, Clone, Copy)]
pub struct FunctionID {
    pub id: usize,
}

impl fmt::Display for FunctionID {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Function ID: {:?}", self.id)
    }
}

/// DataSegmentID
#[derive(Debug, Clone, Copy)]
pub struct DataSegmentID {
    pub id: usize,
}

impl fmt::Display for DataSegmentID {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Data Segment ID: {:?}", self.id)
    }
}

/// GlobalsID
#[derive(Debug, Clone, Copy)]
pub struct GlobalsID {
    pub id: usize,
}

impl fmt::Display for GlobalsID {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Globals ID: {:?}", self.id)
    }
}

/// ImportsID
#[derive(Debug, Clone, Copy)]
pub struct ImportsID {
    pub id: usize,
}

impl fmt::Display for ImportsID {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Imports ID: {:?}", self.id)
    }
}

/// ExportsID
#[derive(Debug, Clone, Copy)]
pub struct ExportsID {
    pub id: usize,
}

impl fmt::Display for ExportsID {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Exports ID: {:?}", self.id)
    }
}

#[allow(clippy::identity_op)]
pub(crate) fn v128_to_u128(value: &wasmparser::V128) -> u128 {
    let n = value.bytes();
    ((n[0] as u128) << 0)
        | ((n[1] as u128) << 8)
        | ((n[2] as u128) << 16)
        | ((n[3] as u128) << 24)
        | ((n[4] as u128) << 32)
        | ((n[5] as u128) << 40)
        | ((n[6] as u128) << 48)
        | ((n[7] as u128) << 56)
        | ((n[8] as u128) << 64)
        | ((n[9] as u128) << 72)
        | ((n[10] as u128) << 80)
        | ((n[11] as u128) << 88)
        | ((n[12] as u128) << 96)
        | ((n[13] as u128) << 104)
        | ((n[14] as u128) << 112)
        | ((n[15] as u128) << 120)
}

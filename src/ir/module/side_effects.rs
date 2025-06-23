use crate::ir::module::module_types::Types;
use crate::ir::types::{FuncInstrMode, InitExpr, InstrumentationMode, Tag};
use crate::DataType;
use wasmparser::{ExternalKind, Operator, TypeRef};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum InjectType {
    // Module additions
    Import,
    Export,
    Memory,
    Data,
    Global,
    Func,
    Local,
    Table,
    Element,

    // Probes
    Probe,
}

pub enum Injection<'a> {
    /// Represents an import that has been added to the module.
    Import {
        /// The module being imported from.
        module: String,
        /// The name of the imported item.
        name: String,
        /// The type of the import.
        type_ref: TypeRef,
        tag: Tag,
    },
    /// Represents an export that has been added to the module.
    Export {
        /// The name of the exported item.
        name: String,
        /// The kind of the exported item.
        kind: ExternalKind,
        /// The index of the exported item.
        index: u32,
        tag: Tag,
    },
    Type {
        ty: Types,
        tag: Tag,
    },
    /// Represents a memory that has been added to the module.
    Memory {
        /// The memory's ID.
        id: u32, // TODO -- may not need (it's ordered in a vec)
        /// The initial number of pages for this memory.
        initial: u64,
        /// The maximum number of pages for this memory.
        maximum: Option<u64>,
        tag: Tag,
    },
    /// Represents a data segment that has been added to the module.
    Data {
        /// The memory index for the data segment.
        memory_index: u32,
        /// The memory offset where this active data segment will be automatically
        /// initialized.
        /// Contains the offset expression for this item.
        offset_expr: InitExpr,
        /// The data of the data segment.
        data: Vec<u8>,
        tag: Tag,
    },

    /// Represents a global that has been added to the module.
    Global {
        /// The global's ID.
        id: u32, // TODO -- may not need (it's ordered in a vec)
        /// The global's type.
        ty: DataType,
        /// Whether the global is shared.
        shared: bool,
        /// Whether the global is mutable.
        mutable: bool,
        /// Contains the init expression for this item.
        init_expr: InitExpr,
        tag: Tag,
    },
    /// Represents a local function that has been added to the module.
    Func {
        /// The function's ID.
        id: u32,
        /// The function's name.
        fname: Option<String>,
        /// The function's signature (params, results).
        sig: (Vec<DataType>, Vec<DataType>),
        /// The function's local variables
        locals: Vec<DataType>,
        /// The body of the function (in WAT).
        body: Vec<String>,
        tag: Tag,
    },
    /// Represents a local variable that has been added to a module's local function.
    Local {
        /// The ID of the function this probe is inserted into.
        target_fid: u32,
        ty: DataType,
        tag: Tag,
    },
    /// Represents a table that has been added to the module.
    Table {
        tag: Tag,
    },
    /// Represents a table element that has been added to the module.
    Element {
        tag: Tag,
    },

    // Probes
    /// Represents a probe that has been injected into the module at a specific location in a function.
    FuncBodyProbe {
        /// The ID of the function this probe is inserted into.
        target_fid: u32,
        /// The opcode offset in the target that this probe is inserted at.
        target_opcode_idx: u32,
        /// The mode of the probe to use during insertion.
        mode: InstrumentationMode,
        /// The body of the probe.
        body: Vec<Operator<'a>>,
        tag: Tag,
    },
    /// Represents a probe that has been injected into the module's function, at the function-level.
    FuncProbe {
        /// The ID of the function this probe is inserted into.
        target_fid: u32,
        /// The mode of the probe to use during insertion.
        mode: FuncInstrMode,
        /// The body of the probe.
        body: Vec<Operator<'a>>,
        tag: Tag,
    },
}

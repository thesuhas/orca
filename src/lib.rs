//! Orca is a WebAssembly Transformation Library for the Component Model. The design has been inspired by [Dfinity's IC]
//! and [Walrus].
//!
//! [Dfinity's IC]: https://github.com/dfinity/ic/tree/master/rs/wasm_transform
//! [Walrus]: https://github.com/rustwasm/walrus/tree/main

mod error;
pub mod ir;
pub mod iterator;
pub mod module_builder;
pub mod opcode;
pub mod subiterator;

pub use crate::opcode::Opcode;

pub use crate::ir::component::Component;
// pub use crate::ir::function::FunctionBuilder;
pub use crate::ir::module::Module;
pub use crate::ir::types::DataSegment;
pub use crate::ir::types::DataSegmentKind;
pub use crate::ir::types::DataType;
pub use crate::ir::types::InitExpr;
pub use crate::ir::types::Location;

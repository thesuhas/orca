//! Orca is a WebAssembly Transformation Library for the Component Model. The design has been inspired by [Dfinity's IC]
//! and [Walrus].
//!
//! [Dfinity's IC]: https://github.com/dfinity/ic/tree/master/rs/wasm_transform
//! [Walrus]: https://github.com/rustwasm/walrus/tree/main

mod error;
pub mod ir;
pub mod iterator;
pub mod subiterator;

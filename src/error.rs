use std::ops::Range;
use wasmparser::BinaryReaderError;

use gimli;

#[derive(Debug, Clone)]
#[allow(clippy::enum_variant_names)]

/// Error for parsing
pub enum Error {
    BinaryReaderError(BinaryReaderError),
    UnknownVersion(u32),
    UnknownSection {
        section_id: u8,
    },
    MissingFunctionEnd {
        func_range: Range<usize>,
    },
    IncorrectDataCount {
        declared_count: usize,
        actual_count: usize,
    },
    ConversionError(String),
    IncorrectCodeCounts {
        function_section_count: usize,
        code_section_declared_count: usize,
        code_section_actual_count: usize,
    },
    MultipleStartSections,
    /// `memory.grow` and `memory.size` operations must have a 0x00 byte
    /// immediately after the instruction (it is not valid to have some other
    /// variable length encoding representation of 0). This is because the
    /// immediate byte will be used to reference other memories in the
    /// multi-memory proposal.
    InvalidMemoryReservedByte {
        func_range: Range<usize>,
    },
}

impl From<gimli::Error> for Error {
    fn from(err: gimli::Error) -> Error {
        // You need to map `gimli::Error` to your custom error type here.
        // This is a simple example, adjust based on your `error::Error` definition.
        Self::ConversionError(err.to_string()) // Assuming you have a variant `GimliError` in your `error::Error` enum
    }
}

impl From<BinaryReaderError> for Error {
    fn from(e: BinaryReaderError) -> Self {
        Self::BinaryReaderError(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BinaryReaderError(err) => {
                write!(f, "Error from wasmparser: {}", err)
            }
            Error::UnknownVersion(ver) => {
                write!(f, "Unknown version: {}", ver)
            }
            Error::UnknownSection { section_id } => {
                write!(f, "Unknown section: {}", section_id)
            }
            Error::MissingFunctionEnd { func_range } => {
                write!(
                    f,
                    "Missing function End for function in range {} - {}",
                    func_range.start, func_range.end
                )
            }
            Error::IncorrectDataCount {
                declared_count,
                actual_count,
            } => {
                write!(
                    f,
                    "Incorrect data count. Declared: {}, actual: {}",
                    declared_count, actual_count
                )
            }
            Error::ConversionError(s) => {
                write!(
                    f,
                    "Unable to convert wasmparser type to wasm-encoder: {}",
                    s
                )
            }
            Error::IncorrectCodeCounts {
                function_section_count,
                code_section_declared_count,
                code_section_actual_count,
            } => {
                write!(
                    f,
                    "Incorrect code counts. Function section count: {}, code section declared count: {}, code section actual count: {}",
                    function_section_count, code_section_declared_count, code_section_actual_count
                )
            }
            Error::MultipleStartSections => {
                write!(f, "Multiple start sections")
            }
            Error::InvalidMemoryReservedByte { func_range } => {
                write!(f, "Found a `memory.*` instruction with an invalid reserved byte in function at {:?}", func_range)
            }
        }
    }
}

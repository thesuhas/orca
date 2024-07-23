use std::fmt;
use std::fmt::Formatter;

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
pub struct GlobalID {
    pub id: usize,
}

impl fmt::Display for GlobalID {
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

//! Enums the represent a section of a Module or a Component

#[derive(Debug, Clone, Eq, PartialEq)]
/// Represents a Section in a Component
pub enum ComponentSection {
    Module,
    Alias,
    CoreType,
    ComponentType,
    ComponentImport,
    ComponentExport,
    CoreInstance,
    ComponentInstance,
    Canon,
    CustomSection,
    Component,
    ComponentStartSection,
}

#[derive(Debug, Clone, Eq, PartialEq)]
/// Represents a Section in a Module
pub enum ModuleSection {
    FuncType,
    Import,
    Function,
    Table,
    Memory,
    Global,
    DataSegment,
    Export,
    Start,
    Elements,
    CodeSection,
    CustomSection,
    // ProducerSection
}

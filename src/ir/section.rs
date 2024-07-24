#[derive(Debug, Clone, Eq, PartialEq)]
/// Represents a Section in a Component
pub enum Section {
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
}

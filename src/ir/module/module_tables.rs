//! Intermediate representation of the Tables in a Module

use crate::ir::id::TableID;
use wasmparser::{RefType, TableType};

/// Tables Section of a module
#[derive(Clone, Debug)]
pub struct ModuleTables<'a> {
    tables: Vec<(TableType, Option<wasmparser::ConstExpr<'a>>)>,
}

impl<'a> ModuleTables<'a> {
    /// Create a new table section
    pub fn new(tables: Vec<(TableType, Option<wasmparser::ConstExpr<'a>>)>) -> Self {
        ModuleTables { tables }
    }

    /// Check if there are any tables
    pub fn is_empty(&self) -> bool {
        self.tables.is_empty()
    }

    /// Create an iterable over the table section
    pub fn iter(&self) -> std::slice::Iter<'_, (TableType, Option<wasmparser::ConstExpr<'a>>)> {
        self.tables.iter()
    }

    /// Finds a unique function table in a module.
    ///
    /// Modules produced by compilers like LLVM typically have one function
    /// table for indirect function calls. This function will look for a single
    /// function table inside this module, and return that if found. If no
    /// function tables are present `None` will be returned
    ///
    /// # Errors
    ///
    /// Returns an error if there are two function tables in this module
    ///
    /// Inspired from [walrus' implementation]
    ///
    /// [walrus' implementation]: https://docs.rs/walrus/latest/walrus/struct.ModuleTables.html#method.main_function_table
    pub fn main_function(&self) -> Option<TableID> {
        let mut tables = self
            .tables
            .iter()
            .enumerate()
            .filter(|(_, (t, _))| t.element_type == RefType::FUNCREF);
        let id = match tables.next() {
            Some((index, _)) => Some(index as TableID),
            None => return None,
        };
        if tables.next().is_some() {
            panic!("module contains more than one function table");
        }
        id
    }

    /// Get a table
    pub fn get(&self, table_id: TableID) -> Option<TableType> {
        if table_id < self.tables.len() as u32 {
            return Some(self.tables[table_id as usize].0.clone());
        }
        None
    }

    /// Get a mutable reference to a table
    pub fn get_mut(&mut self, table_id: TableID) -> &mut TableType {
        if table_id < self.tables.len() as u32 {
            return &mut self.tables[table_id as usize].0;
        }
        panic!("Invalid Table ID")
    }
}

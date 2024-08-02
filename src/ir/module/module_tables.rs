use crate::ir::id::TableID;
use wasmparser::{RefType, TableType};

#[derive(Clone, Debug)]
pub struct ModuleTables<'a> {
    tables: Vec<(TableType, Option<wasmparser::ConstExpr<'a>>)>,
}

impl<'a> ModuleTables<'a> {
    pub fn new(tables: Vec<(TableType, Option<wasmparser::ConstExpr<'a>>)>) -> Self {
        ModuleTables { tables }
    }

    pub fn is_empty(&self) -> bool {
        self.tables.is_empty()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, (TableType, Option<wasmparser::ConstExpr<'a>>)> {
        self.tables.iter()
    }

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

    pub fn get(&self, table_id: TableID) -> Option<TableType> {
        if table_id < self.tables.len() as u32 {
            return Some(self.tables[table_id as usize].0.clone());
        }
        None
    }
}

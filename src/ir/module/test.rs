// ==================================
// ==== Manipulation API Testing ====
// ==================================

use crate::ir::function::FunctionBuilder;
use crate::ir::id::{FunctionID, ImportsID};
use crate::{DataType, InitExpr, Module, Opcode};
use log::debug;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

// FUNCTIONS
#[test]
fn test_add_local_func() {
    let (buff, mut init_state) = setup();
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    state_assertions(&module, &init_state, false);

    // add local func
    let mut builder = FunctionBuilder::new(&*vec![], &*vec![]);
    builder.i32_const(1);
    builder.drop();
    assert_eq!(
        init_state.func_count + 1 as FunctionID,
        builder.finish_module(&mut module)
    );
    init_state.add_local_func();

    is_valid(
        &mut module,
        &mut init_state,
        &HashMap::new(),
        &HashMap::new(),
        "test_add_local_func",
    );
}

#[test]
fn test_add_import_func() {
    let (buff, mut init_state) = setup();
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    state_assertions(&module, &init_state, false);

    // add imported func
    let (fid, imp0) = module.add_import_func("test0".to_string(), "func0".to_string(), 0);
    assert_eq!(init_state.next_fid(), fid); // zero-based, no '+ 1'
    assert_eq!(init_state.next_imp_id(), imp0); // zero-based, no '+ 1'
    init_state.add_imported_func();

    is_valid(
        &mut module,
        &mut init_state,
        &HashMap::new(),
        &HashMap::new(),
        "test_add_import_func",
    );
}

#[test]
fn test_add_local_then_imported_func() {
    let (buff, mut init_state) = setup();
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    state_assertions(&module, &init_state, false);

    // add local function
    let mut builder = FunctionBuilder::new(&*vec![], &*vec![]);
    builder.i32_const(1);
    builder.drop();
    assert_eq!(init_state.next_fid(), builder.finish_module(&mut module));
    init_state.add_local_func();

    // add imported func
    let (fid, imp0) = module.add_import_func("test0".to_string(), "func0".to_string(), 0);
    assert_eq!(init_state.next_fid(), fid); // zero-based, no '+ 1'
    assert_eq!(init_state.next_imp_id(), imp0); // zero-based, no '+ 1'
    init_state.add_imported_func();

    is_valid(
        &mut module,
        &mut init_state,
        &HashMap::new(),
        &HashMap::new(),
        "test_add_local_then_imported_func",
    );
}

#[test]
fn test_add_imported_then_local_func() {
    let (buff, mut init_state) = setup();
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    state_assertions(&module, &init_state, false);

    // add imported func
    let (fid, imp0) = module.add_import_func("test0".to_string(), "func0".to_string(), 0);
    assert_eq!(init_state.next_fid(), fid); // zero-based, no '+ 1'
    assert_eq!(init_state.next_imp_id(), imp0); // zero-based, no '+ 1'
    init_state.add_imported_func();

    // add local function using the imported function
    let mut builder = FunctionBuilder::new(&vec![], &vec![DataType::I32]);
    builder.i32_const(1);
    builder.i32_const(1);
    builder.call(fid);
    assert_eq!(init_state.next_fid(), builder.finish_module(&mut module));
    init_state.add_local_func();

    is_valid(
        &mut module,
        &mut init_state,
        &HashMap::new(),
        &HashMap::new(),
        "test_add_imported_then_local_func",
    );
}

#[test]
fn test_add_then_delete_local_func() {
    let (buff, mut init_state) = setup();
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    state_assertions(&module, &init_state, false);

    // add local function
    let mut builder = FunctionBuilder::new(&*vec![], &*vec![]);
    builder.i32_const(1);
    builder.drop();
    let fid = builder.finish_module(&mut module);
    assert_eq!(init_state.next_fid(), fid);
    init_state.add_local_func();

    module.delete_func(fid);
    assert!(module.functions.is_deleted(fid));
    init_state.delete_local_func();

    is_valid(
        &mut module,
        &mut init_state,
        &HashMap::new(),
        &HashMap::new(),
        "test_add_then_delete_local_func",
    );
}

#[test]
fn test_delete_local_func() {
    let (buff, mut init_state) = setup();
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    state_assertions(&module, &init_state, false);

    // delete local function
    module.delete_func(52); // unused in wat file!
    init_state.delete_local_func();

    module.exports.delete(49);

    is_valid(
        &mut module,
        &mut init_state,
        &HashMap::new(),
        &HashMap::new(),
        "test_delete_local_func",
    );
}

#[test]
fn test_add_then_delete_imported_func() {
    let (buff, mut init_state) = setup();
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    state_assertions(&module, &init_state, false);

    // add imported func
    let (fid, imp0) = module.add_import_func("test0".to_string(), "func0".to_string(), 0);
    assert_eq!(init_state.next_fid(), fid); // zero-based, no '+ 1'
    assert_eq!(init_state.next_imp_id(), imp0); // zero-based, no '+ 1'
    init_state.add_imported_func();

    // delete imported function
    module.delete_func(fid); // unused in wat file!
    init_state.delete_imported_func();

    is_valid(
        &mut module,
        &mut init_state,
        &HashMap::new(),
        &HashMap::new(),
        "test_add_then_delete_imported_func",
    );
}

#[test]
fn test_delete_imported_func() {
    let (buff, mut init_state) = setup();
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    state_assertions(&module, &init_state, false);

    // delete imported function
    module.delete_func(0); // unused in wat file!
    init_state.delete_imported_func();

    is_valid(
        &mut module,
        &mut init_state,
        &HashMap::new(),
        &HashMap::new(),
        "test_delete_imported_func",
    );
}

#[test]
fn test_delete_local_and_imported_func() {
    let (buff, mut init_state) = setup();
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    state_assertions(&module, &init_state, false);

    // delete local function
    module.delete_func(52); // unused in wat file!
    init_state.delete_local_func();

    module.exports.delete(49);

    // delete imported function
    module.delete_func(0); // unused in wat file!
    init_state.delete_imported_func();

    is_valid(
        &mut module,
        &mut init_state,
        &HashMap::new(),
        &HashMap::new(),
        "test_delete_local_and_imported_func",
    );
}

#[test]
fn test_convert_import_fn_to_local() {
    let (buff, mut init_state) = setup();
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    state_assertions(&module, &init_state, false);

    // convert the import to a function
    let mut builder = FunctionBuilder::new(&*vec![DataType::I32], &*vec![DataType::I32]);
    builder.i32_const(1);
    builder.drop();
    builder.replace_import_in_module(&mut module, 0);

    // add local function using the translated function
    let mut builder = FunctionBuilder::new(&vec![], &vec![DataType::I32]);
    builder.i32_const(1);
    builder.i32_const(1);
    builder.call(0);
    assert_eq!(init_state.next_fid(), builder.finish_module(&mut module));
    init_state.add_local_func();
    init_state.import_func_to_local();

    is_valid(
        &mut module,
        &mut init_state,
        &HashMap::new(),
        &HashMap::new(),
        "test_convert_import_fn_to_local",
    );
}

#[test]
fn test_convert_local_fn_to_import() {
    let (buff, mut init_state) = setup();
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    state_assertions(&module, &init_state, false);

    // convert local func to import
    module.convert_local_fn_to_import(52, "please".to_string(), "work".to_string(), 1); // unused in wat file!
    init_state.local_func_to_import();

    is_valid(
        &mut module,
        &mut init_state,
        &HashMap::new(),
        &HashMap::new(),
        "test_convert_local_fn_to_import",
    );
}

#[test]
fn test_set_fn_name_import_through_import() {
    let (buff, mut init_state) = setup();
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    state_assertions(&module, &init_state, false);

    let mut new_import_names = HashMap::new();
    module.imports.set_name("test".to_string(), 0);
    new_import_names.insert(0, "test".to_string());

    is_valid(
        &mut module,
        &mut init_state,
        &new_import_names,
        &HashMap::new(),
        "test_set_fn_name_import_through_import",
    );
}

#[test]
fn test_set_fn_name_import_through_module() {
    let (buff, mut init_state) = setup();
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    state_assertions(&module, &init_state, false);

    let mut new_import_names = HashMap::new();
    module.set_fn_name(0, "test".to_string());
    new_import_names.insert(0, "test".to_string());

    is_valid(
        &mut module,
        &mut init_state,
        &new_import_names,
        &HashMap::new(),
        "test_set_fn_name_import_through_module",
    );
}

#[test]
fn test_set_fn_name_local_through_functions() {
    let (buff, mut init_state) = setup();
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    state_assertions(&module, &init_state, false);

    let mut new_func_names = HashMap::new();
    module.functions.set_local_fn_name(10, "test".to_string());
    new_func_names.insert(10, "test".to_string());

    is_valid(
        &mut module,
        &mut init_state,
        &HashMap::new(),
        &new_func_names,
        "test_set_fn_name_local_through_functions",
    );
}

#[test]
fn test_set_fn_name_local_through_module() {
    let (buff, mut init_state) = setup();
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    state_assertions(&module, &init_state, false);

    let mut new_func_names = HashMap::new();
    module.set_fn_name(10, "test".to_string());
    new_func_names.insert(10, "test".to_string());

    is_valid(
        &mut module,
        &mut init_state,
        &HashMap::new(),
        &new_func_names,
        "test_set_fn_name_local_through_module",
    );
}

#[test]
fn test_set_fn_name_local_through_func_builder() {
    let (buff, mut init_state) = setup();
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    state_assertions(&module, &init_state, false);

    let mut new_func_names = HashMap::new();

    // add local function
    let name = "test0";
    let mut builder = FunctionBuilder::new(&*vec![], &*vec![]);
    builder.i32_const(1);
    builder.drop();
    builder.set_name(name.to_string());
    let fid = builder.finish_module(&mut module);

    assert_eq!(init_state.next_fid(), fid);
    init_state.add_local_func();
    new_func_names.insert(fid, name.to_string());

    // add local function
    let name = "other";
    let mut builder = FunctionBuilder::new(&*vec![], &*vec![]);
    builder.i32_const(1);
    builder.drop();
    builder.set_name("test1".to_string());
    let fid = builder.finish_module(&mut module);

    assert_eq!(init_state.next_fid(), fid);
    init_state.add_local_func();
    module.set_fn_name(fid, name.to_string());
    new_func_names.insert(fid, name.to_string());

    is_valid(
        &mut module,
        &mut init_state,
        &HashMap::new(),
        &new_func_names,
        "test_set_fn_name_local_through_func_builder",
    );
}

// GLOBALS

#[test]
fn test_create_and_add_global() {
    let (buff, mut init_state) = setup();
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    state_assertions(&module, &init_state, false);

    // add a local global
    let gid = module.add_global(
        InitExpr::Value(crate::ir::types::Value::I32(0)),
        DataType::I32,
        true,
        false,
    );
    assert_eq!(init_state.next_gid(), gid);
    init_state.add_local_global();

    // add a function using the new global
    let mut builder = FunctionBuilder::new(&*vec![], &*vec![]);
    builder.global_get(gid);
    builder.drop();
    let fid = builder.finish_module(&mut module);

    assert_eq!(init_state.next_fid(), fid);
    init_state.add_local_func();

    is_valid(
        &mut module,
        &mut init_state,
        &HashMap::new(),
        &HashMap::new(),
        "test_create_and_add_global",
    );
}

#[test]
fn test_add_imported_global() {
    let (buff, mut init_state) = setup();
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    state_assertions(&module, &init_state, false);

    // add an imported global
    let (gid, imp_id) = module.add_imported_global(
        "knock knock".to_string(),
        "gimme a global".to_string(),
        DataType::I32,
        true,
        false,
    );
    assert_eq!(init_state.next_gid(), gid);
    assert_eq!(init_state.next_imp_id(), imp_id);
    init_state.add_imported_global();

    // add a function using the new global
    let mut builder = FunctionBuilder::new(&*vec![], &*vec![]);
    builder.global_get(gid);
    builder.drop();
    let fid = builder.finish_module(&mut module);

    assert_eq!(init_state.next_fid(), fid);
    init_state.add_local_func();

    is_valid(
        &mut module,
        &mut init_state,
        &HashMap::new(),
        &HashMap::new(),
        "test_add_imported_global",
    );
}

#[test]
fn test_delete_global() {
    let (buff, mut init_state) = setup();
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    state_assertions(&module, &init_state, false);

    module.delete_global(2);
    init_state.delete_local_global();

    is_valid(
        &mut module,
        &mut init_state,
        &HashMap::new(),
        &HashMap::new(),
        "test_delete_global",
    );
}

#[test]
fn test_delete_imported_global() {
    let (buff, mut init_state) = setup();
    let mut module = Module::parse(&buff, false).expect("Unable to parse");
    state_assertions(&module, &init_state, false);

    module.delete_global(0);
    init_state.delete_imported_global();

    is_valid(
        &mut module,
        &mut init_state,
        &HashMap::new(),
        &HashMap::new(),
        "test_delete_imported_global",
    );
}

// ==========================
// ==== HELPER UTILITIES ====
// ==========================

struct State {
    // import state
    import_count: u32,
    import_func_count: u32,
    import_global_count: u32,
    import_table_count: u32,
    import_memory_count: u32,
    import_tag_count: u32,

    // import additions
    import_funcs_added: u32,
    import_tables_added: u32,
    import_globals_added: u32,
    import_memories_added: u32,
    import_tags_added: u32,

    // local state
    func_count: u32,
    global_count: u32,
    table_count: u32,
    memory_count: u32,
    #[allow(dead_code)]
    tag_count: u32,
}
impl State {
    fn clear_temporal(&mut self) {
        self.import_funcs_added = 0;
        self.import_tables_added = 0;
        self.import_globals_added = 0;
        self.import_memories_added = 0;
        self.import_tags_added = 0;
    }
    // IMPORTS
    fn next_imp_id(&self) -> u32 {
        self.import_count
    }
    fn add_import(&mut self) {
        self.import_count += 1;
    }
    fn delete_import(&mut self) {
        self.import_count -= 1;
    }
    // FUNCTIONS

    fn next_fid(&self) -> u32 {
        self.import_func_count + self.func_count
    }
    fn add_imported_func(&mut self) {
        self.add_import();
        self.import_funcs_added += 1;
        self.import_func_count += 1;
    }
    fn add_local_func(&mut self) {
        self.func_count += 1;
    }
    fn delete_local_func(&mut self) {
        self.func_count -= 1;
    }
    fn delete_imported_func(&mut self) {
        self.delete_import();
        self.import_func_count -= 1;
    }
    fn import_func_to_local(&mut self) {
        self.delete_imported_func();
        self.add_local_func();
    }
    fn local_func_to_import(&mut self) {
        self.delete_local_func();
        self.add_imported_func();
    }
    // GLOBALS

    fn next_gid(&self) -> u32 {
        self.import_global_count + self.global_count
    }
    fn add_imported_global(&mut self) {
        self.add_import();
        self.import_globals_added += 1;
        self.import_global_count += 1;
    }
    fn add_local_global(&mut self) {
        self.global_count += 1;
    }
    fn delete_local_global(&mut self) {
        self.global_count -= 1;
    }
    fn delete_imported_global(&mut self) {
        self.delete_import();
        self.import_global_count -= 1;
    }
}

fn setup() -> (Vec<u8>, State) {
    let file = "tests/test_inputs/spec-test/modules/if.wat";
    let buff = wat::parse_file(file).expect("couldn't convert the input wat to Wasm");
    let init_state = State {
        // import state
        import_count: 4,
        import_func_count: 1,
        import_global_count: 1,
        import_table_count: 1,
        import_memory_count: 1,
        import_tag_count: 0, // todo test with tags!

        // import additions
        import_funcs_added: 0,
        import_tables_added: 0,
        import_globals_added: 0,
        import_memories_added: 0,
        import_tags_added: 0,

        // local state
        func_count: 53,
        global_count: 2,
        table_count: 1,
        memory_count: 1,
        tag_count: 0, // todo test with tags!
    };

    (buff, init_state)
}

const TEST_DEBUG_DIR: &str = "output/tests/debug_me/ir.module.test/";

/// create output path if it doesn't exist
pub(crate) fn try_path(path: &str) {
    if !PathBuf::from(path).exists() {
        std::fs::create_dir_all(PathBuf::from(path).parent().unwrap()).unwrap();
    }
}

fn is_valid(
    module: &mut Module,
    state: &mut State,
    new_import_names: &HashMap<ImportsID, String>,
    new_fn_names: &HashMap<FunctionID, String>,
    test_name: &str,
) {
    state_assertions(&module, state, true);

    // encode and write to file
    let output_wasm_path = format!("{TEST_DEBUG_DIR}/{test_name}.wasm");
    encode_and_validate_wasm(module, &output_wasm_path);

    // reload from file
    let buff = std::fs::read(output_wasm_path).unwrap();
    let new_module = Module::parse(&buff, false).expect("Unable to parse");

    for (id, name) in new_import_names {
        assert_eq!(
            name,
            new_module.imports.get_import_name(*id).as_ref().unwrap()
        )
    }

    for (id, name) in new_fn_names {
        assert_eq!(name, new_module.functions.get_name(*id).as_ref().unwrap())
    }

    // make sure state assertions pass on the reloaded module!
    state.clear_temporal();
    state_assertions(&new_module, state, false)
}

fn state_assertions(module: &Module, state: &State, only_temporal: bool) {
    // import additions
    assert_eq!(state.import_funcs_added, module.imports.num_funcs_added);
    assert_eq!(state.import_globals_added, module.imports.num_globals_added);
    assert_eq!(state.import_tables_added, module.imports.num_tables_added);
    assert_eq!(
        state.import_memories_added,
        module.imports.num_memories_added
    );
    assert_eq!(state.import_tags_added, module.imports.num_tags_added);
    if only_temporal {
        return;
    }

    // import state
    assert_eq!(state.import_count, module.imports.len() as u32);
    assert_eq!(state.import_func_count, module.num_import_func());
    assert_eq!(state.import_func_count, module.imports.num_funcs);
    assert_eq!(state.import_global_count, module.imports.num_globals);
    assert_eq!(state.import_table_count, module.imports.num_tables);
    assert_eq!(state.import_memory_count, module.imports.num_memories);
    assert_eq!(state.import_tag_count, module.imports.num_tags);

    // local state
    assert_eq!(state.func_count, module.num_local_functions);
    assert_eq!(state.global_count, module.num_local_globals);
    assert_eq!(state.table_count, module.num_local_tables);
    assert_eq!(state.memory_count, module.num_local_memories);
}

pub(crate) fn encode_and_validate_wasm(module: &mut Module, output_wasm_path: &str) {
    try_path(output_wasm_path);
    if let Err(e) = module.emit_wasm(&output_wasm_path) {
        panic!(
            "Failed to dump wasm to {output_wasm_path} due to error: {}",
            e
        );
    }
    validate_wasm(&output_wasm_path);
}

pub(crate) fn validate_wasm(wasm_path: &str) -> bool {
    debug!("Running 'wasm-tools validate' on file: {wasm_path}");
    let res = Command::new("wasm-tools")
        .arg("validate")
        .arg(wasm_path)
        .output()
        .expect("failed to execute process");
    if !res.status.success() {
        println!("{:?}", std::str::from_utf8(&res.stderr).unwrap());
    }

    res.status.success()
}
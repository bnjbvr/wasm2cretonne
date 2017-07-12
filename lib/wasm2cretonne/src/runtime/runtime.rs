//! All the runtime support necessary for the wasm -> cretonne translation is formalized by the
//! trait `WasmRuntime`.
use cton_frontend::FunctionBuilder;
use cretonne::ir::{Value, Type};
use translation_utils::Local;


/// Struct that models Wasm globals
#[derive(Debug,Clone,Copy)]
pub struct Global {
    pub ty: Type,
    pub mutability: bool,
}

/// Struct that models Wasm tables
#[derive(Debug,Clone,Copy)]
pub struct Table {
    pub ty: TableElementType,
    pub size: u32,
    pub maximum: Option<u32>,
}

#[derive(Debug,Clone,Copy)]
pub enum TableElementType {
    Val(Type),
    Func(),
}

pub trait WasmRuntime {
    fn translate_get_global(&self,
                            builder: &mut FunctionBuilder<Local>,
                            global_index: u32)
                            -> Value;
    fn translate_set_global(&self,
                            builder: &mut FunctionBuilder<Local>,
                            global_index: u32,
                            val: Value);
    fn translate_grow_memory(&self, builder: &mut FunctionBuilder<Local>, val: Value);
    fn translate_current_memory(&self, builder: &mut FunctionBuilder<Local>) -> Value;

    fn declare_global(&mut self, global: Global);
    fn declare_table(&mut self, table: Table);
}

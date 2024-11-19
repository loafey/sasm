// ! WARNING, this module consists of macro abuse! :)

use std::{collections::HashMap, fmt::Debug, path::PathBuf};
pub mod clean_model;
mod memory;
use clean_model::Model;
use memory::Memory;
mod error;
use crate::parser::{BlockType, ExportDesc, BT};
pub use error::RuntimeError;

mod methods;
mod typecheck;

#[derive(Clone, Copy, PartialEq)]
#[allow(unused)]
pub enum Value {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Externref(u32),
    FuncRef(u32),
    BlockLock,
}
impl Value {
    pub fn as_str(&self) -> &'static str {
        match self {
            Value::I32(_) => "i32",
            Value::I64(_) => "i64",
            Value::F32(_) => "f32",
            Value::F64(_) => "f64",
            Value::Externref(_) => "externref",
            Value::FuncRef(_) => "funcref",
            Value::BlockLock => "BlockLock",
        }
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I32(arg0) => write!(f, "i32({arg0})"),
            Self::I64(arg0) => write!(f, "i64({arg0})"),
            Self::F32(arg0) => write!(f, "f32({arg0})"),
            Self::F64(arg0) => write!(f, "f64({arg0})"),
            Self::Externref(arg0) => write!(f, "externref({arg0})"),
            Self::FuncRef(arg0) => write!(f, "funcref({arg0})"),
            Self::BlockLock => write!(f, "--- BLOCK ---"),
        }
    }
}

pub struct DepthValue {
    bt: BT,
    pos: usize,
    vt: BlockType,
}

impl Debug for DepthValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} - {}", self.bt, self.pos)
    }
}

#[derive(Debug)]
pub struct Frame {
    pub func_id: u32,
    pub pc: usize,
    pub stack: Vec<Value>,
    pub locals: HashMap<u32, Value>,
    // pub labels: HashMap<u32, u32>,
    pub depth_stack: Vec<DepthValue>,
}

pub struct Runtime {
    pub _path: PathBuf,
    pub module: Model,
    pub stack: Vec<Frame>,
    pub globals: HashMap<u32, Value>,
    pub memory: Memory<{ 65536 + 1 }>,
    pub exports: HashMap<String, ExportDesc>,
    #[allow(unused)]
    pub datas: HashMap<u32, Vec<u8>>,
}

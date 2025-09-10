#![allow(unused)]

use inkwell::execution_engine::JitFunction;

#[cfg(feature = "llvm8-0")]
extern crate inkwell_80 as inkwell;
#[cfg(feature = "llvm8-0")]
extern crate llvm_sys_80 as llvm_sys;


#[cfg(feature = "llvm9-0")]
extern crate inkwell_90 as inkwell;
#[cfg(feature = "llvm9-0")]
extern crate llvm_sys_90 as llvm_sys;

#[cfg(feature = "llvm10-0")]
extern crate inkwell_100 as inkwell;
#[cfg(feature = "llvm10-0")]
extern crate llvm_sys_100 as llvm_sys;

#[cfg(feature = "llvm11-0")]
extern crate inkwell_110 as inkwell;
#[cfg(feature = "llvm11-0")]
extern crate llvm_sys_110 as llvm_sys;

#[cfg(feature = "llvm12-0")]
extern crate inkwell_120 as inkwell;
#[cfg(feature = "llvm12-0")]
extern crate llvm_sys_120 as llvm_sys;

#[cfg(feature = "llvm13-0")]
extern crate inkwell_130 as inkwell;
#[cfg(feature = "llvm13-0")]
extern crate llvm_sys_130 as llvm_sys;

#[cfg(feature = "llvm14-0")]
extern crate inkwell_140 as inkwell;
#[cfg(feature = "llvm14-0")]
extern crate llvm_sys_140 as llvm_sys;

#[cfg(feature = "llvm15-0")]
extern crate inkwell_150 as inkwell;
#[cfg(feature = "llvm15-0")]
extern crate llvm_sys_150 as llvm_sys;

#[cfg(feature = "llvm16-0")]
extern crate inkwell_160 as inkwell;
#[cfg(feature = "llvm16-0")]
extern crate llvm_sys_160 as llvm_sys;

#[cfg(feature = "llvm17-0")]
extern crate inkwell_170 as inkwell;
#[cfg(feature = "llvm17-0")]
extern crate llvm_sys_170 as llvm_sys;

#[cfg(feature = "llvm18-1")]
extern crate inkwell_181 as inkwell;
#[cfg(feature = "llvm18-1")]
extern crate llvm_sys_181 as llvm_sys;

#[cfg(feature = "llvm19-1")]
extern crate inkwell_191 as inkwell;
#[cfg(feature = "llvm19-1")]
extern crate llvm_sys_191 as llvm_sys;

#[cfg(feature = "llvm20-1")]
extern crate inkwell_201 as inkwell;
#[cfg(feature = "llvm20-1")]
extern crate llvm_sys_201 as llvm_sys;

pub mod sandbox;
pub mod orc;

struct Foo<const BAR: u32>;

impl Foo<69> {
    pub fn foo_69() {
        
    }
}

impl Foo<70> {
    pub fn foo_70() {
        
    }
}

fn main() {
    inkwell::targets::Target::initialize_native(&inkwell::targets::InitializationConfig::default())
        .expect("Failed to initialize native target");
    let triple = inkwell::targets::TargetTriple::create("x86_64-pc-windows-msvc");
    let target = inkwell::targets::Target::from_triple(&triple).expect("Failed to create target.");
    let target_machine = target.create_target_machine(
        &triple,
        "x86-64",
        "",
        inkwell::OptimizationLevel::Aggressive,
        inkwell::targets::RelocMode::Default,
        inkwell::targets::CodeModel::Default,
    ).expect("Failed to create target machine");
    
    let jit_stack = orc::jit_stack::JitStack::new(&target_machine).expect("Failed to create JIT Stack");
}

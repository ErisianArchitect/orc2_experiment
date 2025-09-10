use std::{ffi::CStr, marker::PhantomData, mem::MaybeUninit, rc::Rc};

use inkwell::module::Module;
use llvm_sys::{error::{LLVMDisposeErrorMessage, LLVMGetErrorMessage}, orc::{
    LLVMOrcAddEagerlyCompiledIR, LLVMOrcAddLazilyCompiledIR, LLVMOrcAddObjectFile, LLVMOrcCreateIndirectStub, LLVMOrcCreateInstance, LLVMOrcCreateLazyCompileCallback, LLVMOrcDisposeInstance, LLVMOrcDisposeMangledSymbol, LLVMOrcGetErrorMsg, LLVMOrcGetMangledSymbol, LLVMOrcGetSymbolAddress, LLVMOrcGetSymbolAddressIn, LLVMOrcJITStackRef, LLVMOrcLazyCompileCallbackFn, LLVMOrcModuleHandle, LLVMOrcOpaqueJITStack, LLVMOrcRegisterJITEventListener, LLVMOrcRemoveModule, LLVMOrcSetIndirectStubPointer, LLVMOrcSymbolResolverFn, LLVMOrcTargetAddress, LLVMOrcUnregisterJITEventListener
}};

use crate::orc::{orc_module::OrcModule, symbol_resolver::{SymbolResolver, _SymbolResolver, _symbol_resolver}};

#[derive(Clone)]
pub(crate) struct JitStackInner(Rc<LLVMOrcJITStackRef>);

impl Drop for JitStackInner {
    fn drop(&mut self) {
        if Rc::strong_count(&self.0) == 1 {
            unsafe {
                if self.0.is_null() {
                    return;
                }
                let err = unsafe { LLVMOrcDisposeInstance(*self.0) };
                if !err.is_null() {
                    let cstr = unsafe { LLVMGetErrorMessage(err) };
                    let scstr = unsafe { CStr::from_ptr(cstr) };
                    eprintln!("Error disposing LLVMOrcJITStack: {:?}", scstr);
                    unsafe { LLVMDisposeErrorMessage(cstr); }
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct JitStack {
    inner: JitStackInner,
}

impl JitStack {
    // TODO: Error
    // TODOC
    pub(crate) fn new(target_machine: &inkwell::targets::TargetMachine) -> Result<Self, ()> {
        let ptr = unsafe { LLVMOrcCreateInstance(target_machine.as_mut_ptr()) };
        if ptr.is_null() {
            return Err(());
        }
        Ok(Self {
            inner: JitStackInner(Rc::new(ptr)),
        })
    }
    
    // TODO: Error
    // TODOC
    /// Takes ownership of the module. It is important that you do not use the module
    /// after passing it as an argument to this function.
    pub fn add_eagerly_compiled_ir(&self, module: Module<'_>, symbol_resolver: Option<&dyn SymbolResolver>) -> Result<OrcModule, ()> {
        let mut _sym_resolver = MaybeUninit::uninit();
        // TODO: This isn't right. _SymbolResolver cannot live on the
        //       stack.
        let resolve_fn = if let Some(resolver) = symbol_resolver {
            _sym_resolver.write(_SymbolResolver::new(resolver));
            _sym_resolver.as_mut_ptr()
        } else {
            std::ptr::null_mut()
        };
        let mut handle_result = 0 as LLVMOrcModuleHandle;
        let err = unsafe { LLVMOrcAddEagerlyCompiledIR(
            *self.inner.0,
            &mut handle_result,
            module.as_mut_ptr(),
            if symbol_resolver.is_some() {
                Some(_symbol_resolver)
            } else {
                None
            },
            resolve_fn.cast(),
        ) };
        if !err.is_null() {
            let cstr = unsafe { LLVMGetErrorMessage(err) };
            let scstr = unsafe { CStr::from_ptr(cstr) };
            eprintln!("Error adding eagerly compiled IR: {:?}", scstr);
            unsafe { LLVMDisposeErrorMessage(cstr); }
            return Err(());
        }
        Ok(unsafe { OrcModule::new(handle_result) })
    }
}
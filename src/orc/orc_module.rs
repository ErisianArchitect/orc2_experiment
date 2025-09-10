use std::marker::PhantomData;

use llvm_sys::orc::LLVMOrcModuleHandle;

#[derive(Debug, Clone, Copy)]
pub struct OrcModule<'ctx> {
    handle: LLVMOrcModuleHandle,
    _lifetime: PhantomData<fn(&'ctx ())>,
}

impl<'ctx> OrcModule<'ctx> {
    #[inline]
    pub unsafe fn new(handle: LLVMOrcModuleHandle) -> Self {
        Self {
            handle,
            _lifetime: PhantomData,
        }
    }
}
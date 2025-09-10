use std::ffi::{c_char, c_void, CStr};


pub trait SymbolResolver {
    fn resolve_symbol(&self, name: &str) -> Option<u64>;
}

impl<F> SymbolResolver for F
where F: Fn(&str) -> Option<u64> {
    #[inline]
    fn resolve_symbol(&self, name: &str) -> Option<u64> {
        (self)(name)
    }
}

pub(crate) struct _SymbolResolver<'ctx> {
    resolver: &'ctx dyn SymbolResolver,
}

impl<'ctx> _SymbolResolver<'ctx> {
    #[inline]
    pub(crate) fn new(resolver: &'ctx dyn SymbolResolver) -> Self {
        Self { resolver }
    }
}

pub(crate) extern "C" fn _symbol_resolver(name: *const c_char, ctx: *mut c_void) -> u64 {
    unsafe {
        if ctx.is_null() {
            return 0;
        }
        let ctx = &*(ctx as *mut _SymbolResolver);
        // let resolver = ctx.cast::<_SymbolResolver>().as_ref();
        // let Some(resolver) = resolver else {
        //     return 0;
        // };
        
        let cstr = CStr::from_ptr(name);
        let Ok(name) = cstr.to_str() else {
            eprintln!("Could not create string slice from c string. (_symbol_resolver)");
            return 0;
        };
        if let Some(addr) = ctx.resolver.resolve_symbol(name) {
            addr
        } else {
            0
        }
    }
}
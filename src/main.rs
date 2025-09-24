#![allow(unused)]
#![allow(unsafe_op_in_unsafe_fn)]

use std::{collections::HashMap, fmt::Display, ptr::{null_mut, NonNull}, sync::atomic::{AtomicUsize, Ordering}, time::Duration};

use inkwell::{attributes::{Attribute, AttributeLoc}, context::Context, data_layout::DataLayout, execution_engine::JitFunction, fn_addr, orc::{function_address::FunctionAddress, orc_jit_fn::{OrcFunction, UnsafeOrcFn}, OrcEngine}, support::LLVMString, targets::{TargetMachine, TargetTriple}};
use llvm_sys::{orc::LLVMOrcTargetAddress, target_machine::LLVMGetDefaultTargetTriple};

use crate::orc::orc_jit_fn::OrcJitFunction;

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
pub mod lockfree_linked_list;

struct Foo<const BAR: u32>;

impl Foo<69> {
    pub fn foo_69() {
        
    }
}

impl Foo<70> {
    pub fn foo_70() {
        
    }
}

mod fnord {
    pub unsafe extern "C" fn foo() -> usize {
        0
    }
}

#[test]
fn niche_opt_test() {
    use std::mem::{size_of, align_of};
    use std::num::NonZero;
    #[repr(transparent)]
    struct FuncAddr(NonZero<LLVMOrcTargetAddress>);
    assert_eq!(size_of::<Option<FuncAddr>>(), size_of::<LLVMOrcTargetAddress>());
}

fn take_iter(items: impl Iterator<Item = (i64, i64)>) {
    for (a, b) in items {
        println!("({a:>3}, {b:>3})");
    }
}

struct Registrar {
    
}

impl Registrar {
    fn register(&mut self, name: impl Into<String>, value: i64) -> Option<i64> {
        let name: String = name.into();
        println!("Register: {name} = {value}");
        None
    }
}

trait GlobalSymbolRegistrar {
    fn register_all(self, registrar: &mut Registrar);
}

fn lockfree_linked_list_experiment() {
    use std::sync::atomic::AtomicPtr;
    #[inline]
    const fn max_ptr<T: Sized>() -> *const T {
        unsafe { std::mem::transmute(usize::MAX) }
    }
    #[inline]
    const fn max_ptr_mut<T: Sized>() -> *mut T {
        unsafe { std::mem::transmute(usize::MAX) }
    }
    trait PtrConsts {
        const NULL: *const Self;
        const NULL_MUT: *mut Self;
        const MAX_PTR: *const Self;
        const MAX_PTR_MUT: *mut Self;
    }
    impl<T: Sized> PtrConsts for T {
        const NULL: *const Self = std::ptr::null();
        const NULL_MUT: *mut Self = std::ptr::null_mut();
        const MAX_PTR: *const Self = max_ptr();
        const MAX_PTR_MUT: *mut Self = max_ptr_mut();
    }
    #[repr(C)]
    struct Node<T: Sized + 'static> {
        handle: AtomicPtr<AtomicPtr<Self>>,
        next: AtomicPtr<Self>,
        value: T
    }
    #[repr(transparent)]
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct NodePtr<T: Sized + 'static> {
        ptr: Option<NonNull<Node<T>>>,
    }
    impl<T: Sized + 'static> NodePtr<T> {
        fn value<'a>(&'a self) -> Option<&'a T> {
            unsafe { self.ptr.map(|node| &node.as_ref().value) }
        }
        fn value_mut<'a>(&'a mut self) -> Option<&'a mut T> {
            unsafe { self.ptr.map(|mut node| &mut node.as_mut().value) }
        }
        #[must_use]
        #[inline(always)]
        unsafe fn take(&mut self) -> Option<NonNull<Node<T>>> {
            self.ptr.take()
        }
        fn delete(&mut self) {
            unsafe {
                let Some(node) = self.take() else {
                    return;
                };
                let node_ref = unsafe { node.as_ref() };
                let mut handle = node_ref.handle.load(Ordering::Relaxed);
                loop {
                    debug_assert!(!handle.is_null(), "handle was null.");
                    // CAS handle to make sure we swap self.ptr with MAX_PTR_MUT
                    // this ensures that we're locking the correct handle.
                    // handle should never be null.
                    match (*handle).compare_exchange_weak(
                        node.as_ptr(),
                        <_>::MAX_PTR_MUT,
                        Ordering::AcqRel,
                        Ordering::Acquire,
                    ) {
                        Ok(_) => break,
                        Err(_) => {
                            std::hint::spin_loop();
                            handle = node_ref.handle.load(Ordering::Relaxed);
                            continue;
                        }
                    }
                }
                // now we know that handle is the handle for this node.
                // that means that we can then set the handle for the `next` node
                // to this node's handle. Since the handle is still MAX_PTR_MUT,
                // it will continue to be locked until the handle is set to the
                // next node's pointer.
                let mut next_node = node_ref.next.load(Ordering::Acquire);
                loop {
                    // no next node
                    if next_node.is_null() {
                        break;
                    }
                    match (*next_node).handle.compare_exchange_weak(
                        &node_ref.next as *const _ as *mut _,
                        handle,
                        Ordering::AcqRel,
                        Ordering::Acquire
                    ) {
                        Ok(_) => break,
                        Err(_) => {
                            next_node = node_ref.next.load(Ordering::Relaxed);
                            std::hint::spin_loop();
                            continue;
                        }
                    }
                }
                // Finally, set the handle to the next node.
                (*handle).store(next_node, Ordering::Relaxed);
                // Create a box and let it drop.
                _ = Box::from_raw(node.as_ptr());
            }
        }
    }
    // impl<T: Sized + 'static> Drop for NodePtr<T> {
    //     fn drop(&mut self) {
    //         self.delete();
    //     }
    // }
    struct List<T: Sized + 'static> {
        // Placed in a Box so that there is a stable pointer to the AtomicPtr<Node<T>>.
        // This makes it possible to set the handle pointer for the first node to the head of the list.
        head: Box<AtomicPtr<Node<T>>>,
    }
    impl<T: Sized + 'static> List<T> {
        fn new() -> Self {
            Self {
                head: Box::new(AtomicPtr::new(std::ptr::null_mut())),
            }
        }
        unsafe fn push_node(
            &self,
            node_handle: &AtomicPtr<AtomicPtr<Node<T>>>,
            node_next: &AtomicPtr<Node<T>>,
            node: *const Node<T>,
        ) {
            let mut head = self.head.load(Ordering::Relaxed);
            loop {
                while head == <_>::MAX_PTR_MUT {
                    std::hint::spin_loop();
                    head = self.head.load(Ordering::Relaxed);
                }
                // Set head to node
                match self.head.compare_exchange_weak(
                    head,
                    node as *mut _,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                ) {
                    Ok(_) => break,
                    Err(new_head) => head = new_head,
                }
            }
            node_handle.store(&*self.head as *const _ as *mut _, Ordering::Relaxed);
            node_next.store(head, Ordering::Relaxed);
            if let Some(head_ref) = head.as_ref() {
                head_ref.handle.store(node_next as *const _ as *mut _, Ordering::Relaxed);
            }
        }
        fn push(&self, value: T) -> NodePtr<T> {
            let node = Box::leak(Box::new(Node {
                handle: AtomicPtr::new(std::ptr::null_mut()),
                next: AtomicPtr::new(std::ptr::null_mut()),
                value,
            }));
            let handle = &node.handle;
            let next = &node.next;
            unsafe { self.push_node(handle, next, node); }
            NodePtr {
                ptr: Some(unsafe { NonNull::new_unchecked(node) }),
            }
        }
    }
    impl<T: Sized + 'static> Drop for List<T> {
        fn drop(&mut self) {
            let mut node = self.head.load(Ordering::Relaxed);
            while node == <_>::MAX_PTR_MUT {
                std::hint::spin_loop();
                node = self.head.load(Ordering::Relaxed);
            }
            if node.is_null() {
                return;
            }
            loop {
                match self.head.compare_exchange_weak(
                    node,
                    std::ptr::null_mut(),
                    Ordering::AcqRel,
                    Ordering::Acquire,
                ) {
                    Ok(_) => break,
                    Err(new_node) => {
                        node = new_node;
                        continue;
                    }
                }
            }
            loop {
                let node_box = unsafe { Box::from_raw(node) };
                node = node_box.next.load(Ordering::Relaxed);
                while node == <_>::MAX_PTR_MUT {
                    std::hint::spin_loop();
                    node = node_box.next.load(Ordering::Relaxed);
                }
                if node.is_null() {
                    break;
                }
                match node_box.next.compare_exchange_weak(
                    node,
                    std::ptr::null_mut(),
                    Ordering::AcqRel,
                    Ordering::Acquire,
                ) {
                    Ok(_) => continue,
                    Err(new_node) => node = new_node,
                }
            }
        }
    }
    #[repr(transparent)]
    struct OnDrop(u32);
    impl Drop for OnDrop {
        fn drop(&mut self) {
            println!("On Drop: {0}", self.0);
        }
    }
    let list = List::new();
    list.push(OnDrop(0));
    let mut nodes = vec![];
    nodes.push(list.push(OnDrop(1)));
    nodes.push(list.push(OnDrop(2)));
    nodes.push(list.push(OnDrop(3)));
    nodes.push(list.push(OnDrop(4)));
    nodes.push(list.push(OnDrop(5)));
    nodes.push(list.push(OnDrop(6)));
    nodes.push(list.push(OnDrop(7)));
    nodes.push(list.push(OnDrop(8)));
    nodes.push(list.push(OnDrop(9)));
    nodes.push(list.push(OnDrop(10)));
    for node in nodes.iter_mut().take(5) {
        node.delete();
    }
    list.push(OnDrop(11));
    drop(list);
    println!("Finished.");
}

fn main() {
    return lockfree_linked_list_experiment();
    inkwell::targets::Target::initialize_native(&inkwell::targets::InitializationConfig::default())
    .expect("Failed to initialize native target");
    // let triple = TargetMachine::get_default_triple();
    // let target = inkwell::targets::Target::from_triple(&triple).expect("Failed to create target.");
    // let target_machine = target.create_target_machine(
    //     &triple,
    //     TargetMachine::get_host_cpu_name().to_str().unwrap(),
    //     TargetMachine::get_host_cpu_features().to_str().unwrap(),
    //     inkwell::OptimizationLevel::None,
    //     inkwell::targets::RelocMode::Default,
    //     inkwell::targets::CodeModel::Default,
    // ).expect("Failed to create target machine");
    // let target_data = target_machine.get_target_data();
    
    let context = Context::create();
    let dll_export = context.create_string_attribute("dllexport", "");
    let module = context.create_module("main_module");
    let builder = context.create_builder();
    
    let void_t = context.void_type();
    let i32_t = context.i32_type();
    
    let sample_fn_t = i32_t.fn_type(&[
        i32_t.into(),
        i32_t.into(),
    ], false);
    let bar_fn_t = void_t.fn_type(&[i32_t.into()], false);

    let sample_fn = module.add_function("sample_fn\0", sample_fn_t, Some(inkwell::module::Linkage::External));
    sample_fn.add_attribute(AttributeLoc::Function, dll_export);
    let bar_fn = module.add_function("bar\0", bar_fn_t, None);
    
    let sample_fn_entry = context.append_basic_block(sample_fn, "entry\0");
    
    builder.position_at_end(sample_fn_entry);
    
    let arg0 = sample_fn.get_nth_param(0).expect("failed to get arg0").into_int_value();
    let arg1 = sample_fn.get_nth_param(1).expect("failed to get arg1").into_int_value();
    
    let add_args = builder.build_int_add(arg0, arg1, "add_args\0").unwrap();
    
    builder.build_call(bar_fn, &[add_args.into()], "_ignored").unwrap();
    
    builder.build_return(Some(&add_args)).unwrap();
    
    module.print_to_file("sample.llvm").unwrap();
    module.verify().unwrap();
    
    // let mem_buff = target_machine.write_to_memory_buffer(&module, inkwell::targets::FileType::Object).unwrap();
    
    let engine = OrcEngine::new_default().unwrap();
    extern "C" fn bar(num: i32) {
        println!("extern \"C\" fn bar({num})");
    }

    // unsafe { engine.register_function_raw("bar", bar as _).unwrap(); }
    // engine.register_function("bar", fn_addr!(bar : (i32)));
    let func_addr = engine.create_lazy_compile_callback(|engine| {
        println!("Lazy Compile Callback was called.");
        fn_addr!(bar : (i32))
    }).unwrap();
    
    engine.register_function("bar", func_addr).unwrap();
    
    engine.add_module(
        "main_module",
        module,
        inkwell::orc::CompilationMode::Lazy,
        None,
    ).unwrap();
    // drop(builder);
    // drop(context);
    
    // engine.add_object_from_memory("main_module", &mem_buff, None).unwrap();
    // let addr = FunctionAddress::new::<unsafe extern "C" fn()>(food::foo);
    println!("Here");
    let function = unsafe { engine.get_function::<unsafe extern "C" fn(i32, i32) -> i32>("sample_fn").unwrap() };
    let result = unsafe { function.call(2, 3) };
    assert_eq!(result, 5);
    println!("Result: {result}");
}

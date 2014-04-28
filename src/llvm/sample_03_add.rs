extern crate rustc = "rustc#0.11-pre";
extern crate libc = "libc#0.11-pre";

use rustc::lib::llvm::llvm;
use rustc::lib::llvm::{BuilderRef, ContextRef, ModuleRef};
use rustc::lib::llvm::{False, TypeRef, ValueRef};
use rustc::lib::llvm::BasicBlockRef;
use libc::{c_char, c_uint, c_ulonglong};

struct CoreUtils;
impl CoreUtils {
    // https://github.com/TheHydroImpulse/rust-llvm-example
    pub unsafe fn create_context() -> ContextRef {
        llvm::LLVMContextCreate()
    }

    pub unsafe fn create_module(name: ~str, context: ContextRef) -> ModuleRef {
        name.with_c_str(|name| {
            llvm::LLVMModuleCreateWithNameInContext(name, context)
        })
    }

    pub unsafe fn create_builder(context: ContextRef) -> BuilderRef {
        llvm::LLVMCreateBuilderInContext(context)
    }

    pub unsafe fn dump_module(module: ModuleRef) {
        llvm::LLVMDumpModule(module);
    }

    // https://github.com/mozilla/rust/blob/master/src/librustc/middle/trans/builder.rs
    pub fn noname() -> *c_char {
        static cnull: c_char = 0;
        &cnull as *c_char
    }

    pub unsafe fn const_int(type_ref: TypeRef, value: int) -> ValueRef {
        llvm::LLVMConstInt(type_ref, value as c_ulonglong, False)
    }
}

struct ModuleUtils;
impl ModuleUtils {
    pub unsafe fn add_function(module: ModuleRef, name: ~str, func_type: TypeRef) -> ValueRef {
        name.with_c_str(|name| {
            llvm::LLVMAddFunction(module, name, func_type)
        })
    }

    pub unsafe fn append_basic_block(context: ContextRef, func_value: ValueRef,
                                     name: ~str) -> BasicBlockRef {
        name.with_c_str(|name| {
            llvm::LLVMAppendBasicBlockInContext(context, func_value, name)
        })
    }
}

struct TypeUtils;
impl TypeUtils {
    pub unsafe fn int32(context: ContextRef) -> TypeRef {
        llvm::LLVMInt32TypeInContext(context)
    }

    pub unsafe fn func(args: &[TypeRef], ret: &TypeRef) -> TypeRef {
        llvm::LLVMFunctionType(*ret, args.as_ptr(), args.len() as c_uint, False)
    }
}

fn main() {
    unsafe {
        let context = CoreUtils::create_context();
        let module = CoreUtils::create_module(~"top", context);
        let _builder = CoreUtils::create_builder(context);

        let int32_type = TypeUtils::int32(context);
        let func_type = TypeUtils::func(&[], &int32_type);
        let main_func = ModuleUtils::add_function(module, ~"main", func_type);
        let entry_block = ModuleUtils::append_basic_block(context, main_func, ~"entry");

        //let value_ref1 = CoreUtils::const_int(int32_type, 42);
        //let value_ref2 = CoreUtils::const_int(int32_type, 4);

        println!("{:?}", main_func);
        println!("{:?}", entry_block);
        CoreUtils::dump_module(module);
    }
}

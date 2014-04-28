extern crate rustc = "rustc#0.11-pre";
extern crate libc = "libc#0.11-pre";

use rustc::lib::llvm::llvm;
use rustc::lib::llvm::False;
use libc::c_char;

// https://github.com/mozilla/rust/blob/master/src/librustc/middle/trans/builder.rs
pub fn noname() -> *c_char {
    static cnull: c_char = 0;
    &cnull as *c_char
}

fn main() {
    // http://www.ibm.com/developerworks/linux/library/os-createcompilerllvm1/index.html

    // http://llvm.org/docs/doxygen/html/modules.html
    // http://llvm.org/docs/LangRef.html#store-instruction
    // http://llvm.org/docs/tutorial/LangImpl7.html

    // https://github.com/fsprojects/llvm-fs/blob/master/test/add.fs
    // https://github.com/fsprojects/llvm-fs/blob/master/test/CSSimpleTest2.cs
    // https://github.com/wickedchicken/llvm-c-example/blob/master/fac.c
    // https://github.com/hobinjk/rusty-kaleidoscope/blob/master/lang.rs

    // https://github.com/mozilla/rust/blob/master/src/librustc/lib/llvm.rs

    unsafe {
        let context = llvm::LLVMContextCreate();
        let module = "top".with_c_str(|name| {
            llvm::LLVMModuleCreateWithNameInContext(name, context)
        });
        let builder = llvm::LLVMCreateBuilderInContext(context);

        // %ptr = alloca i32
        // store i32 3, i32* %ptr
        // %val = load i32* %ptr

        let type_i32 = llvm::LLVMInt32TypeInContext(context);
        let ptr = llvm::LLVMBuildAlloca(builder, type_i32, noname());

        let value = llvm::LLVMConstInt(type_i32, 3, False);
        llvm::LLVMBuildStore(builder, value, ptr);
        llvm::LLVMBuildLoad(builder, ptr, noname());

        llvm::LLVMDumpModule(module);
    }
}

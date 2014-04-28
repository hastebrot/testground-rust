extern crate rustc = "rustc#0.11-pre";

use rustc::lib::llvm::llvm;

fn main() {
    unsafe {
        let context = llvm::LLVMContextCreate();
        let module = "top".with_c_str(|name| {
            llvm::LLVMModuleCreateWithNameInContext(name, context)
        });
        let builder = llvm::LLVMCreateBuilderInContext(context);

        llvm::LLVMDumpModule(module);
        // ; ModuleID = 'top'
    }
}

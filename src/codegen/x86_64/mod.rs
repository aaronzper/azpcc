use std::{fs::File, path::Path};
use faerie::{ArtifactBuilder, Decl, Link};
use iced_x86::{code_asm::*, Instruction};
use target_lexicon::Triple;

pub fn generate(triple: Triple, output: &Path) {
    let mut asm = CodeAssembler::new(64).unwrap();

    let mut label_main = asm.create_label();
    let mut label_add = asm.create_label();
    let mut print_invoked = asm.create_label();

    asm.set_label(&mut label_main).unwrap();
    asm.mov(rdi, 69u64).unwrap();
    asm.call(label_add).unwrap();
    asm.ret().unwrap();

    asm.set_label(&mut label_add).unwrap();
    asm.add(rdi, 10).unwrap();
    asm.push(rdi).unwrap();

    asm.add_instruction(Instruction::with_declare_byte_1(0xE8)).unwrap();
    asm.set_label(&mut print_invoked).unwrap();
    asm.add_instruction(Instruction::with_declare_dword_1(0)).unwrap();

    asm.pop(rax).unwrap();
    asm.ret().unwrap();

    let result = asm.assemble_options(0x0,4).unwrap();

    let main_ip = result.label_ip(&label_main).unwrap() as usize;
    let add_ip = result.label_ip(&label_add).unwrap() as usize;
    let print_ref_ip = result.label_ip(&print_invoked).unwrap();
    let print_offset = print_ref_ip - (add_ip as u64);
    println!("main: {}, add: {}", main_ip, add_ip);

    let bytes = result.inner.code_buffer;
    let main = &bytes[main_ip..add_ip];
    let add = &bytes[add_ip..];

    let f = File::create(output).unwrap();
    let mut obj = ArtifactBuilder::new(triple)
        .name(output.to_str().unwrap().to_owned())
        .finish();

    obj.declare_with("main_unused", Decl::function().global(), main.to_owned())
        .unwrap();
    obj.declare_with("add", Decl::function().global(), add.to_owned())
        .unwrap();
    obj.declare("print_num", Decl::function_import()).unwrap();

    println!("{}", print_offset);
    obj.link(Link { from: "add", to: "print_num", at: print_offset }).unwrap();

    obj.write(f).unwrap();
}

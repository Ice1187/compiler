use std::io::{stdin, stdout, Write};
use std::fs::File;
use std::env;
use struct_lib::Node;
use parser_lib;
use asm_lib as asm;


fn get_filename() -> (String, String) {
	let mut filename = String::new();
	let mut sfile = String::new();
	let mut args = env::args_os();
	println!("args len: {:?}", args.len());
	if args.len() > 1 { 
		filename = args.nth(1).expect("Invalid arg[1]").into_string().expect("Invalid arg[1]").clone();
		println!("arg[1]: {}\nUse arg[1] as filename.", filename)
	} else {	
		print!("File name: ");
		stdout().flush().unwrap();
		stdin().read_line(&mut filename).unwrap();
	}
	// print!(".s file name: ");
	// stdout().flush().unwrap();
	// stdin().read_line(&mut sfile).unwrap();

	filename = filename.trim().to_string();
	sfile = filename.replace(".c", ".s");
	(filename, sfile)
}


// $ gcc -m32 -masm=intel main.s -o main

fn main() {
	// let mut filename = String::new();
	// let mut sfile = String::new();
	let (filename, sfile) = get_filename();
	println!("----------------");

	let mut f = File::create(&sfile).expect("Unable to create the file.");
	let ast: Vec<Node::Node> = parser_lib::parse(&filename, &sfile);
	// parser_lib::print_ast(&ast, 1);

	// println!("\n[+] Start generating assembly.\n");
	analyze_ast(& ast, 0, 0, &mut f);
	// println!("\n[+] Finish generating assembly.\n");
	println!("----------------");
	println!("Generated file: {}", sfile);
}

fn analyze_ast(ast: & Vec<Node::Node>, me: usize, from: usize, mut f: &mut File) {
	match ast[me]._level.as_str() {
		"Program"   => program(&ast, me, from, &mut f),
		"Function"  => function(&ast, me, from, &mut f),
		"Statement" => statement(&ast, me, from, &mut f),
		_ => panic!("Oh No!"),
	}
}

fn program(ast: & Vec<Node::Node>, me: usize, from: usize, mut f: &mut File) {
	// intel syntax
	println!("  .intel_syntax noprefix");
	print!("  ");
	print!(".file       ");
	println!("\"{}\"", ast[me]._name);

	write!(f, "  .intel_syntax noprefix\n").expect("generator program: Unable to write to the file.");
	write!(f, "  ").expect("generator program: Unable to write to the file.");
	write!(f, ".file       ").expect("generator program: Unable to write to the file.");
	write!(f, "\"{}\"\n", ast[me]._name).expect("generator program: Unable to write to the file.");


	for i in ast[me].to.iter() {
		let to = *i;
		analyze_ast(& ast, to, me, &mut f);
	}
}

fn function(ast: & Vec<Node::Node>, me: usize, from: usize, mut f: &mut File) {
	print!("  ");
	print!(".globl      ");
	println!("{}", ast[me]._name);
	println!("{}:", ast[me]._name);

	write!(f, "  ").expect("generator func: Unable to write to the file.");
	write!(f, ".globl      ").expect("generator func: Unable to write to the file.");
	write!(f, "{}\n", ast[me]._name).expect("generator func: Unable to write to the file.");
	write!(f, "{}:\n", ast[me]._name).expect("generator func: Unable to write to the file.");

	for i in ast[me].to.iter() {
		let to = *i;
		analyze_ast(& ast, to, me, &mut f);
	}
}

fn statement(ast: & Vec<Node::Node>, me: usize, from: usize, mut f: &mut File) {
	match ast[me]._type.as_str() {
		"RETURN_KEYWORD" => stat_return(&ast, me, from, &mut f),
		_ => (),
	}
}

fn stat_return(ast: & Vec<Node::Node>, me: usize, from: usize, mut f: &mut File) {
	let ret_val = exp(&ast, ast[me].to[0], me);
	asm::mov("eax", ret_val, &mut f);
	print!("  ");
	println!("ret");

	write!(f, "  ").expect("stat_return: Unable to write to the file.");
	write!(f, "ret\n").expect("stat_return: Unable to write to the file.");
}

fn exp(ast: & Vec<Node::Node>, me: usize, from: usize) -> &str {
	ast[me]._value.as_str()
}


use std::io::{stdin, stdout, Write};
use struct_lib::Node;
use parser_lib;
use asm_lib as asm;

// $ gcc -m32 -masm=intel main.s -o main

fn main() {
	let mut filename = String::new();
	let mut prefix = String::new();
	print!("File name: ");
	stdout().flush().unwrap();
	stdin().read_line(&mut filename).unwrap();
	filename = filename.trim().to_string();

	print!("File prefix: ");
	stdout().flush().unwrap();
	stdin().read_line(&mut prefix).unwrap();
	prefix = prefix.trim().to_string();
	println!("----------------\n");

	let ast: Vec<Node::Node> = parser_lib::parse(&filename, &prefix);
	parser_lib::print_ast(&ast, 1);

	println!("\n[+] Start generating assembly.\n");
	analyze_ast(& ast, 0, 0);
	println!("\n[+] Finish generating assembly.\n");

}

fn analyze_ast(ast: & Vec<Node::Node>, me: usize, from: usize) {
	match ast[me]._level.as_str() {
		"Program"   => program(&ast, me, from),
		"Function"  => function(&ast, me, from),
		"Statement" => statement(&ast, me, from),
		_ => panic!("Oh No!"),
	}
}

fn program(ast: & Vec<Node::Node>, me: usize, from: usize) {
	// intel syntax
	println!("  .intel_syntax noprefix");

	print!("  ");
	print!(".file       ");
	println!("\"{}\"", ast[me]._name);
	for i in ast[me].to.iter() {
		let to = *i;
		analyze_ast(& ast, to, me);
	}
}

fn function(ast: & Vec<Node::Node>, me: usize, from: usize) {
	print!("  ");
	print!(".globl      ");
	println!("{}", ast[me]._name);
	println!("{}:", ast[me]._name);
	for i in ast[me].to.iter() {
		let to = *i;
		analyze_ast(& ast, to, me);
	}
}

fn statement(ast: & Vec<Node::Node>, me: usize, from: usize) {
	match ast[me]._type.as_str() {
		"RETURN_KEYWORD" => stat_return(&ast, me, from),
		_ => (),
	}
}

fn stat_return(ast: & Vec<Node::Node>, me: usize, from: usize) {
	let ret_val = exp(&ast, ast[me].to[0], me);
	asm::mov("eax", ret_val);
	print!("  ");
	println!("ret");
}

fn exp(ast: & Vec<Node::Node>, me: usize, from: usize) -> &str {
	ast[me]._value.as_str()
}


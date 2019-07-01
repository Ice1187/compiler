use std::io::{Write};
use std::fs::File;
use lib::{Node};
use parser_lib as parser;
use asm_lib as asm;

// $ gcc -m32 -masm=intel main.s -o main

pub fn gen(path: & String, filename: & String) {
	// println!("----------------");

	let ast: Vec<Node::Node> = parser::parse(&path, &filename);
	let mut f = File::create(&filename).expect("Gen: Unable to create the file.");
	// Pprint::print_ast(&ast);

	// println!("\n[+] Start generating assembly.\n");
	analyze_ast(& ast, 0, 0, &mut f);
	// println!("\n[+] Finish generating assembly.\n");
	// Pprint::print_asm(&filename);
	// println!("----------------");
	// println!("Generated file: {}", filename);
}

fn analyze_ast(ast: & Vec<Node::Node>, me: usize, from: usize, mut f: &mut File) {
	match ast[me]._level.as_str() {
		"Program"   => program(&ast, me, from, &mut f),
		"Function"  => function(&ast, me, from, &mut f),
		"Statement" => statement(&ast, me, from, &mut f),
		_ => panic!("Gen: Unrecoginized AST node."),
	}
}

fn program(ast: & Vec<Node::Node>, me: usize, from: usize, mut f: &mut File) {
	// intel syntax
	// println!("  .intel_syntax noprefix");
	// print!("  ");
	// print!(".file       ");
	// println!("\"{}\"", ast[me]._name);

	write!(f, "  .intel_syntax noprefix\n").expect("Gen: generator program: Unable to write to the file.");
	write!(f, "  ").expect("Gen: generator program: Unable to write to the file.");
	write!(f, ".file       ").expect("Gen: generator program: Unable to write to the file.");
	write!(f, "\"{}\"\n", ast[me]._name).expect("Gen: generator program: Unable to write to the file.");


	for i in ast[me].to.iter() {
		let to = *i;
		analyze_ast(& ast, to, me, &mut f);
	}
}

fn function(ast: & Vec<Node::Node>, me: usize, from: usize, mut f: &mut File) {
	// print!("  ");
	// print!(".globl      ");
	// println!("{}", ast[me]._name);
	// println!("{}:", ast[me]._name);

	write!(f, "  ").expect("Gen: generator func: Unable to write to the file.");
	write!(f, ".globl      ").expect("Gen: generator func: Unable to write to the file.");
	write!(f, "{}\n", ast[me]._name).expect("Gen: generator func: Unable to write to the file.");
	write!(f, "{}:\n", ast[me]._name).expect("Gen: generator func: Unable to write to the file.");

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
	// print!("  ");
	// println!("ret");

	write!(f, "  ").expect("Gen: stat_return: Unable to write to the file.");
	write!(f, "ret\n").expect("Gen: stat_return: Unable to write to the file.");
}

fn exp(ast: & Vec<Node::Node>, me: usize, from: usize) -> &str {
	ast[me]._value.as_str()
}


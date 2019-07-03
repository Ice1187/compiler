use std::io::{Write};
use std::fs::File;
use lib::{Node};
use parser_lib as parser;
use asm_lib as asm;

// $ gcc -m32 -masm=intel main.s -o main

pub fn gen(ast: & Vec<Node::Node>, filename: & String) {
	// println!("");
 //    println!("----------------");
 //    println!("[+] Code Generator:");

	let mut f = File::create(&filename).expect("Gen: Unable to create the file.");

	analyze_ast(& ast, 0, 0, &mut f);

}

fn analyze_ast(ast: & Vec<Node::Node>, me: usize, from: usize, mut f: &mut File) {
	match ast[me]._level.as_str() {
		"Program"   => program(&ast, me, from, &mut f),
		"Function"  => function(&ast, me, from, &mut f),
		"Statement" => statement(&ast, me, from, &mut f),
		_ => panic!("Gen: Unrecoginized AST node.\nAST node: {} {} {} {}", 
			ast[me]._level, ast[me]._type, ast[me]._name, ast[me]._value),
	}
}

fn program(ast: & Vec<Node::Node>, me: usize, from: usize, mut f: &mut File) {
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
		"RETURN_KEYWORD" => Return(&ast, me, from, &mut f),
		_ => (),
	}
}

fn Return(ast: & Vec<Node::Node>, me: usize, from: usize, mut f: &mut File) {
	expression(&ast, ast[me].to[0], &mut f);
	asm::ret(&mut f);

}

fn expression(ast: & Vec<Node::Node>, me: usize, mut f: &mut File){
	match ast[me]._type.as_str() {
		"CONSTANT"   => Constant(&ast, me, &mut f),
		"NEGATION"   => Neg(&ast, me, &mut f),
		"BIT_COMPLE" => BitComple(&ast, me, &mut f),
		"LOGIC_NEG"   => LogicNeg(&ast, me, &mut f),
		_ => panic!("Gen: Unrecoginized expression type.\nExp type: {} {} {}",
					ast[me]._level, ast[me]._type, ast[me]._value),
	}

	
}

fn Constant(ast: & Vec<Node::Node>, me: usize, mut f: &mut File) {
	let ret = ast[me]._value.as_str();
	asm::mov("eax", ret, &mut f);
}

fn Neg(ast: & Vec<Node::Node>, me: usize, mut f: &mut File) {
	expression(&ast, ast[me].to[0], &mut f);
	asm::neg("eax", &mut f);
}

fn BitComple(ast: & Vec<Node::Node>, me: usize, mut f: &mut File) {
	expression(&ast, ast[me].to[0], &mut f);
	asm::not("eax", &mut f);
}


// cmp eax(exp), 0  // return exp==0 ? 
// sete eax 		 //     1 : 0

fn LogicNeg(ast: & Vec<Node::Node>, me: usize, mut f: &mut File) {
	expression(&ast, ast[me].to[0], &mut f);
	asm::cmp("eax", "0", &mut f);
	asm::mov("eax", "0", &mut f);
	asm::sete("al", &mut f);
}

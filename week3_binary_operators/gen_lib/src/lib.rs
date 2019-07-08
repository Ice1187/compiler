use std::io::Write;
use std::fs::File;
use lib::Node;
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
	exp(&ast, ast[me].to[0], &mut f);
	asm::ret(&mut f);

}

fn exp(ast: & Vec<Node::Node>, me: usize, mut f: &mut File){
	match ast[me]._type.as_str() {
		"CONSTANT"  => Constant(&ast, me, &mut f),
		"UNARY_OP"  => UnOp(&ast, me, &mut f),
		"BINARY_OP" => BinOp(&ast, me, &mut f),
		_ => panic!("Gen: Unrecoginized exp type.\nExp type: {} {} {}",
					ast[me]._level, ast[me]._type, ast[me]._value),
	}
}

fn Constant(ast: & Vec<Node::Node>, me: usize, mut f: &mut File) {
	let ret = ast[me]._value.as_str();
	asm::mov("eax", ret, &mut f);
}

fn UnOp(ast: & Vec<Node::Node>, me: usize, mut f: &mut File) {
	exp(&ast, ast[me].to[0], &mut f);
	match ast[me]._value.as_str() {
		"-" => asm::neg("eax", &mut f),
		"~" => asm::not("eax", &mut f),
		"!" => { asm::cmp("eax", "0", &mut f);
				 asm::mov("eax", "0", &mut f);
				 asm::sete("al", &mut f);    }
		_ => panic!("Gen UnOp: Unrecoginized unary operator.\nUnOp type: {} {} {}",
					ast[me]._level, ast[me]._type, ast[me]._value),
	}
}

fn BinOp(ast: & Vec<Node::Node>, me: usize, mut f: &mut File) {
	exp(&ast, ast[me].to[0], &mut f);
	asm::push("eax", &mut f);
	exp(&ast, ast[me].to[1], &mut f);
	asm::push("eax", &mut f);
	asm::pop("ecx", &mut f);
	asm::pop("eax", &mut f);
	match ast[me]._value.as_str() {
		"+" => asm::add("eax", "ecx", &mut f),
		"-" => asm::sub("eax", "ecx", &mut f),
		"*" => asm::imul("eax", "ecx", &mut f),
		"/" => { asm::xor("edx", "edx", &mut f);
				 asm::idiv("ecx", &mut f); }
		_ => panic!("Gen UnOp: Unrecoginized unary operator.\nUnOp type: {} {} {}",
					ast[me]._level, ast[me]._type, ast[me]._value),
	}
}


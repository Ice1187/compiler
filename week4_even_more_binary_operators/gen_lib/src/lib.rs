use std::io::Write;
use std::fs::File;
use lib::Node;
use asm_lib as asm;
use simple_counter as count;

count::generate_counter!(or_c, i64);
count::generate_counter!(and_c, i64);
count::generate_counter!(end_c, i64);

// $ gcc -m32 -masm=intel main.s -o main
pub fn gen(ast: & Vec<Node::Node>, filename: & String) {
	// println!("");
 //    println!("----------------");
 //    println!("[+] Code Generator:");

	let mut f = File::create(&filename).expect("Gen: Unable to create the .s file.");

	or_c::reset();
	and_c::reset();
	end_c::reset();
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
	let mut or = "_OR".to_string();
	let mut and = "_AND".to_string();
	let mut end = "_END".to_string();
	match ast[me]._value.as_str() {
		"+"  => { BinInit(&ast, me, &mut f);
				  asm::add("eax", "ecx", &mut f) }
		"-"  => { BinInit(&ast, me, &mut f);
				  asm::sub("eax", "ecx", &mut f) }
		"*"  => { BinInit(&ast, me, &mut f);
				  asm::imul("eax", "ecx", &mut f) }
		"/"  => { BinInit(&ast, me, &mut f);
				  asm::xor("edx", "edx", &mut f);
				  asm::idiv("ecx", &mut f); 	}
		"==" => { BinInit(&ast, me, &mut f);
				  asm::cmp("eax", "ecx", &mut f);
				  asm::mov("eax", "0", &mut f);
				  asm::sete("al", &mut f);		}
		"!=" => { BinInit(&ast, me, &mut f);
				  asm::cmp("eax", "ecx", &mut f);
				  asm::mov("eax", "0", &mut f);
				  asm::setne("al", &mut f);		}
		">" =>  { BinInit(&ast, me, &mut f);
				  asm::cmp("eax", "ecx", &mut f);
				  asm::mov("eax", "0", &mut f);
				  asm::setg("al", &mut f);		}
		"<" =>  { BinInit(&ast, me, &mut f);
				  asm::cmp("eax", "ecx", &mut f);
				  asm::mov("eax", "0", &mut f);
				  asm::setl("al", &mut f);		}
		">=" => { BinInit(&ast, me, &mut f);
				  asm::cmp("eax", "ecx", &mut f);
				  asm::mov("eax", "0", &mut f);
				  asm::setge("al", &mut f);		}
		"<=" => { BinInit(&ast, me, &mut f);
				  asm::cmp("eax", "ecx", &mut f);
				  asm::mov("eax", "0", &mut f);
				  asm::setle("al", &mut f);		}
		"||" => { or.push_str(or_c::next().to_string().as_str());
				  end.push_str(end_c::next().to_string().as_str());
				  exp(&ast, ast[me].to[0], &mut f);
				  asm::cmp("eax", "0", &mut f);
				  asm::je(&or, &mut f);
				  asm::mov("eax", "1", &mut f);
				  asm::jmp(&end, &mut f);
				  asm::make_func(&or, &mut f);
				  exp(&ast, ast[me].to[1], &mut f);
				  asm::cmp("eax", "0", &mut f);
				  asm::mov("eax", "0", &mut f);
				  asm::setne("al", &mut f);
				  asm::make_func(&end, &mut f);
		}
		"&&" => { and.push_str(and_c::next().to_string().as_str());
				  end.push_str(end_c::next().to_string().as_str());
				  exp(&ast, ast[me].to[0], &mut f);
				  asm::cmp("eax", "0", &mut f);
				  asm::jne(&and, &mut f);
				  asm::mov("eax", "0", &mut f);
				  asm::jmp(&end, &mut f);
				  asm::make_func(&and, &mut f);
				  exp(&ast, ast[me].to[1], &mut f);
				  asm::cmp("eax", "0", &mut f);
				  asm::mov("eax", "0", &mut f);
				  asm::setne("al", &mut f);
				  asm::make_func(&end, &mut f);
		}
		_ => panic!("Gen BinOp: Unrecoginized binary operator.\nBinOp type: {} {} {}",
					ast[me]._level, ast[me]._type, ast[me]._value),
	}
}


fn BinInit(ast: & Vec<Node::Node>, me: usize, mut f: &mut File) {
	exp(&ast, ast[me].to[0], &mut f);
	asm::push("eax", &mut f);
	exp(&ast, ast[me].to[1], &mut f);
	asm::push("eax", &mut f);
	asm::pop("ecx", &mut f);
	asm::pop("eax", &mut f);
}
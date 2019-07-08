use lexer_lib as lexer;
use parser_lib as parser;
use gen_lib as gen;
use lib::Pprint;
use std::env;
use std::io::{stdin, stdout, Write};
use std::process::Command;
use std::os::unix::process::CommandExt;


fn get_args() -> (String, String) {
	let mut path = String::new();
	let mut args = env::args_os();
	if args.len() > 1 { 
		path = args.nth(1).expect("Compiler: Invalid arg[1]").into_string().expect("Compiler: Invalid arg[1]").clone();
	} else {	
		print!("File name: ");
		stdout().flush().unwrap();
		stdin().read_line(&mut path).expect("Compiler: Invalid file name.");
	}

	let filename = c2s(path.clone());
	(path, filename)

}

fn c2s(path: String) -> String {
	// path = .c ; filename = .s
	let filename = path.trim().to_string().replace(".c", ".s");
	filename
}


fn gcc(filename: & String) {
	let out = filename.replace(".s", "");
	Command::new("gcc")
    .uid(0)
    .arg("-masm=intel")
    .arg(&filename)
    .arg("-o")
    .arg(&out)
    .status().unwrap();
}


fn main() {
	// let valid = vec!["../test/add.c", "../test/associativity_2.c", 
	// "../test/associativity.c", "../test/div.c", "../test/mult.c", 
	// "../test/parens.c", "../test/precedence.c", "../test/sub.c", 
	// "../test/sub_neg.c", "../test/unop_add.c", "../test/unop_parens.c"];

	let (path, filename) = get_args();
	
	// let mut filename: String;
	// for path in valid {
	// filename = c2s(path.to_string());
	// Pprint::print_file(&path.to_string().clone());  	 

	let token_vec = lexer::lex(&path);	
	// Pprint::print_token(&token_vec);

	let ast = parser::parse(token_vec, &path);
	// Pprint::print_ast(&ast, 0);

	gen::gen(&ast, &filename.to_string());
	// Pprint::print_asm(&filename.to_string());

	gcc(&filename);
	// }
}
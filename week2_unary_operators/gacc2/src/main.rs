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

	// path = .c ; filename = .s
	path = path.trim().to_string();
	let filename = path.replace(".c", ".s");
	(path, filename)

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
	// let valid = vec!["../../test/stage_2/valid/bitwise.c", "../../test/stage_2/valid/bitwise_zero.c", 
	// 				"../../test/stage_2/valid/neg.c", "../../test/stage_2/valid/nested_ops_2.c", 
	// 				"../../test/stage_2/valid/nested_ops.c", "../../test/stage_2/valid/not_five.c", 
	// 				"../../test/stage_2/valid/not_zero.c"];

	let (path, filename) = get_args();

	let token_vec = lexer::lex(&path);	
	// Pprint::print_token(&token_vec);

	let ast = parser::parse(token_vec, &path);
	// Pprint::print_ast(&ast, 2);

	gen::gen(&ast, &filename);
	// Pprint::print_asm(&filename);

	gcc(&filename);
}
use lexer_lib as lexer;
use parser_lib as parser;
use gen_lib as gen;
use lib::{Token, Node, Pprint};


fn get_filename() -> (String, String) {
	let path = env args;
	let filename = path_resolver(&path);

	(path, filename)
}


fn main() {
	let valid = vec!["../../test/stage_2/valid/bitwise.c", "../../test/stage_2/valid/bitwise_zero.c", 
					"../../test/stage_2/valid/neg.c", "../../test/stage_2/valid/nested_ops_2.c", 
					"../../test/stage_2/valid/nested_ops.c", "../../test/stage_2/valid/not_five.c", 
					"../../test/stage_2/valid/not_zero.c"];
	for file in valid {

		let token_vec = lexer::lex(&path.as_str());	
		let ast = parser::parse(&token_vec, &filename);







	}
}
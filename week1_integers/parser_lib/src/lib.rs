use lib::{Token, Node};
use lexer_lib;


pub fn parse(path: & String, filename: & String) -> Vec<Node::Node> {
	let mut token_vec = lexer_lib::lex(&path);

	// println!("Parser:");
	// Pprint::print_token(&token_vec);
	// println!("----------------");

	// first node of AST
	let mut ast: Vec<Node::Node> = Vec::new();
	// println!("\n[+] Start parsing.\n");
	program(&mut token_vec,&mut ast, path.clone());
	// println!("\n[+] Finish parsing.");

	ast
}


fn program(mut token_vec: &mut Vec<Token::Token>, mut ast: &mut Vec<Node::Node>, filename: String) {
	ast.push(Node::new());
	ast[0]._level = String::from("Program"); 
	ast[0]._type  = String::from("FILE");
	ast[0]._name  = String::from(filename); 

	// print to test
	// println!("{}: {}", &ast[0]._level, &ast[0]._name);
	
	while token_vec.len() != 0 {
		function(&mut token_vec, &mut ast, 0);
	}
}


fn function(mut token_vec: &mut Vec<Token::Token>, mut ast: &mut Vec<Node::Node>, root: usize) {
	if ast.len() == 0 { panic!("Parser: Unable to set AST at program level."); }
	loop {
		let ast_len = ast.len();
		
		// push ast node
		ast.push(Node::new());
		
		// add previous node's "to"
		ast[root].to.push(ast_len);

		// set Function node's level
		ast[ast_len]._level = String::from("Function");
		
		// set Function node's type
		if token_vec[0]._type == "INT_KEYWORD" { 
			ast[ast_len]._type = String::from(token_vec[0]._type.clone());
			token_vec.remove(0);
		} else { panic!("Parser: Function type was invalid."); }

		// set Function node's name, value
		if token_vec[0]._type == "IDENTIFIER" {
			ast[ast_len]._name = String::from(token_vec[0]._value.clone());
			token_vec.remove(0);
		} else { panic!("Parser: Function name was unvalid."); }
		
		// set Function node's (
		if token_vec[0]._type == "OPEN_PAREN" { 
			token_vec.remove(0);
		} else { panic!("Parser: Function ( not found."); }

		// set Function node's )
		if token_vec[0]._type == "CLOSE_PAREN" { 
			token_vec.remove(0);
		} else { panic!("Parser: Function ) not found."); }

		// set Function node's {
		if token_vec[0]._type == "OPEN_BRACE" { 
			token_vec.remove(0);
		} else { panic!("Parser: Function { not found."); }

		// print to test
		// println!("  {}: {} {}", ast[ast_len]._level, ast[ast_len]._type, ast[ast_len]._name);

		statement(&mut token_vec, &mut ast, ast_len);

		// set Function node's }
		if token_vec[0]._type == "CLOSE_BRACE" { 
			token_vec.remove(0);
			break;
		} else { panic!("Parser: Function }} not found.\n Function }}: {} {}", token_vec[0]._type, token_vec[0]._value); }
	}
}


fn statement(mut token_vec: &mut Vec<Token::Token>, mut ast: &mut Vec<Node::Node>, root: usize) {
	// set Statement node's type
	while token_vec[0]._type != "CLOSE_BRACE" {
		let ast_len = ast.len();
		// push ast node
		ast.push(Node::new());
		// set previous node's "to"
		ast[root].to.push(ast_len);
		// set Statement node's level
		ast[ast_len]._level = String::from("Statement");

		if token_vec[0]._type == "RETURN_KEYWORD" {
			ast[ast_len]._type = String::from(token_vec[0]._type.clone());
			ast[ast_len]._name = String::from(token_vec[0]._value.clone());
			token_vec.remove(0);

			// println!("    {}: {} {}", ast[ast_len]._level, ast[ast_len]._type, ast[ast_len]._name);

			exp(&mut token_vec, &mut ast, ast_len)

		} else { panic!("Parser: Statement type was wrong. \n Statement type: {} {}", token_vec[0]._type, token_vec[0]._value); }
		if token_vec[0]._type == "SEMICOLON" {
			token_vec.remove(0);
		} else { panic!("Parser: Statement end was wrong. \n Statement end: {} {}", token_vec[0]._type, token_vec[0]._value);		}
	}
}

//
fn exp(token_vec: &mut Vec<Token::Token>, ast: &mut Vec<Node::Node>, root: usize) {
	let ast_len = ast.len();

	// push ast node
	ast.push(Node::new());

	// add previous node's "to"
	ast[root].to.push(ast_len);

	// set Expression node's level
	ast[ast_len]._level = String::from("Expression");

	if token_vec[0]._type == "CONSTANT" {
		ast[ast_len]._type = String::from(token_vec[0]._type.clone());
		ast[ast_len]._value = String::from(token_vec[0]._value.clone());
		token_vec.remove(0);
	} else { panic!("Parser: Expression type was wrong. \n Expression type: {} {}", token_vec[0]._type, token_vec[0]._value); }

	// print to test
	// println!("      {}: {} {}", ast[ast_len]._level, ast[ast_len]._type, ast[ast_len]._value);

}

// <program> ::= <function>
// <function> ::= <type> <identifier> "(" ")" "{" <statement> "}"
// <type> ::= "int"
// <identifier> ::= [a-zA-z_]{[a-zA-z_0-9]}
// <statement> ::= "return" <exp> ";"
// <exp> ::= [0-9]


// fn set_token(token_vec: &mut Vec<Token::Token>) {
// 	token_vec.push(Token::Token {
// 		_type:  "INT_KEYWORD".to_string(),
// 		_value: "int".to_string(),
// 	} );
// 	token_vec.push(Token::Token {
// 		_type:  "IDENTIFIER".to_string(),
// 		_value: "main".to_string(),
// 	} );
// 	token_vec.push(Token::Token {
// 		_type:  "OPEN_PAREN".to_string(),
// 		_value: "(".to_string(),
// 	} );
// 	token_vec.push(Token::Token {
// 		_type:  "CLOSE_PAREN".to_string(),
// 		_value: ")".to_string(),
// 	} );
// 	token_vec.push(Token::Token {
// 		_type:  "OPEN_BRACE".to_string(),
// 		_value: "{".to_string(),
// 	} );
// 	token_vec.push(Token::Token {
// 		_type:  "RETURN_KEYWORD".to_string(),
// 		_value: "return".to_string(),
// 	} );
// 	token_vec.push(Token::Token {
// 		_type:  "CONSTANT".to_string(),
// 		_value: "2".to_string(),
// 	} );
// 	token_vec.push(Token::Token {
// 		_type:  "SEMICOLON".to_string(),
// 		_value: ";".to_string(),
// 	} );
// 	token_vec.push(Token::Token {
// 		_type:  "RETURN_KEYWORD".to_string(),
// 		_value: "return".to_string(),
// 	} );
// 	token_vec.push(Token::Token {
// 		_type:  "CONSTANT".to_string(),
// 		_value: "3".to_string(),
// 	} );
// 	token_vec.push(Token::Token {
// 		_type:  "SEMICOLON".to_string(),
// 		_value: ";".to_string(),
// 	} );
// 	token_vec.push(Token::Token {
// 		_type:  "CLOSE_BRACE".to_string(),
// 		_value: "}".to_string(),
// 	} );
// 	// func2
// 	token_vec.push(Token::Token {
// 		_type:  "INT_KEYWORD".to_string(),
// 		_value: "int".to_string(),
// 	} );
// 	token_vec.push(Token::Token {
// 		_type:  "IDENTIFIER".to_string(),
// 		_value: "main2".to_string(),
// 	} );
// 	token_vec.push(Token::Token {
// 		_type:  "OPEN_PAREN".to_string(),
// 		_value: "(".to_string(),
// 	} );
// 	token_vec.push(Token::Token {
// 		_type:  "CLOSE_PAREN".to_string(),
// 		_value: ")".to_string(),
// 	} );
// 	token_vec.push(Token::Token {
// 		_type:  "OPEN_BRACE".to_string(),
// 		_value: "{".to_string(),
// 	} );
// 	token_vec.push(Token::Token {
// 		_type:  "RETURN_KEYWORD".to_string(),
// 		_value: "return".to_string(),
// 	} );
// 	token_vec.push(Token::Token {
// 		_type:  "CONSTANT".to_string(),
// 		_value: "4".to_string(),
// 	} );
// 	token_vec.push(Token::Token {
// 		_type:  "SEMICOLON".to_string(),
// 		_value: ";".to_string(),
// 	} );
// 	token_vec.push(Token::Token {
// 		_type:  "RETURN_KEYWORD".to_string(),
// 		_value: "return".to_string(),
// 	} );
// 	token_vec.push(Token::Token {
// 		_type:  "CONSTANT".to_string(),
// 		_value: "5".to_string(),
// 	} );
// 	token_vec.push(Token::Token {
// 		_type:  "SEMICOLON".to_string(),
// 		_value: ";".to_string(),
// 	} );
// 	token_vec.push(Token::Token {
// 		_type:  "CLOSE_BRACE".to_string(),
// 		_value: "}".to_string(),
// 	} );
// }
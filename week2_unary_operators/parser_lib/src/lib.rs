use lib::{Token, Node, Pprint};


pub fn parse(token_vec: Vec<Token::Token>, filename: & String) -> Vec<Node::Node> {
    println!("");
    println!("----------------");
    println!("[+] Parser:");
	
    // Get the token vector
	let mut token_vec = token_vec;
	Pprint::print_token(&token_vec);

	// Build AST
	let mut ast: Vec<Node::Node> = Vec::new();
	// println!("\n[+] Start parsing.\n");
	program(&mut token_vec,&mut ast, filename);
	// println!("\n[+] Finish parsing.");

	ast
}


fn program(mut token_vec: &mut Vec<Token::Token>, mut ast: &mut Vec<Node::Node>, filename: & String) {
	// first node of AST
	ast.push(Node::new());
	ast[0]._level = String::from("Program"); 
	ast[0]._type  = String::from("FILE");
	ast[0]._name  = String::from(filename.clone()); 
	if ast.len() == 0 { panic!("Parser: Unable to create AST."); }

	while token_vec.len() != 0 {
		function(&mut token_vec, &mut ast, 0);
	}
}


fn function(mut token_vec: &mut Vec<Token::Token>, mut ast: &mut Vec<Node::Node>, root: usize) {
	loop {
		let id = ast.len();
		
		// push ast node
		ast.push(Node::new());
		
		// add previous node's "to"
		ast[root].to.push(id);

		// set Function node's level
		ast[id]._level = String::from("Function");
		
		// set Function node's type
		if token_vec[0]._type == "INT_KEYWORD" { 
			ast[id]._type = String::from(token_vec[0]._type.clone());
			token_vec.remove(0);
		} else { panic!("Parser: Function type was invalid.\n Function type: {} {}", token_vec[0]._type, token_vec[0]._value); }

		// set Function node's name, value
		if token_vec[0]._type == "IDENTIFIER" {
			ast[id]._name = String::from(token_vec[0]._value.clone());
			token_vec.remove(0);
		} else { panic!("Parser: Function name was unvalid.\n Function name: {} {}", token_vec[0]._type, token_vec[0]._value); }
		
		// set Function node's (
		if token_vec[0]._type == "OPEN_PAREN" { 
			token_vec.remove(0);
		} else { panic!("Parser: Function ( not found.\n Function (: {} {}", token_vec[0]._type, token_vec[0]._value); }

		// set Function node's )
		if token_vec[0]._type == "CLOSE_PAREN" { 
			token_vec.remove(0);
		} else { panic!("Parser: Function ) not found.\n Function ): {} {}", token_vec[0]._type, token_vec[0]._value); }

		// set Function node's {
		if token_vec[0]._type == "OPEN_BRACE" { 
			token_vec.remove(0);
		} else { panic!("Parser: Function {{ not found.\n Function {{: {} {}", token_vec[0]._type, token_vec[0]._value); }

		statement(&mut token_vec, &mut ast, id);

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
		let id = ast.len();
		// push ast node
		ast.push(Node::new());
		// set previous node's "to"
		ast[root].to.push(id);
		// set Statement node's level
		ast[id]._level = String::from("Statement");

		match token_vec[0]._type.as_str() {
			"RETURN_KEYWORD" => Return(&mut token_vec, &mut ast, id),
			_ => panic!("Parser: Statement type was wrong. \n Statement type: {} {}", token_vec[0]._type, token_vec[0]._value),
		}

		if token_vec[0]._type == "SEMICOLON" {
			token_vec.remove(0);
		} else { panic!("Parser: Statement end was wrong. \n Statement end: {} {}", token_vec[0]._type, token_vec[0]._value);		}
	}
}


fn Return(mut token_vec: &mut Vec<Token::Token>, mut ast: &mut Vec<Node::Node>, root: usize) {
	let id = ast.len()-1;
	ast[id]._type = String::from(token_vec[0]._type.clone());
	ast[id]._name = String::from(token_vec[0]._value.clone());
	token_vec.remove(0);
	exp(&mut token_vec, &mut ast, root)
}

//
fn exp(token_vec: &mut Vec<Token::Token>, ast: &mut Vec<Node::Node>, root: usize) {
	let id = ast.len();

	// push ast node
	ast.push(Node::new());

	// add previous node's "to"
	ast[root].to.push(id);

	// set Expression node's level
	ast[id]._level = String::from("Expression");

	match token_vec[0]._type.as_str() {
		"CONSTANT" => Constant(&mut token_vec, &mut ast, id),
		"NEGATION" | "BIT_COMPLE" | "LOGIC_NEG" => exp_unary_op(&mut token_vec, &mut ast, id),
		_ => panic!("Parser: Expression type was wrong. \n Expression type: {} {}", token_vec[0]._type, token_vec[0]._value),
	}
}


fn exp_unary_op(token_vec: &mut Vec<Token::Token>, ast: &mut Vec<Node::Node>, root: usize) {
	let _type = token_vec[0]._type.clone();
	token_vec.remove(0);

	[How to do this recursively]
	match _type.as_str() {  
		"NEGATION"   => Neg(&mut token_vec, &mut ast, root),
		"BIT_COMPLE" => BitComple(&mut token_vec, &mut ast, root),
		"LOGIC_NEG"  => LogicNeg(&mut token_vec, &mut ast, root),
	}
}


fn Constant(token_vec: &mut Vec<Token::Token>, ast: &mut Vec<Node::Node>, root: usize) {
	let id = ast.len()-1;
	ast[id]._type = String::from(token_vec[0]._type.clone());
	ast[id]._value = String::from(token_vec[0]._value.clone());
	token_vec.remove(0);
}


fn Neg(token_vec: &mut Vec<Token::Token>, ast: &mut Vec<Node::Node>, root: usize) {
	let id = ast.len()-1;

	// remove "-" token
	token_vec.remove(0);

	// ensure next token is a constant
	if token_vec[0]._type != "CONSTANT" { 
		panic!("Parser exp_neg: token not constant.\nToken: {} {}", token_vec[0]._type, token_vec[0]._value);
	}	

	let val = -1 * token_vec[0]._value.parse()
		.expect("Parser exp_neg: Unable to parse the constant");
	ast[id]._type = String::from(token_vec[0]._type.clone());
	ast[id]._value = String::from(val);
	token_vec.remove(0);

}


fn BitComple(token_vec: &mut Vec<Token::Token>, ast: &mut Vec<Node::Node>, root: usize) {

}


fn LogicNeg(token_vec: &mut Vec<Token::Token>, ast: &mut Vec<Node::Node>, root: usize) {

}
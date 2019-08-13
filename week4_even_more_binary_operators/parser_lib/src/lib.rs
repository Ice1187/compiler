use lib::{Token, Node};

// <program> ::= <function>
// <function> ::= <statement>
// <statement> ::= <exp>
// <exp> ::= <term> {("+" | "-") <term>}
// <term> ::= <factor> {("*" | "/") <factor>}
// <factor> ::= <unary_op> <factor> | <constant> | "(" <exp> ")"


pub fn parse(token_vec: Vec<Token::Token>, filename: &str) -> Vec<Node::Node> {
    // println!("");
    // println!("----------------");
    // println!("[+] Parser:");
	
    // Get the token vector
	let mut token_vec = token_vec;

	// Build AST
	let mut ast: Vec<Node::Node> = Vec::new();
	// println!("\n[+] Start parsing.\n");
	program(&mut token_vec, &mut ast, filename);
	// println!("\n[+] Finish parsing.");

	ast
}


fn program(mut token_vec: &mut Vec<Token::Token>, mut ast: &mut Vec<Node::Node>, filename: &str) {
	// first node of AST
	ast.push(Node::new());
	ast[0]._level = String::from("Program"); 
	ast[0]._type  = String::from("FILE");
	ast[0]._name  = String::from(filename); 
	if ast.len() == 0 { panic!("Parser: Unable to create AST."); }

	while token_vec.len() != 0 {
		function(&mut token_vec, &mut ast, 0);
	}
}


fn function(mut token_vec: &mut Vec<Token::Token>, mut ast: &mut Vec<Node::Node>, root: usize) {
	loop {
		let id = ast.len();		
		ast.push(Node::new());
		ast[root].to.push(id);
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

		while token_vec[0]._type != "CLOSE_BRACE" {
			statement(&mut token_vec, &mut ast, id);
		}

		// set Function node's }
		if token_vec[0]._type == "CLOSE_BRACE" { 
			token_vec.remove(0);
			break;
		} else { panic!("Parser: Function }} not found.\n Function }}: {} {}", token_vec[0]._type, token_vec[0]._value); }
	}
}


fn statement(mut token_vec: &mut Vec<Token::Token>, mut ast: &mut Vec<Node::Node>, root: usize) {
	// set Statement node's type
	let id = ast.len();
	// push ast node
	ast.push(Node::new());
	// set previous node's "to"
	ast[root].to.push(id);
	// set Statement node's level
	ast[id]._level = String::from("Statement");

	match token_vec[0]._type.as_str() {
		"RETURN_KEYWORD" => Return(&mut token_vec, &mut ast),
		"INT_KEYWORD"    => Int(&mut token_vec, &mut ast),
		_ => panic!("Parser: Statement type was wrong. \n Statement type: {} {}", token_vec[0]._type, token_vec[0]._value),
	}

	if token_vec[0]._type == "SEMICOLON" {
		token_vec.remove(0);
	} else { panic!("Parser: Statement end was wrong. \n Statement end: {} {}", token_vec[0]._type, token_vec[0]._value);		}
}


fn Return(mut token_vec: &mut Vec<Token::Token>, mut ast: &mut Vec<Node::Node>) {
	// modify statement node to return 
	let id = ast.len()-1;
	ast[id]._type = token_vec[0]._type.clone();
	ast[id]._name = token_vec[0]._value.clone();
	token_vec.remove(0);
	// push child
	let child: usize;
	child = exp(&mut token_vec, &mut ast);
	ast[id].to.push(child); 
}

// declar / assign
fn Int(mut token_vec: &mut Vec<Token::Token>, mut ast: &mut Vec<Node::Node>) {
	let id = ast.len()-1;
	ast[id]._type = "Declare".to_string();
	ast[id]._name = token_vec[0]._value.clone();
	token_vec.remove(0);

	let child = identifier(&mut token_vec, &mut ast);
	ast[id].to.push(child);
	if token_vec[0]._type == "ASSIGN" {
		ast[id]._type = String::from("Assign");
		token_vec.remove(0);
		let right_child = exp(&mut token_vec, &mut ast);
		ast[id].to.push(right_child)

	}
}


fn identifier(mut token_vec: &mut Vec<Token::Token>, mut ast: &mut Vec<Node::Node>) -> usize {
	let id = ast.len();
	ast.push(Node::new());
	ast[id]._level = String::from("Expression");
	ast[id]._type = token_vec[0]._type.clone();
	ast[id]._value = token_vec[0]._value.clone();
	token_vec.remove(0);
	id
}


fn exp(mut token_vec: &mut Vec<Token::Token>, mut ast: &mut Vec<Node::Node>) -> usize {
	let mut ret: usize;
	ret = logic_and_exp(&mut token_vec, &mut ast);

	while token_vec[0]._type == "LOGIC_OR" {
		// Make BinOp node
		let op = Token::Token { 
					_type:  ("BINARY_OP").to_string(), 
					_value: token_vec[0]._value.clone() 
				 };
		token_vec.remove(0);


		let right_child = logic_and_exp(&mut token_vec, &mut ast);
		ret = BinOp(&mut ast, op, ret, right_child);
	}

	ret
}


fn logic_and_exp(mut token_vec: &mut Vec<Token::Token>, mut ast: &mut Vec<Node::Node>) -> usize {
	let mut ret: usize;
	ret = equality_exp(&mut token_vec, &mut ast);

	while token_vec[0]._type == "LOGIC_AND" {
		// Make BinOp node
		let op = Token::Token { 
					_type:  ("BINARY_OP").to_string(), 
					_value: token_vec[0]._value.clone() 
				 };
		token_vec.remove(0);


		let right_child = equality_exp(&mut token_vec, &mut ast);
		ret = BinOp(&mut ast, op, ret, right_child);
	}

	ret
}


fn equality_exp(mut token_vec: &mut Vec<Token::Token>, mut ast: &mut Vec<Node::Node>) -> usize {
	let mut ret: usize;
	ret = relational_exp(&mut token_vec, &mut ast);

	while token_vec[0]._type == "EQUAL" 
	|| token_vec[0]._type == "NOT_EQUAL" {
		// Make BinOp node
		let op = Token::Token { 
					_type:  ("BINARY_OP").to_string(), 
					_value: token_vec[0]._value.clone() 
				 };
		token_vec.remove(0);


		let right_child = relational_exp(&mut token_vec, &mut ast);
		ret = BinOp(&mut ast, op, ret, right_child);
	}

	ret
}


fn relational_exp(mut token_vec: &mut Vec<Token::Token>, mut ast: &mut Vec<Node::Node>) -> usize {
	let mut ret: usize;
	ret = additive_exp(&mut token_vec, &mut ast);

	while token_vec[0]._type == "LESS" 
	|| token_vec[0]._type == "LESS_EQUAL"
	|| token_vec[0]._type == "GREATER" 
	|| token_vec[0]._type == "GREATER_EQUAL"{
		// Make BinOp node
		let op = Token::Token { 
					_type:  ("BINARY_OP").to_string(), 
					_value: token_vec[0]._value.clone() 
				 };
		token_vec.remove(0);


		let right_child = additive_exp(&mut token_vec, &mut ast);
		ret = BinOp(&mut ast, op, ret, right_child);
	}

	ret
}


fn additive_exp(mut token_vec: &mut Vec<Token::Token>, mut ast: &mut Vec<Node::Node>) -> usize {
	let mut ret: usize;
	ret = term(&mut token_vec, &mut ast);

	while token_vec[0]._type == "ADDITION"
	|| token_vec[0]._type == "MINUS" {
		// Make BinOp node
		let op = Token::Token { 
					_type:  ("BINARY_OP").to_string(), 
					_value: token_vec[0]._value.clone() 
				 };
		token_vec.remove(0);


		let right_child = term(&mut token_vec, &mut ast);
		ret = BinOp(&mut ast, op, ret, right_child);
	}

	ret
}


fn term(mut token_vec: &mut Vec<Token::Token>, mut ast: &mut Vec<Node::Node>) -> usize {
	let mut ret: usize;
	ret = factor(&mut token_vec, &mut ast);

	while token_vec[0]._type == "MULTIPLICATION"
	|| token_vec[0]._type == "DIVISION" {
		// Make BinOP node
		let op = Token::Token { 
					_type:  ("BINARY_OP").to_string(), 
					_value: token_vec[0]._value.clone() 
				 };
		token_vec.remove(0);

		let right_child = factor(&mut token_vec, &mut ast);
		ret = BinOp(&mut ast, op, ret, right_child);
	}

	ret
}


fn factor(mut token_vec: &mut Vec<Token::Token>, mut ast: &mut Vec<Node::Node>) -> usize {
	let ret: usize;
	
	// parse parentheses
	if token_vec[0]._type=="OPEN_PAREN" {
		token_vec.remove(0);

		ret = exp(&mut token_vec, &mut ast);
		
		if token_vec[0]._type != "CLOSE_PAREN" { 
			panic!("Parser factor: \")\" invalid\nFactor type: {} {}", token_vec[0]._type, token_vec[0]._value); 
		}
		
		token_vec.remove(0);
	} 

	// pasre constant, UnOp
	else {	
		match token_vec[0]._type.as_str() {
			"CONSTANT" => ret = Constant(&mut token_vec, &mut ast),
			"MINUS" | "BIT_NOT" | "LOGIC_NEG" => ret = UnOp(&mut token_vec, &mut ast),
			_ => panic!("Parser factor: factor type invalid\n Factor type: {} {}", token_vec[0]._type, token_vec[0]._value),
		}
	}

	ret
}


fn BinOp(ast: &mut Vec<Node::Node>, op: Token::Token, left_child: usize, right_child: usize) -> usize {
	let id = ast.len();
	ast.push(Node::new());
	ast[id]._level = "Expression".to_string();
	ast[id]._type  = op._type;
	ast[id]._value = op._value;
	ast[id].to.push(left_child);
	ast[id].to.push(right_child);
	id
}


fn Constant(token_vec: &mut Vec<Token::Token>, ast: &mut Vec<Node::Node>) -> usize {
	let id = ast.len();
	ast.push(Node::new());
	ast[id]._level = "Expression".to_string();
	ast[id]._type = token_vec[0]._type.clone();
	ast[id]._value = token_vec[0]._value.clone();
	token_vec.remove(0);

	id
}


fn UnOp(mut token_vec: &mut Vec<Token::Token>, mut ast: &mut Vec<Node::Node>) -> usize {
	let id = ast.len();
	ast.push(Node::new());
	ast[id]._level = "Expression".to_string();

	// modify minus to negation
	// if token_vec[0]._type == "MINUS" {
		// ast[id]._type = "NEGATION".to_string();
	// } 
	// else {
		// ast[id]._type = token_vec[0]._type.clone();
	// }

	ast[id]._type = "UNARY_OP".to_string();
	ast[id]._value = token_vec[0]._value.clone();
	token_vec.remove(0);

	let child: usize;
	child = factor(&mut token_vec, &mut ast);
	ast[id].to.push(child);

	id
}
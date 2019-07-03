use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use lib::Token;

// Int match part
//     <dec int> ::= <number>{<number>}
//     <number> ::= 0|1|2|3|4|5|6|7|8|9
//     Regex for decimal integers
fn int_match(text: &mut String) -> Token::Token {
    let int_re = Regex::new(r"(^[0-9][0-9]*)\b").unwrap();
    let int_cap = int_re.captures(&text).expect("Lexer int_match: Not a int")
                        .get(1).expect("Lexer int_match: int not match");
    // let int;
    let mut token = Token::new();
    let int_end = int_cap.end();
    let int_str =  int_cap.as_str().to_string();
    token._type = "CONSTANT".to_string();
    token._value = int_str.clone();
    text.replace_range(..int_end, "");
    token
}


// Word match part
    // <word> :: = <alpha_underscore>{<alpha>|"_"|<number>}
    // <alpha_underscore> ::= <alpha>|"_"
    // <alpha> ::= a-zA-Z{a-zA-Z}
    // Regex for keyword, variable name
fn word_match(text: &mut String) -> Token::Token {
	let word_re = Regex::new(r"([a-zA-Z_][a-zA-Z_0-9]*)\b").unwrap();
    let word_cap = word_re.captures(&text).expect("Lexer word_match: Not a word")
                           .get(1).expect("Lexer word_match: word not match.");
    let mut token = Token::new();
    let word_end = word_cap.end();
    let word_str = word_cap.as_str().to_string();
    token._value = word_str.clone();
    token._type = keyword_check(word_str.as_str()).to_string();
	text.replace_range(..word_end, "");
    token
}


// Symbol match part
    // <symbol> ::= "("|")"|"{"|"}"|";"
fn symbol_match(text: &mut String) -> Token::Token {
    let syb_re = Regex::new(r"(^[\(\)\{\};\-~!+*/])").unwrap();
    let syb_cap = syb_re.captures(&text).expect("Lexer symbol_match: Not a symbol")
                           .get(1).expect("Lexer symbol_match: symbol not match.");
    let mut token = Token::new();
	let syb_end = syb_cap.end();
    let syb_str = syb_cap.as_str().to_string();
    token._value = syb_str.clone();
    token._type = keysyb_check(syb_str.as_str()).to_string();
	text.replace_range(..syb_end, "");
    token
}


fn keyword_check(word: &str) -> &str {
	match word {
		"int" => "INT_KEYWORD",
        "char" => "CHAR_KEYWORD",
		"return" => "RETURN_KEYWORD",
		_ => "IDENTIFIER",
	}
}


fn keysyb_check(syb: &str) -> &str {
    match syb {
        "(" => "OPEN_PAREN",
        ")" => "CLOSE_PAREN",
        "{" => "OPEN_BRACE",
        "}" => "CLOSE_BRACE",
        ";" => "SEMICOLON",
        "-" => "MINUS",
        "~" => "BIT_COMPLE",
        "!" => "LOGIC_NEG",
        "+" => "ADDITION",
        "*" => "MULTIPLICATION",
        "/" => "DIVISION",
        _ => panic!("Lexer: Unrecognize symbol"),
    }
}


pub fn lex(path: &str) -> Vec<Token::Token> {
    // println!("");
    // println!("----------------");
    // println!("[+] Lexer:");
	
    // Open the source file
    // println!("[-] Source file: {}", &path);
    let file = File::open(&path).expect("\nLexer: Failed to open the source file\n");
	let reader = BufReader::new(file);
	
    // Create token vector
    let mut token_vec: Vec<Token::Token> = Vec::new();

    // Line number
    // let mut line_count = 1;

    // Lex the file line by line
    for line in reader.lines(){

        // Read line and trim
		let mut text = line.expect("Lexer: Unable to read the line of the source file.")
                           .trim().to_string();
    	// println!("\nLine {}:", line_count);

        // Check first character to determine the match function
	    let mut first: char;
        let mut token: Token::Token;
	    while text.len()!=0 {
            // print to test
            // println!("\"{}\"", &text);

	        first = text.chars().nth(0)
                    .expect("Lexer: Unable to read the first character of the line");
            if first.is_ascii_digit() {
	        	token = int_match(&mut text);
                token_vec.push(token);
	        } else if first.is_ascii_alphabetic() {
	        	token = word_match(&mut text);
                token_vec.push(token);
	        } else if first.is_ascii_punctuation() {
	        	token = symbol_match(&mut text);
                token_vec.push(token);
	        } else {
	        	panic!("Lexer: Invalid char: {}", first);
	        }

            text = text.trim_start().to_string();
	    }
        // line_count += 1;
	}
	token_vec
}

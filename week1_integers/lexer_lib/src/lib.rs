use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use lib::Token;

// Int match part
    // <dec int> ::= <number>{<number>}
    // <number> ::= 0|1|2|3|4|5|6|7|8|9
    // Regex for constant integers
fn int_match(text: &mut String) -> Token::Token {
    let int_re = Regex::new(r"(^[0-9][0-9]*)\b").unwrap();
    let int_cap = int_re.captures(&text);
    let int;
    let mut token = Token::new();
    let mut int_str: String;
    match int_cap {
        Some(int_cap) => {
            int = int_cap.get(1).unwrap();
            // let int_start = int.start();
            let int_end = int.end();
            int_str = int.as_str().to_string();
            token._type = "CONSTANT".to_string();
            token._value = int_str.clone();
            // println!("Int Match: {}", int_str);
            text.replace_range(..int_end, "");
            // println!("Int Start: {}", int_start);
            // println!("Int   End: {}", int_end);
        },
        None => panic!("Lexer: int None"),
    };
    token
}


// Word match part
    // <word> :: = <alpha_underscore>{<alpha>|"_"|<number>}
    // <alpha_underscore> ::= <alpha>|"_"
    // <alpha> ::= a-zA-Z{a-zA-Z}
    // Regex for keyword, variable name + symbols 
fn word_match(text: &mut String) -> Token::Token {
	let word_re = Regex::new(r"([a-zA-Z_][a-zA-Z_0-9]*)\b").unwrap();

    let word_cap = word_re.captures(&text);
    let word;
    let mut token = Token::new();
    let mut word_str: String;
    match word_cap {
        Some(word_cap) => {
            word = word_cap.get(1).unwrap();
            // let word_start = word.start();
            let word_end = word.end();
            token._value = word.as_str().to_string();
            word_str = keyword_match(word.as_str()).to_string();
            token._type = word_str.clone();
            // println!("WordMatch: {}", word_str);
  			text.replace_range(..word_end, "");
            // end = word_end;
            // println!("Word Start: {}", word_start);
            // println!("Word   End: {}", word_end);
        },
        None => panic!("Lexer: word None"),
    };
    token
}


// Symbol match part
    // <symbol> ::= "("|")"|"{"|"}"|";"
fn symbol_match(text: &mut String) -> Token::Token {
    let syb_re = Regex::new(r"(^[\(\)\{\};])").unwrap();

    let syb_cap = syb_re.captures(&text);
    let syb;
    let mut token = Token::new();
    let mut syb_str: String;
    match syb_cap {
    	Some(syb_cap) => {
    		syb = syb_cap.get(1).unwrap();
    		// let syb_start = syb.start();
    		let syb_end = syb.end();
            token._value = syb.as_str().to_string();
    		syb_str = keysyb_match(syb.as_str()).to_string();
            token._type = syb_str.clone();
    		// println!("Syb Match: {}", syb_str);
    		text.replace_range(..syb_end, "");
    		// println!("Syb Start: {}", syb_start);
    		// println!("Syb   End: {}", syb_end);
    	},
    	None => panic!("Lexer: Syb None"),
    };
    token
}


fn keyword_match(word: &str) -> &str {
	match word {
		"int" => "INT_KEYWORD",
		"return" => "RETURN_KEYWORD",

		_ => "IDENTIFIER",
	}
}


fn keysyb_match(word: &str) -> &str {
    match word {
        "(" => "OPEN_PAREN",
        ")" => "CLOSE_PAREN",
        "{" => "OPEN_BRACE",
        "}" => "CLOSE_BRACE",
        ";" => "SEMICOLON",
        _ => panic!("Lexer: Unrecognize symbol"),
    }
}


pub fn lex(path: &str) -> Vec<Token::Token> {
    // println!("Lexer:");
    // println!("File path: {}", &path);
	
    let file = File::open(&path).expect("\nLexer: Failed to open the file\n");
	let reader = BufReader::new(file);
	
    // println!("----------------");
	// println!("[-] File: {}\n", &path);
    
    let mut token_vec = Vec::new();
    for line in reader.lines(){
		let mut text = line.unwrap().trim().to_string();
    	// println!("New Line: \n\"{}\"\n", text);

	    let mut first: char;
        let mut token: Token::Token;
	    while text.len()!=0 {

	        first = text.chars().nth(0).unwrap();
	        if first.is_ascii_digit() {
	        	token = int_match(&mut text);
                // println!("Int token: {}, {}", &token._type, &token._value);
                token_vec.push(token);
	        } else if first.is_ascii_alphabetic() {
	        	token = word_match(&mut text);
                // println!("Word token: {}, {}", &token._type, &token._value);
                token_vec.push(token);
	        } else if first.is_ascii_punctuation() {
	        	token = symbol_match(&mut text);
                // println!("Syb token: {}, {}", &token._type, &token._value);
                token_vec.push(token);
	        } else {
	        	panic!("Lexer: Invalid char: {}", first);
	        }
            text = text.trim_start().to_string();
	    }
	}
	token_vec
}

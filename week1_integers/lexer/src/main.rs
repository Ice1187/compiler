use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Int match part
fn int_match(text: &mut String) -> String {
    // <dec int> ::= <number>{<number>}
    // <number> ::= 0|1|2|3|4|5|6|7|8|9
    // Regex for constant integers
    let int_re = Regex::new(r"(^[0-9][0-9]*)\b").unwrap();

    let int_cap = int_re.captures(&text);
    let int;
    let mut int_str = String::new();
    match int_cap {
        Some(int_cap) => {
            int = int_cap.get(1).unwrap();
            // let int_start = int.start();
            let int_end = int.end();
            int_str = int.as_str().to_string();
            // println!("Int Match: {}", int_str);
            text.replace_range(..int_end, "");
            // println!("Int Start: {}", int_start);
            // println!("Int   End: {}", int_end);
        },
        None => println!("int None"),
    };
    int_str
}


// Word match part
fn word_match(text: &mut String) -> String {
    // <word> :: = <alpha_underscore>{<alpha>|"_"|<number>}
    // <alpha_underscore> ::= <alpha>|"_"
    // <alpha> ::= a-zA-Z{a-zA-Z}
    // Regex for keyword, variable name + symbols 
	let word_re = Regex::new(r"([a-zA-Z_][a-zA-Z_0-9]*)\b").unwrap();

    let word_cap = word_re.captures(&text);
    let word;
    let mut word_str = String::new();
    match word_cap {
        Some(word_cap) => {
            word = word_cap.get(1).unwrap();
            // let word_start = word.start();
            let word_end = word.end();
            word_str = keyword_match(word.as_str()).to_string();
            // println!("WordMatch: {}", word_str);
  			text.replace_range(..word_end, "");
            // end = word_end;
            // println!("Word Start: {}", word_start);
            // println!("Word   End: {}", word_end);
        },
        None => println!("word None"),
    };
    word_str
}


// Symbol match part
fn symbol_match(text: &mut String) -> String {
    // <symbol> ::= "("|")"|"{"|"}"|";"
    let syb_re = Regex::new(r"(^[\(\)\{\};])").unwrap();

    let syb_cap = syb_re.captures(&text);
    let syb;
    let mut syb_str = String::new();
    match syb_cap {
    	Some(syb_cap) => {
    		syb = syb_cap.get(1).unwrap();
    		// let syb_start = syb.start();
    		let syb_end = syb.end();
    		syb_str = keysyb_match(syb.as_str()).to_string();
    		// println!("Syb Match: {}", syb_str);
    		text.replace_range(..syb_end, "");
    		// println!("Syb Start: {}", syb_start);
    		// println!("Syb   End: {}", syb_end);
    	},
    	None => println!("Syb None"),
    };
    syb_str
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
        _ => panic!("Unrecognize symbol"),
    }
}




fn main() {
	let filename_vec = vec!["missing_paren.c", "missing_retval.c", "no_brace.c", "no_semicolon.c", "no_space.c", "wrong_case.c"];
	// let filename_vec = vec!["multi_digit.c", "newlines.c", "no_newlines.c", "return_0.c", "return_2.c", "spaces.c"];
	for filename in filename_vec {
		let filename = "../../test/stage_1/invalid/".to_owned() + filename;
		// println!("{:?}", filename);
		let file = File::open(&filename).expect("Failed to open the file");
		let reader = BufReader::new(file);

		println!("----------------");
		println!("File: {}", &filename);

        let mut token_vec = Vec::new();
	    for line in reader.lines(){
			let mut text = line.unwrap().trim().to_string();
	    	println!("\nNew Line: \n\"{}\"\n", text);

		    let mut first: char;
            let mut token;
		    while text.len()!=0 {
		        // println!("{:?}", &text);

		        first = text.chars().nth(0).unwrap();
		        if first.is_ascii_digit() {
		        	token = int_match(&mut text);
                    println!("Int token: {}", &token);
                    token_vec.push(token);
		        } else if first.is_ascii_alphabetic() {
		        	token = word_match(&mut text);
                    println!("Word token: {}", token);
                    token_vec.push(token);
		        } else if first.is_ascii_punctuation() {
		        	token = symbol_match(&mut text);
                    println!("Syb token: {}", token);
                    token_vec.push(token);
		        } else {
		        	panic!("Invalid char: {}", first);
		        }
                text = text.trim_start().to_string();
		    }
	    }
        println!("\n");
        println!("Token vec: {:?}", token_vec);
		println!("--------\n");
	}
}

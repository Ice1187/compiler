use regex::Regex;

// Word match part
fn word_match(mut text: &str) -> &str {
    // <word> :: = <alpha_underscore>{<alpha>|"_"|<number>}
    // <alpha_underscore> ::= <alpha>|"_"
    // <alpha> ::= a-zA-Z{a-zA-Z}
    // Regex for keyword, variable name + symbols 
    let word_re = Regex::new(r"([a-zA-Z_][a-zA-Z_0-9]*)\b").unwrap();

    let word_cap = word_re.captures(&text);
    let word;
    match word_cap {
        Some(word_cap) => {
            word = word_cap.get(1).unwrap();
            let word_str = word.as_str();
            let word_start = word.start();
            let word_end = word.end();
  			text = &text[word_end..].trim_start();
            // end = word_end;
            println!("WordMatch: {}", word_str);
            println!("Word Start: {}", word_start);
            println!("Word   End: {}", word_end);
        },
        None => println!("word None"),
    };
    text
}

// Int match part
fn int_match(mut text: &str) -> &str {
    // <dec int> ::= <number>{<number>}
    // <number> ::= 0|1|2|3|4|5|6|7|8|9
    // Regex for constant integers
    let int_re = Regex::new(r"(^[0-9][0-9]*)\b").unwrap();

    let int_cap = int_re.captures(&text);
    let int;
    match int_cap {
        Some(int_cap) => {
            int = int_cap.get(1).unwrap();
            let int_str = int.as_str();
            let int_start = int.start();
            let int_end = int.end();
            text = &text[int_end..].trim_start();
            println!("Int Match: {}", int_str);
            println!("Int Start: {}", int_start);
            println!("Int   End: {}", int_end);
        },
        None => println!("int None"),
    };
    text

}

fn main() {

    let mut text = "int 2 main(){return 2;}";
    text = text.trim();



    // <symbol> ::= "("|")"|"{"|"}"|";"

    let symbol_re = Regex::new(r"[\(\)\{\}]");
    
    let mut c = 5;
    // let mut int_ord;
    // let mut word_ord;
    while text.len()!=0 && c>=0{

        println!("{:?}", &text);

        // start = &text[..1];
        if text.get(..1) == Some("2") { text = int_match(&text); }	
        else if text.get(..1) == Some("i") { text = word_match(&text); }	

        c -= 1;
    }
}

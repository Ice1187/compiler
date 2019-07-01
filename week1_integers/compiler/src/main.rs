use gen_lib;
use std::env;
use std::io::{stdin, stdout, Write};
use std::process::Command;
use std::os::unix::process::CommandExt;

fn get_filename() -> (String, String) {
	let mut path = String::new();
	let mut filename: String;
	let mut args = env::args_os();
	// println!("args len: {:?}", args.len());
	if args.len() > 1 { 
		path = args.nth(1).expect("Compiler: Invalid arg[1]").into_string().expect("Compiler: Invalid arg[1]").clone();
		// println!("arg[1]: {}\nUse arg[1] as file.", path)
	} else {	
		print!("File name: ");
		stdout().flush().unwrap();
		stdin().read_line(&mut path).expect("Compiler: Invalid file name.");
	}

	path = path.trim().to_string();
	filename = path.replace(".c", ".s");
	(path, filename)
}


fn gcc(filename: & String) {
	let out = filename.replace(".s", "");
	let s = Command::new("gcc")
    .uid(0)
    .arg("-masm=intel")
    .arg(&filename)
    .arg("-o")
    .arg(&out)
    .status().unwrap();
    // println!("{}", s);
}


fn main() {
    let (path, filename) = get_filename();
    gen_lib::gen(&path, &filename);
    gcc(&filename);
}

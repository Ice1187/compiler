use std::fs::File;
use std::io::Write;

pub fn mov(a: &str, b: &str, f: &mut File) {
	write!(f, "  ").expect("asm: Unable to write to the file.");
	write!(f, "mov         {}, {}\n", a, b).expect("asm: Unable to write to the file.");
}
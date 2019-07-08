use std::fs::File;
use std::io::Write;

pub fn mov(des: &str, src: &str, f: &mut File) {
	write!(f, "  ").expect("asm mov: Unable to write to the file.");
	write!(f, "mov         {}, {}\n", des, src).expect("asm: Unable to write to the file.");
}

pub fn ret(f: &mut File) {
	write!(f, "  ").expect("asm ret: stat_return: Unable to write to the file.");
	write!(f, "ret\n").expect("asm ret: stat_return: Unable to write to the file.");
}

pub fn neg(des: &str, f: &mut File) {	
	write!(f, "  ").expect("asm neg: Unable to write to the file.");
	write!(f, "neg         {}\n", des).expect("asm neg: Unable to write to the file.");
}

pub fn not(des: &str, f: &mut File) {	
	write!(f, "  ").expect("asm neg: Unable to write to the file.");
	write!(f, "not         {}\n", des).expect("asm neg: Unable to write to the file.");
}

pub fn cmp(des: &str, src: &str, f: &mut File) {
	write!(f, "  ").expect("asm neg: Unable to write to the file.");
	write!(f, "cmp         {}, {}\n", des, src).expect("asm neg: Unable to write to the file.");	
}

pub fn sete(des: &str, f: &mut File) {
	write!(f, "  ").expect("asm neg: Unable to write to the file.");
	write!(f, "sete        {}\n", des).expect("asm neg: Unable to write to the file.");	
}

pub fn push(des: &str, f: &mut File) {
	write!(f, "  ").expect("asm push: Unable to write to the file.");
	write!(f, "push        {}\n", des).expect("asm push: Unable to write to the file.");
}

pub fn pop(des: &str, f: &mut File) {
	write!(f, "  ").expect("asm pop: Unable to write to the file.");
	write!(f, "pop         {}\n", des).expect("asm pop: Unable to write to the file.");
}

pub fn add(des: &str, src: &str, f: &mut File) {
	write!(f, "  ").expect("asm add: Unable to write to the file.");
	write!(f, "add         {}, {}\n", des, src).expect("asm add: Unable to write to the file.");
}

pub fn sub(des: &str, src: &str, f: &mut File) {
	write!(f, "  ").expect("asm sub: Unable to write to the file.");
	write!(f, "sub         {}, {}\n", des, src).expect("asm sub: Unable to write to the file.");
}

pub fn imul(des: &str, src: &str, f: &mut File) {
	write!(f, "  ").expect("asm imul: Unable to write to the file.");
	write!(f, "imul        {}, {}\n", des, src).expect("asm imul: Unable to write to the file.");
}

pub fn idiv(des: &str, f: &mut File) {
	write!(f, "  ").expect("asm idiv: Unable to write to the file.");
	write!(f, "idiv        {}\n", des).expect("asm idiv: Unable to write to the file.");
}

pub fn xor(des: &str, src: &str, f: &mut File) {
	write!(f, "  ").expect("asm xor: Unable to write to the file.");
	write!(f, "xor         {}, {}\n", des, src).expect("asm xor: Unable to write to the file.");
}



use std::fs::File;
use std::io::Write;

pub fn mov(des: &str, src: &str, f: &mut File) {
	write!(f, "  ").expect("asm mov: Unable to write to the file.");
	write!(f, "mov         {}, {}\n", des, src).expect("asm mov: Unable to write to the file.");
}

pub fn ret(f: &mut File) {
	write!(f, "  ").expect("asm ret: Unable to write to the file.");
	write!(f, "ret\n").expect("asm ret: Unable to write to the file.");
}

pub fn neg(des: &str, f: &mut File) {	
	write!(f, "  ").expect("asm neg: Unable to write to the file.");
	write!(f, "neg         {}\n", des).expect("asm neg: Unable to write to the file.");
}

pub fn not(des: &str, f: &mut File) {	
	write!(f, "  ").expect("asm not: Unable to write to the file.");
	write!(f, "not         {}\n", des).expect("asm not: Unable to write to the file.");
}

pub fn cmp(des: &str, src: &str, f: &mut File) {
	write!(f, "  ").expect("asm cmp: Unable to write to the file.");
	write!(f, "cmp         {}, {}\n", des, src).expect("asm cmp: Unable to write to the file.");	
}

// set if equal
pub fn sete(des: &str, f: &mut File) {
	write!(f, "  ").expect("asm sete: Unable to write to the file.");
	write!(f, "sete        {}\n", des).expect("asm sete: Unable to write to the file.");	
}

// set if not equal
pub fn setne(des: &str, f: &mut File) {
	write!(f, "  ").expect("asm setne: Unable to write to the file.");
	write!(f, "setne       {}\n", des).expect("asm setne: Unable to write to the file.");	
}

// set if greater
pub fn setg(des: &str, f: &mut File) {
	write!(f, "  ").expect("asm setg: Unable to write to the file.");
	write!(f, "setg        {}\n", des).expect("asm setg: Unable to write to the file.");	
}

// set if greater or equal
pub fn setge(des: &str, f: &mut File) {
	write!(f, "  ").expect("asm setge: Unable to write to the file.");
	write!(f, "setge       {}\n", des).expect("asm setge: Unable to write to the file.");	
}

// set if less
pub fn setl(des: &str, f: &mut File) {
	write!(f, "  ").expect("asm setl: Unable to write to the file.");
	write!(f, "setl        {}\n", des).expect("asm setl: Unable to write to the file.");	
}

// set if less or equal
pub fn setle(des: &str, f: &mut File) {
	write!(f, "  ").expect("asm setle: Unable to write to the file.");
	write!(f, "setle       {}\n", des).expect("asm setle: Unable to write to the file.");	
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

pub fn jmp(des: &str, f: &mut File) {
	write!(f, "  ").expect("asm jmp: Unable to write to the file.");
	write!(f, "jmp          {}\n", des).expect("asm jmp: Unable to write to the file.");
}

pub fn je(des: &str, f: &mut File) {
	write!(f, "  ").expect("asm je: Unable to write to the file.");
	write!(f, "je          {}\n", des).expect("asm je: Unable to write to the file.");
}

pub fn jne(des: &str, f: &mut File) {
	write!(f, "  ").expect("asm jne: Unable to write to the file.");
	write!(f, "jne          {}\n", des).expect("asm jne: Unable to write to the file.");
}

pub fn make_func(des: &str, f: &mut File) {
	write!(f, "{}:\n", des).expect("asm make_func: Unable to write to the file.");	
}
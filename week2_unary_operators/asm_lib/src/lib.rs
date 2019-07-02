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
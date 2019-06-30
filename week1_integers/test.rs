use std::process::Command;
use std::os::unix::process::CommandExt;

fn main() {
    //let s = Command::new("./return_2").uid(1000).output().unwrap().status;
    // let s = Command::new("./return_2").uid(1000).status().unwrap();
    let s = Command::new("gcc")
    .uid(0)
    .arg("-masm=intel")
    .arg("./return_2_in.s")
    .arg("-o")
    .arg("return_2")
    .status().unwrap();
    println!("{}", s);
}

fn main() {
    let mut text = "abc";
    text = fun(&mut text);
    println!("{}", &text);
}

fn fun(mut text: &str) -> &str {
    text = &text[..2];
    println!("Fun: {}", &text);
    text
}

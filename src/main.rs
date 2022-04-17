
fn main() {
    let s = String::from("hello");
    println!("{s}");
    let t = s.to_string();
    let t = " and good world!".to_string();
    println!("{t}");

    let data = "initial contents good";

    let z = data.to_string();

    // the method also works on a literal directly:
    let z = "initial contents".to_string();
    println!("{z}");
    let a = String::from("Sting initial contents");
    let x = a.to_string();

    // the method also works on a literal directly:
    let x = "initial contents".to_string();
    println!("{x}");

    let hello1 = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
    println!("{hello1}");

    let len = String::from("Hola").len();
    println!("{len}");
    let len2 = String::from("Здравствуйте").len();
    println!("{len2}");



}

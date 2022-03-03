// this code not working
// When you're writing a program, if you don't know the exhaustive set of types the program will get at runtime to store in a vector, the enum technique won't work. Instead, you can use a trait object, which we'll cover in Chapter 17. 


enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        SpreadsheetCell:Int(3),
        SpreadsheetCell:Text(String::from("blue")),
        SpreadsheetCell:Float(10.12233),
    ];

    println!("row vec! print : {row}")
} 
  

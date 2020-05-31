extern crate frases;

fn main() {
	println!("Hello in Spanish: {}", frases::spanish::saludo::hello());
	println!("Goodbye in Spanish: {}", frases::spanish::despedida::goodbye());
	
    println!("Hello in English: {}", frases::english::saludo::hello());
    println!("Goodbye in English: {}", frases::english::despedida::goodbye());
}

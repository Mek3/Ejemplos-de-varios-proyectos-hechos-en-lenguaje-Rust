#[warn(unused_must_use)]

use std::io::stdin;

/// Funcion para sumar a y b
fn sumar(a: i32, b: i32) -> i32
{
	a + b
}
/// Funcion para restar a y b
fn restar(a: i32, b: i32) -> i32
{
	b - a
} 
/// Funcion para multiplicar a y b
fn multiplicar(a: i32, b: i32) -> i32
{
	b * a
} 
/// Funcion para dividir a y b
pub fn dividir(a: i32, b: i32) -> i32
{
	b / a
} 

/// Menú principal de la calculadora.
fn main() {	
	let  mut en: i32 = 0;
		
	while en != 5 {	
		
	println!("************* Menú calculadora **************");
	println!("1- Sumar. ");
	println!("2- Restar. ");
	println!("3- Multiplicar. ");
	println!("4- Dividir. ");
	println!("5- Salir. ");
	println!("Opción:  ");
	let mut entrada = String::new();
	stdin().read_line(&mut entrada);
	en = entrada.trim().parse().unwrap();
	if en >= 5 {
		println!("Hasta la próxima!\n");		
	} else
	{
		println!("a:  ");
		let mut a_en = String::new();
		stdin().read_line(&mut a_en);
		let a: i32 = a_en.trim().parse().unwrap();
		println!("b:  ");
		let mut b_en = String::new();
		stdin().read_line(&mut b_en);
		let b: i32 = b_en.trim().parse().unwrap();
		if en == 1 {
		     println!("Resultado de la suma: {}\n", sumar(a, b));
		} else if en == 2 {
			 println!("Resultado de la resta: {}\n", restar(a, b));
		} else if en == 3 {
			 println!("Resultado de la multiplicación: {}\n", multiplicar(a, b));
		} else if en == 4 {
			 println!("Resultado de la división: {} \n", dividir(a, b));		
		}
	}  
   }   
}

#[warn(unused_imports)]
extern crate vehiculo;

use vehiculo::motor::gasolina::coche;


fn main() 
{
	let coche_c = coche::Coche{marca: "Seat".to_string(),
			                   modelo: "Ibiza".to_string()};
						  
	coche::imprimir_datos_coche(coche_c);    
}

extern crate modulos_en_carpetas;


fn main() 
{
	let coche = coche::Coche{marca: "Seat".to_string(),
						  modelo: "Ibiza".to_string()};
						  
	vehiculo::coche::imprimir_datos_coche(coche.marca, coche.modelo);    
}

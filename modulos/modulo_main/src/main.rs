#[warn(dead_code)]

mod my {
	
	pub struct Coche {
		pub marca: String,
		pub modelo: String,
	}
	
	pub fn imprimir_datos_coche(marca: String,
								modelo: String)
	{
		println!("Marca: {} \nModelo: {}", marca, modelo);
	}
}

fn main() {
	
	let coche = my::Coche{marca: "Seat".to_string(), modelo: "Ibiza".to_string()};
	my::imprimir_datos_coche(coche.marca, coche.modelo);    
}

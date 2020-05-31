#[warn(dead_code)]

mod my {
	
	pub struct Coche {
		pub marca: String,
		pub modelo: String,
	}
	
	pub fn imprimir_datos_coche(marca: String, modelo: String)
	{
	      println!("Marca: {} \nModelo: {}", marca, modelo);
	}
}


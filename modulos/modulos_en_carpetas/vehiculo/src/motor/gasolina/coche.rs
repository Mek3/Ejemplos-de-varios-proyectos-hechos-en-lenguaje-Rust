
pub struct Coche {
	pub marca: String,
	pub modelo: String,
}
	
pub fn imprimir_datos_coche(coche_c: Coche)
{
	println!("Marca: {} \nModelo: {}", coche_c.marca, coche_c.modelo);
}

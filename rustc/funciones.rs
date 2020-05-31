fn datos_usuario()-> (i32, String, bool){
	(24, "Mohamed".to_string(), true)
}

fn suma(a: i32, b: i32) -> i32
{
    a+b
}
fn imprimir_resultado(result: i32)
{
    println!("resultado suma: {}, {:?}",result, datos_usuario());
}

fn add_elemento(mut lista_elementos: Vec<i32>) -> Vec<i32> {
        lista_elementos.push(10);
        lista_elementos
}

fn main()
{
    let a = 1994;
    let b = 24;
    imprimir_resultado(suma(a,b));
    
    let lista_elementos = vec![];
	let lista_elementos2 = add_elemento(lista_elementos);
	println!("Lista de elementos: {:?}", lista_elementos2);
	
	let cuadrado = |m| m * m;
	let result = cuadrado(4);
	println!("resultado multiplicación: {:?}", result); 
	
	let cuadrado_sup = |c| {
        let y = c+1;
        y*y
    };
    
    let result2 = cuadrado_sup(8);
    println!("resultado multiplicación: {:?}", result2); 
}

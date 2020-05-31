// Estructura con dos elementos
/*struct Punto {
    x: f32,
    y: f32,
}*/

fn main()
{ 
    let tupla = (24, "Mohamed", false);
    let (edad,nombre,hace_ejercicio) = tupla;
    println!("Datos tupla: ({}, {}, {}) ",edad, nombre, hace_ejercicio);

    let (_,nombre,_) = tupla; //Ignoramos el 1º y el 3º valor.
    println!("Nombre: {}", nombre);
    
    /*let p1: Punto = Punto { x: 0.0, y: 0.0 };
    let p2: Punto = Punto { x: 0.0, y: 1.0 };
	let p3: Punto = Punto { x: 2.0, y: 1.0 };

    let triangulo = (p1, p2, p3);
    
    println!("Puntos triangulo: {}", triangulo.1.x, triangulo.1.y,
									 triangulo.2.x, triangulo.2.y,
									 triangulo.3.x,triangulo.3.y);*/
	
}

